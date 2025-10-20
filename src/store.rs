//! Store implementation for managing atom state
//!
//! Reference: `jotai/src/vanilla/store.ts` and `jotai/src/vanilla/internals.ts`
//!
//! The Store is the runtime container that holds all atom values, tracks dependencies,
//! manages subscriptions, and coordinates updates.
//!
//! ## Functional Programming Patterns
//! - Encapsulation: Store hides internal state management
//! - Higher-order functions: subscribe returns unsubscribe function
//! - Monadic patterns: Getter/Setter provide controlled state access

use dashmap::DashMap;
use parking_lot::{Mutex, RwLock};
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::atom::{self, Atom, WritableAtom};
use crate::error::{AtomError, Result};
use crate::internals::{AtomState, Mounted};
use crate::types::{AtomId, EpochNumber, Getter, Listener, Setter, Unsubscribe};

/// The Store manages all atom state and coordinates updates
///
/// Reference: `jotai/src/vanilla/internals.ts` (buildStore function)
///
/// The Store contains several key data structures:
/// - `atom_states`: Maps atom IDs to their current state (value, dependencies, epoch)
/// - `mounted`: Maps atom IDs to subscription info (only for subscribed atoms)
/// - `invalidated`: Set of atoms that need recomputation
/// - `changed`: Set of atoms that changed and need listener notification
///
/// **FP Pattern**: Encapsulation of mutable state with pure interface
pub struct Store {
    /// Map of atom IDs to their current state
    ///
    /// Uses `Box<dyn Any>` for type erasure since we need to store heterogeneous types.
    /// Each value is actually an `AtomState<T>` but type-erased.
    ///
    /// **Rust Pattern**: Type erasure with Any
    ///
    /// TODO: Phase 1.2 - Initialize this map
    /// TODO: Phase 1.3 - Read from this map in get()
    /// TODO: Phase 1.4 - Update this map in set()
    pub(crate) atom_states: DashMap<AtomId, Arc<RwLock<Box<dyn Any + Send + Sync>>>>,

    /// Map of mounted (subscribed) atoms to their subscription info
    ///
    /// Only atoms with active subscriptions are in this map.
    /// This enables automatic garbage collection of unused atoms.
    ///
    /// **FP Pattern**: Lazy mounting pattern
    ///
    /// TODO: Phase 3.1 - Track mounted atoms
    /// TODO: Phase 3.2 - Add/remove on subscribe/unsubscribe
    pub(crate) mounted: DashMap<AtomId, Arc<RwLock<Mounted>>>,

    /// Set of atoms that have been invalidated and need recomputation
    ///
    /// TODO: Phase 2.3 - Mark atoms as invalidated when dependencies change
    /// TODO: Phase 4.1 - Use in topological sort
    pub(crate) invalidated: Arc<RwLock<HashSet<AtomId>>>,

    /// Set of atoms that changed (for listener notification)
    ///
    /// TODO: Phase 3.3 - Collect changed atoms during updates
    pub(crate) changed: Arc<RwLock<HashSet<AtomId>>>,

    /// Pending mount callbacks
    ///
    /// TODO: Phase 8.1 - Execute after flush
    pub(crate) mount_callbacks: Arc<Mutex<Vec<Box<dyn FnOnce() + Send>>>>,

    /// Pending unmount callbacks
    ///
    /// TODO: Phase 8.1 - Execute after flush
    pub(crate) unmount_callbacks: Arc<Mutex<Vec<Box<dyn FnOnce() + Send>>>>,
}

