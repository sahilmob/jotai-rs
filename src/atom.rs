//! Atom primitives and factory functions
//!
//! Reference: `jotai/src/vanilla/atom.ts`
//!
//! This module defines the core `Atom` types and the `atom()` factory function
//! for creating atoms. Atoms are immutable configuration objects that represent
//! pieces of state or derived computations.
//!
//! ## Functional Programming Patterns
//! - Factory pattern: `atom()` function creates configured objects
//! - Immutability: Atom config never changes after creation
//! - First-class functions: Read/write functions stored as data
//! - Type-level programming: Complex type relationships

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::types::{AtomId, ReadFn, WriteFn, Getter, Setter, OnUnmount};
use crate::error::Result;

/// Global atom ID counter
///
/// Reference: `jotai/src/vanilla/atom.ts:73`
///
/// ```typescript
/// let keyCount = 0
/// ```
///
/// Each atom gets a unique ID for identification. This is more efficient
/// than string-based keys and enables WeakMap-like behavior.
///
/// **FP Pattern**: Closure captures this counter
static ATOM_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Generate the next unique atom ID
///
/// **FP Pattern**: Side effect encapsulated in a function
///
/// TODO: Phase 1.1 - Implement atomic counter
/// Hint: Use ATOM_ID_COUNTER.fetch_add(1, Ordering::Relaxed) to atomically increment and return the ID
fn next_atom_id() -> AtomId {
    todo!("Implement next_atom_id - Phase 1.1: Use fetch_add on ATOM_ID_COUNTER")
}

/// Core atom type
///
/// Reference: `jotai/src/vanilla/atom.ts:42-56`
///
/// ```typescript
/// export interface Atom<Value> {
///   toString: () => string
///   read: Read<Value>
///   debugLabel?: string
///   debugPrivate?: boolean
///   unstable_onInit?: (store: Store) => void
/// }
/// ```
///
/// Atoms are immutable configuration objects. They describe how to compute
/// a value, but don't store the value themselves.
///
/// **FP Pattern**: Immutability, first-class functions
#[derive(Clone)]
pub struct Atom<T: Clone + Send + Sync + 'static> {
    /// Unique identifier for this atom
    pub(crate) id: AtomId,

    /// Function that computes this atom's value
    ///
    /// **FP Pattern**: Pure function, lazy evaluation
    ///
    /// The read function is called with a Getter that provides access to
    /// other atoms. It should be deterministic based on its dependencies.
    ///
    /// TODO: Phase 1.1 - Store read function
    /// TODO: Phase 2.2 - Support dependency tracking through Getter
    pub(crate) read_fn: ReadFn<T>,

    /// Optional debug label for development
    ///
    /// Reference: `jotai/src/vanilla/atom.ts:45`
    pub(crate) debug_label: Option<String>,

    /// Marker for type safety
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + Send + Sync + 'static> Atom<T> {
    /// Get the atom's unique ID
    pub fn id(&self) -> AtomId {
        self.id
    }

    /// Get the atom's debug label, if any
    pub fn debug_label(&self) -> Option<&str> {
        self.debug_label.as_deref()
    }

    /// Set or update the debug label (builder pattern)
    ///
    /// TODO: Phase 1.1 - Implement builder pattern for debug label
    /// Hint: Set self.debug_label = Some(label.into()) and return self
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        todo!("Implement with_label - Phase 1.1: Set debug_label and return self for builder pattern")
    }

    /// Convert atom to string representation
    ///
    /// Reference: `jotai/src/vanilla/atom.ts:105-109`
    ///
    /// ```typescript
    /// toString() {
    ///   return import.meta.env?.MODE !== 'production' && this.debugLabel
    ///     ? key + ':' + this.debugLabel
    ///     : key
    /// }
    /// ```
    /// TODO: Phase 1.1 - Implement string representation
    /// Hint: If debug_label exists, format as "atom{id}:{label}", otherwise "atom{id}"
    pub fn to_string(&self) -> String {
        todo!("Implement to_string - Phase 1.1: Format as 'atomN' or 'atomN:label'")
    }

    /// Call the read function to compute the value
    ///
    /// This is used internally by the store.
    ///
    /// TODO: Phase 1.3 - Use in store.get()
    /// TODO: Phase 1.3 - Pass proper context (Store reference) to read_fn
    /// Hint: Simply call (self.read_fn)() to invoke the stored function
    pub(crate) fn read(&self) -> Result<T> {
        todo!("Implement Atom::read - Phase 1.3: Call the read_fn closure")
    }
}

impl<T: Clone + Send + Sync + 'static> std::fmt::Debug for Atom<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Atom")
            .field("id", &self.id)
            .field("debug_label", &self.debug_label)
            .finish()
    }
}

