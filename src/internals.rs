//! Internal state management structures
//!
//! Reference: `jotai/src/vanilla/internals.ts`
//!
//! This module contains the internal data structures used by the Store to
//! manage atom state, dependencies, and subscriptions. These are not part
//! of the public API.
//!
//! ## Functional Programming Patterns
//! - Immutability where possible
//! - Epoch-based versioning instead of mutation
//! - Separation of data and behavior

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use parking_lot::RwLock;

use crate::types::{AtomId, EpochNumber, Listener, OnUnmount};
use crate::error::{AtomError, Result};

/// State for a single atom
///
/// Reference: `jotai/src/vanilla/internals.ts` (AtomState type ~line 50)
///
/// ```typescript
/// type AtomState<Value = unknown> = {
///   d: Map<AnyAtom, number>  // dependencies with their epoch numbers
///   p: Set<AnyAtom>          // pending promises
///   n: number                // epoch number (version)
///   v?: Value                // cached value
///   e?: AnyError             // error if read failed
/// }
/// ```
///
/// The AtomState tracks:
/// - Current value (or error)
/// - Epoch number (version) for cache invalidation
/// - Dependencies with their epoch numbers when this was computed
/// - Pending promises (for async atoms)
///
/// **FP Pattern**: Immutable state snapshots with version numbers
#[derive(Debug, Clone)]
pub struct AtomState<T: Clone> {
    /// Dependencies: map of atom ID to the epoch number when read
    ///
    /// This enables intelligent cache invalidation. We can check if a dependency
    /// has changed by comparing its current epoch to the stored epoch.
    ///
    /// **FP Pattern**: Epoch-based cache invalidation (instead of deep equality)
    ///
    /// TODO: Phase 2.1 - Track dependencies during read
    /// TODO: Phase 2.4 - Use for cache validation
    pub dependencies: HashMap<AtomId, EpochNumber>,

    /// Pending promises that this atom depends on
    ///
    /// TODO: Phase 6.1 - Track async dependencies
    pub pending_promises: HashSet<AtomId>,

    /// Current epoch number (incremented on each change)
    ///
    /// **FP Pattern**: Version numbers for immutability tracking
    ///
    /// TODO: Phase 1.4 - Increment on value changes
    /// TODO: Phase 2.4 - Use for cache validation
    pub epoch: EpochNumber,

    /// Cached value (if computed and fresh)
    ///
    /// TODO: Phase 1.3 - Store computed values
    /// TODO: Phase 2.4 - Validate freshness before returning
    pub value: Option<Result<T>>,

    // TODO: Phase 6.1 - Add promise tracking
    // pub promise: Option<Arc<dyn Future<Output = Result<T>> + Send + Sync>>,
}

impl<T: Clone> AtomState<T> {
    /// Create a new uninitialized atom state
    ///
    /// TODO: Phase 1.2 - Implement state initialization
    /// Hint: Create AtomState with empty dependencies, no pending promises, epoch 0, and None value
    pub fn new() -> Self {
        todo!("Implement AtomState::new - Phase 1.2: Initialize empty state")
    }

    /// Create an atom state with an initial value
    ///
    /// TODO: Phase 1.2 - Implement state with initial value
    /// Hint: Same as new() but set value to Some(Ok(value))
    pub fn with_value(value: T) -> Self {
        todo!("Implement AtomState::with_value - Phase 1.2: Initialize state with given value")
    }

    /// Check if the cached value is fresh (dependencies haven't changed)
    ///
    /// Reference: `jotai/src/vanilla/internals.ts` (cache checking in readAtomState)
    ///
    /// Returns true if:
    /// 1. We have a cached value
    /// 2. All dependencies are at the same epoch as when we computed
    ///
    /// **FP Pattern**: Epoch-based memoization
    ///
    /// TODO: Phase 2.4 - Implement cache validation
    pub fn is_fresh(&self, get_epoch: impl Fn(AtomId) -> Option<EpochNumber>) -> bool {
        // TODO: Check if value exists
        // TODO: For each dependency, check if epoch matches
        todo!("AtomState::is_fresh - Phase 2.4")
    }