impl Store {
    /// Create a new Store
    ///
    /// Reference: `jotai/src/vanilla/store.ts:9-20`
    ///
    /// ```typescript
    /// export function createStore(): Store {
    ///   const atomStateMap: WeakMap<AnyAtom, AtomState> = new WeakMap()
    ///   const mountedMap: WeakMap<AnyAtom, Mounted> = new WeakMap()
    ///   // ... other initialization
    ///   return { get: storeGet, set: storeSet, sub: storeSub }
    /// }
    /// ```
    ///
    /// TODO: Phase 1.2 - Initialize all data structures
    pub fn new() -> Self {
        Store {
            atom_states: DashMap::new(),
            mounted: DashMap::new(),
            invalidated: Arc::new(RwLock::new(HashSet::new())),
            changed: Arc::new(RwLock::new(HashSet::new())),
            mount_callbacks: Arc::new(Mutex::new(Vec::new())),
            unmount_callbacks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Read an atom's current value
    ///
    /// Reference: `jotai/src/vanilla/internals.ts` (storeGet function ~line 900)
    ///
    /// ```typescript
    /// const storeGet = <Value>(atom: Atom<Value>): Value => {
    ///   const atomState = readAtomState(atom)
    ///   return atomState.v
    /// }
    /// ```
    ///
    /// This function:
    /// 1. Looks up or initializes the atom's state
    /// 2. If value is cached and fresh, returns it
    /// 3. Otherwise, calls the atom's read function
    /// 4. Tracks dependencies during read
    /// 5. Caches the result with current epoch
    ///
    /// **FP Pattern**: Lazy evaluation, memoization
    ///
    /// TODO: Phase 1.3 - Basic implementation for primitive atoms
    /// TODO: Phase 2.1 - Add dependency tracking
    /// TODO: Phase 2.4 - Add epoch-based cache checking
    /// TODO: Phase 6.1 - Handle promises/async
    pub fn get<T: Clone + Send + Sync + 'static>(&self, atom: &Atom<T>) -> Result<T> {
        // TODO: Phase 1.3 - Implement basic get
        // Steps:
        // 1. Check if atom_states has this atom
        // 2. If not, initialize with default/uncomputed state
        // 3. Check if value is cached
        // 4. If not, call atom.read() with a Getter implementation
        // 5. Store the result in atom_states
        // 6. Return the value
        if let Some(state_arc) = self.atom_states.get(&atom.id) {
            let lock = state_arc.read();
            if let Some(atom_state) = lock.downcast_ref::<AtomState<T>>() {
                if let Some(ref result) = atom_state.value {
                    return result.clone();
                }
            }
        }

        let v = atom.read()?;
        self.atom_states.insert(
            atom.id,
            Arc::new(RwLock::new(Box::new(AtomState {
                epoch: 1,
                value: Some(Ok(v.clone())),
                dependencies: HashMap::new(),
                pending_promises: HashSet::new(),
            }))),
        );
        Ok(v)
    }

    /// Update an atom's value
    ///
    /// Reference: `jotai/src/vanilla/internals.ts` (storeSet function ~line 950)
    ///
    /// ```typescript
    /// const storeSet = <Value, Args, Result>(
    ///   atom: WritableAtom<Value, Args, Result>,
    ///   ...args: Args
    /// ): Result => {
    ///   return writeAtomState(atom, ...args)
    /// }
    /// ```
    ///
    /// This function:
    /// 1. Calls the atom's write function
    /// 2. Updates the value in atom_states
    /// 3. Increments the epoch number
    /// 4. Marks all dependent atoms as invalidated
    /// 5. Recomputes invalidated atoms
    /// 6. Notifies listeners of changed atoms
    ///
    /// **FP Pattern**: State transformation, cascading updates
    ///
    /// TODO: Phase 1.4 - Basic implementation for primitive atoms
    /// TODO: Phase 2.3 - Add invalidation of dependents
    /// TODO: Phase 4.2 - Add recomputation loop
    /// TODO: Phase 3.3 - Add listener notification
    pub fn set<T: Clone + Send + Sync + 'static>(
        &self,
        atom: &WritableAtom<T>,
        value: T,
    ) -> Result<()> {
        // Phase 1.4 - Basic set implementation for primitive atoms
        // For primitive atoms, we directly update the state without calling write_fn
        // (write_fn is for derived/writable atoms in later phases)

        // 1. Initialize state if it doesn't exist
        if !self.atom_states.contains_key(&atom.id()) {
            let initial_state: AtomState<T> = AtomState {
                epoch: 0,
                value: None,
                dependencies: HashMap::new(),
                pending_promises: HashSet::new(),
            };
            self.atom_states
                .insert(atom.id(), Arc::new(RwLock::new(Box::new(initial_state))));
        }

        // 2. Update the value and increment epoch
        if let Some(state_arc) = self.atom_states.get(&atom.id()) {
            let mut lock = state_arc.write();
            if let Some(state) = lock.downcast_mut::<AtomState<T>>() {
                state.value = Some(Ok(value));
                state.epoch += 1;
            }
        }

        // 3. Mark atom as changed (for listener notification in Phase 3)
        self.changed.write().insert(atom.id());

        // TODO: Phase 2.3 - Invalidate dependents
        // TODO: Phase 3.3 - Flush callbacks

        Ok(())
    }