impl<T: Clone + Send + Sync + 'static> std::fmt::Display for Atom<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Writable atom type
///
/// Reference: `jotai/src/vanilla/atom.ts:58-63`
///
/// ```typescript
/// export interface WritableAtom<Value, Args, Result> extends Atom<Value> {
///   read: Read<Value, SetAtom<Args, Result>>
///   write: Write<Args, Result>
///   onMount?: OnMount<Args, Result>
/// }
/// ```
///
/// **FP Pattern**: Extension with additional capabilities (write function)
#[derive(Clone)]
pub struct WritableAtom<T: Clone + Send + Sync + 'static> {
    /// Base atom with read functionality
    pub(crate) atom: Atom<T>,

    /// Function that handles writes to this atom
    ///
    /// **FP Pattern**: State transformation function
    ///
    /// The write function receives:
    /// - Getter: to read current state
    /// - Setter: to update state
    /// - Value: the new value/action
    ///
    /// TODO: Phase 1.4 - Implement write handling
    /// TODO: Phase 5.1 - Support complex write patterns
    pub(crate) write_fn: WriteFn<T>,

    /// Optional mount callback
    ///
    /// Reference: `jotai/src/vanilla/atom.ts:62`
    ///
    /// Called when the atom is first subscribed to.
    /// Can return a cleanup function to be called on unmount.
    ///
    /// **FP Pattern**: Closure for lifecycle management
    ///
    /// Note: Removed Setter parameter for now to avoid dyn compatibility issues
    /// TODO: Phase 8.1 - Implement onMount lifecycle with proper setter access
    pub(crate) on_mount: Option<Arc<dyn Fn() -> Option<OnUnmount> + Send + Sync>>,
}

impl<T: Clone + Send + Sync + 'static> WritableAtom<T> {
    /// Get the underlying base atom
    pub fn as_atom(&self) -> &Atom<T> {
        &self.atom
    }

    /// Get the atom's unique ID
    pub fn id(&self) -> AtomId {
        self.atom.id()
    }

    /// Call the write function
    ///
    /// TODO: Phase 1.4 - Use in store.set()
    /// TODO: Phase 1.4 - Pass proper context (Store reference) to write_fn
    /// Hint: Call (self.write_fn)(value) to invoke the stored write function
    pub(crate) fn write(&self, value: T) -> Result<()> {
        todo!("Implement WritableAtom::write - Phase 1.4: Call the write_fn closure with value")
    }

    /// Call the onMount callback if present
    ///
    /// TODO: Phase 8.1 - Use in store subscription mounting
    /// Hint: Check if on_mount exists, if so call it and return the result (Option<OnUnmount>)
    pub(crate) fn on_mount(&self) -> Option<OnUnmount> {
        todo!("Implement on_mount - Phase 8.1: Call on_mount callback if present")
    }
}

impl<T: Clone + Send + Sync + 'static> std::fmt::Debug for WritableAtom<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WritableAtom")
            .field("id", &self.atom.id)
            .field("debug_label", &self.atom.debug_label)
            .field("has_on_mount", &self.on_mount.is_some())
            .finish()
    }
}

/// Primitive atom type (shorthand for writable atom with simple value)
///
/// Reference: `jotai/src/vanilla/atom.ts:67-71`
///
/// ```typescript
/// export type PrimitiveAtom<Value> = WritableAtom<
///   Value,
///   [SetStateAction<Value>],
///   void
/// >
/// ```
pub type PrimitiveAtom<T> = WritableAtom<T>;

// ============================================================================
// ATOM FACTORY FUNCTIONS
// ============================================================================
//
// Reference: `jotai/src/vanilla/atom.ts:76-122`
//
// Jotai uses function overloading for different atom types.
// In Rust, we use separate functions with descriptive names.
//
// **FP Pattern**: Factory functions, builder pattern

/// Create a primitive atom with an initial value
///
/// Reference: `jotai/src/vanilla/atom.ts:95-97` (primitive atom overload)
///
/// ```typescript
/// export function atom<Value>(initialValue: Value): PrimitiveAtom<Value>
/// ```
///
/// This is the simplest form of atom - it holds a value that can be read and written.
///
/// **FP Pattern**: Factory function, closure captures initial value
///
/// # Example
///
/// ```rust,ignore
/// use jotai_rs::atom;
///
/// let count = atom(0);
/// ```
///
/// TODO: Phase 1.1 - Implement primitive atom factory
/// Hint:
/// 1. Call next_atom_id() to get a unique ID
/// 2. Create read_fn that will read from store (for now, just todo!())
/// 3. Create write_fn that will write to store (for now, just todo!())
/// 4. Build and return WritableAtom with these functions
/// Note: The actual read/write logic happens in the store, not here
pub fn atom<T: Clone + Send + Sync + 'static>(initial_value: T) -> PrimitiveAtom<T> {
    todo!("Implement atom() factory - Phase 1.1: Generate ID, create read_fn and write_fn closures")
}