    /// Mark this state as stale (needs recomputation)
    ///
    /// TODO: Phase 2.3 - Use in invalidation
    pub fn invalidate(&mut self) {
        // Option 1: Clear the value
        // self.value = None;

        // Option 2: Increment epoch (marks as changed)
        // self.epoch += 1;

        // TODO: Decide on invalidation strategy
        todo!("AtomState::invalidate - Phase 2.3")
    }

    /// Update the value and increment epoch
    ///
    /// TODO: Phase 1.4 - Implement value update with epoch increment
    /// Hint: Set self.value = Some(Ok(value)) and increment self.epoch
    pub fn set_value(&mut self, value: T) {
        todo!("Implement set_value - Phase 1.4: Update value and increment epoch")
    }

    /// Update with an error
    ///
    /// TODO: Phase 8.3 - Implement error storage with epoch increment
    /// Hint: Set self.value = Some(Err(error)) and increment self.epoch
    pub fn set_error(&mut self, error: AtomError) {
        todo!("Implement set_error - Phase 8.3: Store error and increment epoch")
    }

    /// Record a dependency
    ///
    /// TODO: Phase 2.1 - Implement dependency tracking
    /// Hint: Insert the atom_id and epoch into self.dependencies HashMap
    pub fn add_dependency(&mut self, atom_id: AtomId, epoch: EpochNumber) {
        todo!("Implement add_dependency - Phase 2.1: Insert dependency into HashMap")
    }

    /// Clear all dependencies (before recomputing)
    ///
    /// TODO: Phase 2.2 - Implement dependency clearing
    /// Hint: Call self.dependencies.clear()
    pub fn clear_dependencies(&mut self) {
        todo!("Implement clear_dependencies - Phase 2.2: Clear the dependencies HashMap")
    }
}

impl<T: Clone> Default for AtomState<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Mounted state for a subscribed atom
///
/// Reference: `jotai/src/vanilla/internals.ts` (Mounted type ~line 70)
///
/// ```typescript
/// type Mounted = {
///   l: Set<() => void>  // listeners
///   d: Set<AnyAtom>     // dependencies
///   t: Set<AnyAtom>     // dependents (atoms depending on this)
///   u?: () => void      // unmount cleanup callback
/// }
/// ```
///
/// The Mounted structure tracks subscription information for an atom.
/// Only atoms with active subscriptions have a Mounted entry.
///
/// **FP Pattern**: Observer pattern, lazy mounting
pub struct Mounted {
    /// Listeners to notify when this atom changes
    ///
    /// **FP Pattern**: Observer pattern callbacks
    ///
    /// TODO: Phase 3.2 - Add listeners on subscribe
    /// TODO: Phase 3.3 - Call listeners on change
    pub listeners: Vec<Listener>,

    /// Dependencies: atoms this atom reads from
    ///
    /// Used to know what to mount when this atom is mounted.
    ///
    /// TODO: Phase 3.4 - Track for recursive mounting
    pub dependencies: HashSet<AtomId>,

    /// Dependents: atoms that read from this atom
    ///
    /// Used to propagate invalidation and to know if this atom is still needed.
    ///
    /// TODO: Phase 2.3 - Use for invalidation propagation
    /// TODO: Phase 3.2 - Use for automatic unmounting
    pub dependents: HashSet<AtomId>,

    /// Cleanup function returned by onMount callback
    ///
    /// **FP Pattern**: Closure for lifecycle cleanup
    ///
    /// TODO: Phase 8.1 - Store cleanup from onMount
    /// TODO: Phase 3.2 - Call on unmount
    pub cleanup: Option<OnUnmount>,
}

impl Mounted {
    /// Create a new Mounted entry
    ///
    /// TODO: Phase 3.2 - Implement Mounted initialization
    /// Hint: Create Mounted with empty Vec for listeners, empty HashSets for deps/dependents, None cleanup
    pub fn new() -> Self {
        todo!("Implement Mounted::new - Phase 3.2: Initialize empty mounted state")
    }