    /// Subscribe to atom changes
    ///
    /// Reference: `jotai/src/vanilla/internals.ts` (storeSub function ~line 1000)
    ///
    /// ```typescript
    /// const storeSub = (atom: AnyAtom, listener: () => void) => {
    ///   mountAtom(atom, listener)
    ///   flushCallbacks()
    ///   const unsubscribe = () => {
    ///     unmountAtom(atom, listener)
    ///     flushCallbacks()
    ///   }
    ///   return unsubscribe
    /// }
    /// ```
    ///
    /// This function:
    /// 1. Mounts the atom (creates Mounted entry)
    /// 2. Recursively mounts dependencies
    /// 3. Adds the listener to the Mounted entry
    /// 4. Calls atom's onMount callback if present
    /// 5. Returns an unsubscribe function
    ///
    /// **FP Pattern**: Higher-order function returns cleanup function
    ///
    /// TODO: Phase 3.2 - Implement subscription system
    /// TODO: Phase 3.4 - Implement recursive mounting
    /// TODO: Phase 8.1 - Call onMount lifecycle
    pub fn sub<F>(
        &self,
        atom: &Atom<impl Clone + Send + Sync + 'static>,
        listener: F,
    ) -> Unsubscribe
    where
        F: Fn() + Send + Sync + 'static,
    {
        // TODO: Phase 3.2 - Implement subscription
        // Steps:
        // 1. Mount the atom
        // 2. Add listener to mounted entry
        // 3. Flush any pending callbacks
        // 4. Return unsubscribe function that:
        //    - Removes listener
        //    - Unmounts if no more listeners
        //    - Calls cleanup if present

        todo!("Store::sub - Phase 3.2")
    }

    /// Ensure an atom has state initialized
    ///
    /// Reference: `jotai/src/vanilla/internals.ts` (ensureAtomState function)
    ///
    /// TODO: Phase 1.3 - Implement state initialization
    pub(crate) fn ensure_atom_state<T: Clone + Send + Sync + 'static>(
        &self,
        atom: &Atom<T>,
    ) -> Result<()> {
        // TODO: Create AtomState if it doesn't exist
        // Call unstable_onInit if present
        let atom_state = AtomState {
            epoch: 1,
            value: Some(atom.read()),
            dependencies: HashMap::new(),
            pending_promises: HashSet::new(),
        };

        Ok(())
    }

    /// Read atom state, computing if necessary
    ///
    /// Reference: `jotai/src/vanilla/internals.ts` (readAtomState function)
    ///
    /// This is the core function that:
    /// - Checks cache validity
    /// - Calls read function if needed
    /// - Tracks dependencies
    ///
    /// TODO: Phase 1.3 - Implement
    pub(crate) fn read_atom_state<T: Clone + Send + Sync + 'static>(
        &self,
        atom: &Atom<T>,
    ) -> Result<T> {
        self.get(atom)
    }

    /// Write atom state
    ///
    /// Reference: `jotai/src/vanilla/internals.ts` (writeAtomState function)
    ///
    /// TODO: Phase 1.4 - Implement
    pub(crate) fn write_atom_state<T: Clone + Send + Sync + 'static>(
        &self,
        atom: &WritableAtom<T>,
        value: T,
    ) -> Result<()> {
        atom.write(value.clone())?;
        // TODO: Call atom.write() with getter/setter
        // TODO: Update state
        // TODO: Increment epoch
        if let Some(state_arc) = self.atom_states.get(&atom.id()) {
            let mut lock = state_arc.write();
            if let Some(state) = lock.downcast_mut::<AtomState<T>>() {
                state.epoch += 1;
                let mut r = self.changed.write();
                r.insert(atom.id());
                state.value = Some(Ok(value));
                // self.invalidate_dependents(atom.id());
                // self.flush_callbacks();
            }
        }

        Ok(())
    }

    /// Invalidate all atoms that depend on the given atom
    ///
    /// Reference: `jotai/src/vanilla/internals.ts` (invalidateDependents function)
    ///
    /// Uses breadth-first search to mark all transitive dependents as invalidated.
    ///
    /// TODO: Phase 2.3 - Implement
    pub(crate) fn invalidate_dependents(&self, atom_id: AtomId) {
        // TODO: BFS through dependents
        // TODO: Mark all as invalidated
        todo!("invalidate_dependents - Phase 2.3")
    }

    /// Recompute all invalidated atoms in topological order
    ///
    /// Reference: `jotai/src/vanilla/internals.ts` (recomputeInvalidatedAtoms function)
    ///
    /// Uses DFS-based topological sort to determine recomputation order.
    ///
    /// TODO: Phase 4.1 - Implement topological sort
    /// TODO: Phase 4.2 - Implement recomputation loop
    pub(crate) fn recompute_invalidated(&self) -> Result<()> {
        // TODO: Topological sort of invalidated atoms
        // TODO: Recompute in dependency order
        // TODO: Track which actually changed
        todo!("recompute_invalidated - Phase 4")
    }

    /// Flush pending callbacks (mount, unmount, listeners)
    ///
    /// Reference: `jotai/src/vanilla/internals.ts` (flushCallbacks function)
    ///
    /// Loops until no more changes occur.
    ///
    /// TODO: Phase 3.3 - Implement callback flushing
    pub(crate) fn flush_callbacks(&self) {
        // TODO: Loop until stable
        // TODO: Call all listeners for changed atoms
        // TODO: Execute mount/unmount callbacks
        todo!("flush_callbacks - Phase 3.3")
    }

    /// Mount an atom (add to mounted map)
    ///
    /// Reference: `jotai/src/vanilla/internals.ts` (mountAtom function)
    ///
    /// TODO: Phase 3.2 - Implement mounting
    pub(crate) fn mount_atom<T: Clone + Send + Sync + 'static>(
        &self,
        atom: &Atom<T>,
        listener: Listener,
    ) -> Result<()> {
        // TODO: Create Mounted entry if needed
        // TODO: Add listener
        // TODO: Mount dependencies recursively
        // TODO: Call onMount callback
        todo!("mount_atom - Phase 3.2")
    }

    /// Unmount an atom (remove from mounted map)
    ///
    /// Reference: `jotai/src/vanilla/internals.ts` (unmountAtom function)
    ///
    /// TODO: Phase 3.2 - Implement unmounting
    pub(crate) fn unmount_atom<T: Clone + Send + Sync + 'static>(
        &self,
        atom: &Atom<T>,
        listener: &Listener,
    ) -> Result<()> {
        // TODO: Remove listener
        // TODO: If no more listeners, remove Mounted entry
        // TODO: Call cleanup callback
        // TODO: Unmount dependencies if not used elsewhere
        todo!("unmount_atom - Phase 3.2")
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

// Implement Getter trait for Store
impl Getter for Store {
    fn get<T: Clone + Send + Sync + 'static>(&self, atom: &Atom<T>) -> Result<T> {
        self.get(atom)
    }
}