/// Create a read-only derived atom
///
/// Reference: `jotai/src/vanilla/atom.ts:82` (read-only atom overload)
///
/// ```typescript
/// export function atom<Value>(read: Read<Value>): Atom<Value>
/// ```
///
/// Derived atoms compute their value based on other atoms. The read function
/// receives a Getter to access dependencies.
///
/// **FP Pattern**: Function composition, pure functions
///
/// # Example
///
/// ```rust,ignore
/// use jotai_rs::{atom, atom_derived};
///
/// let count = atom(0);
/// let double = atom_derived(move |get| {
///     get(&count) * 2
/// });
/// ```
///
/// TODO: Phase 2.2 - Implement with dependency tracking
/// Hint:
/// 1. Generate a new atom ID
/// 2. Capture the user's read function (the F parameter)
/// 3. Create a read_fn closure that will call the user's read function with a Getter
/// 4. Return an Atom with this read_fn
/// Note: Dependency tracking happens when the read function calls get() on other atoms
pub fn atom_derived<T, F>(_read: F) -> Atom<T>
where
    T: Clone + Send + Sync + 'static,
    F: Fn(&dyn Getter) -> Result<T> + Send + Sync + 'static,
{
    todo!("Implement atom_derived - Phase 2.2: Create derived atom with dependency tracking")
}

/// Create a writable derived atom with custom read and write logic
///
/// Reference: `jotai/src/vanilla/atom.ts:76-79` (writable derived atom overload)
///
/// ```typescript
/// export function atom<Value, Args, Result>(
///   read: Read<Value, SetAtom<Args, Result>>,
///   write: Write<Args, Result>,
/// ): WritableAtom<Value, Args, Result>
/// ```
///
/// Writable derived atoms can have custom logic for both reading and writing.
/// The write function can update multiple other atoms.
///
/// **FP Pattern**: Higher-order functions, state transformations
///
/// # Example
///
/// ```rust,ignore
/// use jotai_rs::{atom, atom_writable};
///
/// let first = atom("John".to_string());
/// let last = atom("Doe".to_string());
///
/// let full_name = atom_writable(
///     |get| format!("{} {}", get(&first), get(&last)),
///     |get, set, value: String| {
///         let parts: Vec<&str> = value.split(' ').collect();
///         if parts.len() == 2 {
///             set(&first, parts[0].to_string());
///             set(&last, parts[1].to_string());
///         }
///     }
/// );
/// ```
///
/// TODO: Phase 5.1 - Implement writable derived atoms
/// Hint:
/// 1. Generate a new atom ID
/// 2. Capture both the read and write functions
/// 3. Create read_fn that calls the user's read function with Getter
/// 4. Create write_fn that calls the user's write function with Getter and Setter
/// 5. Return WritableAtom with both functions
pub fn atom_writable<T, R, W>(_read: R, _write: W) -> WritableAtom<T>
where
    T: Clone + Send + Sync + 'static,
    R: Fn(&dyn Getter) -> Result<T> + Send + Sync + 'static,
    W: Fn(&dyn Getter, &dyn Setter, T) -> Result<()> + Send + Sync + 'static,
{
    todo!("Implement atom_writable - Phase 5.1: Create atom with custom read and write logic")
}

/// Create a write-only atom (read returns initial value)
///
/// Reference: `jotai/src/vanilla/atom.ts:84-88` (write-only atom overload)
///
/// ```typescript
/// export function atom<Value, Args, Result>(
///   initialValue: Value,
///   write: Write<Args, Result>,
/// ): WritableAtom<Value, Args, Result> & WithInitialValue<Value>
/// ```
///
/// **FP Pattern**: Action-only atoms (like commands/effects)
///
/// TODO: Phase 5.3 - Implement write-only atoms
pub fn atom_write_only<T, W>(initial_value: T, write: W) -> WritableAtom<T>
where
    T: Clone + Send + Sync + 'static,
    W: Fn(&dyn Getter, &dyn Setter, T) -> Result<()> + Send + Sync + 'static,
{
    // TODO: Phase 5.3 - Similar to primitive atom but with custom write
    todo!("Write-only atoms - Phase 5.3")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "atom()")]
    fn test_atom_id_generation() {
        // Test that each atom gets a unique, incrementing ID
        let a1 = atom(1);
        let a2 = atom(2);
        let a3 = atom(3);

        // IDs should be unique and sequential
        assert!(a1.id() < a2.id());
        assert!(a2.id() < a3.id());
    }

    #[test]
    #[should_panic]
    fn test_atom_debug_label() {
        // Test that with_label sets the debug label correctly
        let count = atom(0).with_label("count");
        assert_eq!(count.as_atom().debug_label(), Some("count"));
        assert!(count.as_atom().to_string().contains("count"));
    }

    #[test]
    #[should_panic]
    fn test_atom_to_string() {
        // Test that to_string formats atoms correctly
        let unnamed = atom(42);
        let named = atom(42).with_label("answer");

        assert!(unnamed.as_atom().to_string().starts_with("atom"));
        assert!(named.as_atom().to_string().contains("answer"));
    }

    // TODO: Phase 1.3 - Add tests for atom read function
    // TODO: Phase 1.4 - Add tests for atom write function
    // TODO: Phase 2.2 - Add tests for derived atoms with dependencies
    // TODO: Phase 5.1 - Add tests for writable derived atoms
}