    /// Add a listener
    ///
    /// TODO: Phase 3.2 - Implement listener registration
    /// Hint: Push the listener onto self.listeners Vec
    pub fn add_listener(&mut self, listener: Listener) {
        todo!("Implement add_listener - Phase 3.2: Add listener to the Vec")
    }

    /// Remove a listener
    ///
    /// Returns true if there are no more listeners (should unmount).
    ///
    /// TODO: Phase 3.2 - Call in unsubscribe function
    pub fn remove_listener(&mut self, _listener: &Listener) -> bool {
        // TODO: This is tricky because we need to compare function pointers
        // Might need to use an ID system instead
        todo!("Mounted::remove_listener - Phase 3.2")
    }

    /// Check if there are any listeners
    ///
    /// TODO: Phase 3.2 - Implement listener check
    /// Hint: Return !self.listeners.is_empty()
    pub fn has_listeners(&self) -> bool {
        todo!("Implement has_listeners - Phase 3.2: Check if listeners Vec is empty")
    }

    /// Add a dependency
    ///
    /// TODO: Phase 3.4 - Implement dependency tracking for mounting
    /// Hint: Insert atom_id into self.dependencies HashSet
    pub fn add_dependency(&mut self, atom_id: AtomId) {
        todo!("Implement add_dependency - Phase 3.4: Insert into dependencies HashSet")
    }

    /// Add a dependent
    ///
    /// TODO: Phase 2.1 - Implement reverse dependency tracking
    /// Hint: Insert atom_id into self.dependents HashSet
    pub fn add_dependent(&mut self, atom_id: AtomId) {
        todo!("Implement add_dependent - Phase 2.1: Insert into dependents HashSet")
    }

    /// Remove a dependent
    ///
    /// TODO: Phase 3.2 - Implement dependent removal
    /// Hint: Call self.dependents.remove(atom_id)
    pub fn remove_dependent(&mut self, atom_id: &AtomId) {
        todo!("Implement remove_dependent - Phase 3.2: Remove from dependents HashSet")
    }

    /// Call all listeners
    ///
    /// TODO: Phase 3.3 - Implement listener notification
    /// Hint: Iterate over self.listeners and call each one
    pub fn notify_listeners(&self) {
        todo!("Implement notify_listeners - Phase 3.3: Iterate and call all listeners")
    }

    /// Call cleanup callback if present
    ///
    /// TODO: Phase 8.1 - Implement cleanup execution
    /// Hint: Check if self.cleanup is Some, if so extract and call it
    pub fn cleanup(self) {
        todo!("Implement cleanup - Phase 8.1: Call cleanup callback if present")
    }
}

impl Default for Mounted {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for Mounted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mounted")
            .field("listeners_count", &self.listeners.len())
            .field("dependencies", &self.dependencies)
            .field("dependents", &self.dependents)
            .field("has_cleanup", &self.cleanup.is_some())
            .finish()
    }
}

/// Helper structure for dependency tracking during reads
///
/// When reading an atom, we need to track which other atoms it depends on.
/// This structure is passed as the Getter implementation to the read function.
///
/// TODO: Phase 2.1 - Implement as Getter trait
pub struct DependencyTracker<'a> {
    /// Reference to the store
    pub store: &'a crate::store::Store,

    /// The atom being read (to record dependencies)
    pub reading_atom: AtomId,

    /// Dependencies discovered during this read
    pub discovered_dependencies: Arc<RwLock<HashMap<AtomId, EpochNumber>>>,
}

// TODO: Phase 2.1 - Implement Getter for DependencyTracker

/// Helper structure for setting values during writes
///
/// TODO: Phase 1.4 - Implement as Setter trait
pub struct ValueSetter<'a> {
    /// Reference to the store
    pub store: &'a crate::store::Store,

    /// Atoms that were changed during this operation
    pub changed_atoms: Arc<RwLock<HashSet<AtomId>>>,
}