// Implement Setter trait for Store
impl Setter for Store {
    fn set<T: Clone + Send + Sync + 'static>(&self, atom: &Atom<T>, value: T) -> Result<()> {
        // TODO: This needs to handle WritableAtom conversion
        if let Some(state_arc) = self.atom_states.get(&atom.id()) {
            let mut lock = state_arc.write();
            if let Some(state) = lock.downcast_mut::<AtomState<T>>() {
                state.value = Some(Ok(value));
                state.epoch += 1;
                self.changed.write().insert(atom.id());
            }
        }
        Ok(())
    }
}

impl std::fmt::Debug for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Store")
            .field("atom_states_count", &self.atom_states.len())
            .field("mounted_count", &self.mounted.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_creation() {
        // Test that Store::new initializes all maps correctly
        let store = Store::new();
        assert_eq!(store.atom_states.len(), 0);
        assert_eq!(store.mounted.len(), 0);
    }

    // ============================================================================
    // PHASE 1.3: Store::get() Tests
    // ============================================================================

    #[test]
    fn test_get_primitive_atom() {
        use crate::atom::atom;

        let store = Store::new();
        let count = atom(42);

        // First read should compute and cache the value
        let value = store.get(&count.as_atom()).expect("Should read atom");
        assert_eq!(value, 42);
    }

    #[test]
    fn test_get_caches_value() {
        use crate::atom::atom;

        let store = Store::new();
        let count = atom(100);

        // First read
        let v1 = store.get(&count.as_atom()).unwrap();

        // Second read should return cached value
        let v2 = store.get(&count.as_atom()).unwrap();

        assert_eq!(v1, v2);
        assert_eq!(v1, 100);

        // Verify the atom is now in atom_states
        assert_eq!(store.atom_states.len(), 1);
    }

    #[test]
    fn test_get_multiple_atoms() {
        use crate::atom::atom;

        let store = Store::new();
        let a = atom(1);
        let b = atom(2);
        let c = atom(3);

        assert_eq!(store.get(&a.as_atom()).unwrap(), 1);
        assert_eq!(store.get(&b.as_atom()).unwrap(), 2);
        assert_eq!(store.get(&c.as_atom()).unwrap(), 3);

        // All three atoms should be cached
        assert_eq!(store.atom_states.len(), 3);
    }

    #[test]
    fn test_get_different_types() {
        use crate::atom::atom;

        let store = Store::new();
        let num = atom(42);
        let text = atom("hello".to_string());
        let flag = atom(true);

        assert_eq!(store.get(&num.as_atom()).unwrap(), 42);
        assert_eq!(store.get(&text.as_atom()).unwrap(), "hello");
        assert_eq!(store.get(&flag.as_atom()).unwrap(), true);
    }

    #[test]
    fn test_get_with_label() {
        use crate::atom::atom;

        let store = Store::new();
        let count = atom(5).with_label("counter");

        let value = store.get(&count.as_atom()).unwrap();
        assert_eq!(value, 5);
        assert_eq!(count.as_atom().debug_label(), Some("counter"));
    }

    // TODO: Phase 1.4 - Add tests for set operation
    // TODO: Phase 3.2 - Add tests for subscribe operation
    // TODO: Phase 2.3 - Add tests for invalidation
    // TODO: Phase 4.2 - Add tests for recomputation
}