// TODO: Phase 1.4 - Implement Setter for ValueSetter

/// Graph traversal helper for topological sort
///
/// Used to determine the correct order for recomputing invalidated atoms.
///
/// TODO: Phase 4.1 - Implement DFS-based topological sort
pub struct TopologicalSorter {
    /// Atoms to sort
    pub atoms: Vec<AtomId>,

    /// Dependency relationships
    pub dependencies: HashMap<AtomId, HashSet<AtomId>>,
}

impl TopologicalSorter {
    /// Perform topological sort using DFS
    ///
    /// Reference: `jotai/src/vanilla/internals.ts` (DFS in recomputeInvalidatedAtoms)
    ///
    /// Returns atoms in dependency order (dependencies before dependents).
    ///
    /// **FP Pattern**: Recursion for graph traversal
    ///
    /// TODO: Phase 4.1 - Implement
    pub fn sort(&self) -> Result<Vec<AtomId>> {
        // TODO: Implement DFS-based topological sort
        // 1. Create visited and visiting sets
        // 2. For each atom, run DFS
        // 3. Detect cycles (visiting set)
        // 4. Add to result in post-order
        todo!("TopologicalSorter::sort - Phase 4.1")
    }

    /// DFS helper function
    ///
    /// TODO: Phase 4.1 - Implement recursive DFS
    fn dfs(
        &self,
        atom: AtomId,
        visited: &mut HashSet<AtomId>,
        visiting: &mut HashSet<AtomId>,
        result: &mut Vec<AtomId>,
    ) -> Result<()> {
        // TODO: Implement DFS
        // - Check if already visited (return)
        // - Check if currently visiting (cycle error)
        // - Mark as visiting
        // - Visit all dependencies
        // - Mark as visited
        // - Add to result
        todo!("TopologicalSorter::dfs - Phase 4.1")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "AtomState::new")]
    fn test_atom_state_creation() {
        // Test that AtomState::new creates proper initial state
        let state: AtomState<i32> = AtomState::new();
        assert_eq!(state.epoch, 0);
        assert!(state.value.is_none());
        assert!(state.dependencies.is_empty());
    }

    #[test]
    #[should_panic(expected = "AtomState::with_value")]
    fn test_atom_state_with_value() {
        // Test that AtomState::with_value creates state with initial value
        let state = AtomState::with_value(42);
        assert_eq!(state.epoch, 0);
        assert!(state.value.is_some());
        assert_eq!(state.value.unwrap().unwrap(), 42);
    }

    #[test]
    #[should_panic(expected = "set_value")]
    fn test_atom_state_set_value() {
        // Test that set_value updates the value and increments epoch
        let mut state: AtomState<i32> = AtomState::new();
        state.set_value(100);
        assert_eq!(state.epoch, 1);
        assert_eq!(state.value.as_ref().unwrap().as_ref().unwrap(), &100);
    }

    #[test]
    #[should_panic(expected = "Mounted::new")]
    fn test_mounted_creation() {
        // Test that Mounted::new creates proper initial state
        let mounted = Mounted::new();
        assert!(mounted.listeners.is_empty());
        assert!(mounted.dependencies.is_empty());
        assert!(mounted.dependents.is_empty());
        assert!(mounted.cleanup.is_none());
    }

    #[test]
    #[should_panic(expected = "add_dependency")]
    fn test_mounted_add_dependency() {
        // Test that add_dependency properly inserts into the HashSet
        let mut mounted = Mounted::new();
        mounted.add_dependency(1);
        mounted.add_dependency(2);
        assert_eq!(mounted.dependencies.len(), 2);
        assert!(mounted.dependencies.contains(&1));
        assert!(mounted.dependencies.contains(&2));
    }

    // TODO: Phase 2.4 - Add tests for is_fresh
    // TODO: Phase 3.3 - Add tests for notify_listeners
    // TODO: Phase 4.1 - Add tests for topological sort
}
