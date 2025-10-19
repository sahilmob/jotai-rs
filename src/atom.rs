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

use crate::error::Result;
use crate::types::{AtomId, Getter, OnUnmount, ReadFn, Setter, WriteFn};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

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
    ATOM_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
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
        self.debug_label = Some(label.into());

        self
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
        match self.debug_label.as_ref() {
            Some(label) => format!("atom{}:{}", self.id, label),
            None => format!("atom{}", self.id),
        }
    }

    /// Call the read function to compute the value
    ///
    /// This is used internally by the store.
    ///
    /// TODO: Phase 1.3 - Use in store.get()
    /// TODO: Phase 1.3 - Pass proper context (Store reference) to read_fn
    /// Hint: Simply call (self.read_fn)() to invoke the stored function
    pub(crate) fn read(&self) -> Result<T> {
        (self.read_fn)()
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
        (self.write_fn)(value)
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.atom.debug_label = Some(label.into());

        self
    }

    /// Call the onMount callback if present
    ///
    /// TODO: Phase 8.1 - Use in store subscription mounting
    /// Hint: Check if on_mount exists, if so call it and return the result (Option<OnUnmount>)
    pub(crate) fn on_mount(&self) -> Option<OnUnmount> {
        match self.on_mount.as_ref() {
            Some(f) => f(),
            None => None,
        }
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
pub fn atom<T: Clone + Send + Sync + 'static>(_initial_value: T) -> PrimitiveAtom<T> {
    // For primitive atoms, the store handles read/write directly
    // These functions should never be called
    let read_fn = Arc::new(|| unreachable!("Primitive atom read handled by store"));
    let write_fn = Arc::new(|_| unreachable!("Primitive atom write handled by store"));

    PrimitiveAtom {
        atom: Atom {
            id: next_atom_id(),
            read_fn,
            debug_label: None,
            _phantom: PhantomData,
        },
        on_mount: None,
        write_fn,
    }
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
pub fn atom_derived<T, F>(read: F) -> Atom<T>
where
    T: Clone + Send + Sync + 'static,
    F: Fn(&dyn Getter) -> Result<T> + Send + Sync + 'static,
{
    let read_fn = Arc::new(|| unreachable!());
    Atom {
        id: next_atom_id(),
        read_fn,
        debug_label: None,
        _phantom: PhantomData,
    }
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
pub fn atom_writable<T, R, W>(read: R, write: W) -> WritableAtom<T>
where
    T: Clone + Send + Sync + 'static,
    R: Fn(&dyn Getter) -> Result<T> + Send + Sync + 'static,
    W: Fn(&dyn Getter, &dyn Setter, T) -> Result<()> + Send + Sync + 'static,
{
    let read_fn = Arc::new(|| unreachable!());
    let write_fn = Arc::new(|v| unreachable!());
    WritableAtom {
        atom: Atom {
            id: next_atom_id(),
            read_fn,
            debug_label: None,
            _phantom: PhantomData,
        },
        write_fn,
        on_mount: None,
    }
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
pub fn atom_write_only<T, W>(initial_value: T, _write: W) -> WritableAtom<T>
where
    T: Clone + Send + Sync + 'static,
    W: Fn(&dyn Getter, &dyn Setter, T) -> Result<()> + Send + Sync + 'static,
{
    let write_fn = Arc::new(|_| unreachable!("Write-only atom write handled by store"));
    WritableAtom {
        atom: Atom {
            id: next_atom_id(),
            read_fn: Arc::new(move || Ok(initial_value.clone())), // Clone on each call
            debug_label: None,
            _phantom: PhantomData,
        },
        write_fn,
        on_mount: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Phase 1.1: Core Atom Creation and ID Generation
    // ========================================================================

    #[test]
    fn test_atom_id_generation() {
        // Test that each atom gets a unique, incrementing ID
        let a1 = atom(1);
        let a2 = atom(2);
        let a3 = atom(3);

        // IDs should be unique and sequential
        assert!(a1.id() < a2.id());
        assert!(a2.id() < a3.id());

        // IDs should be exactly 1 apart (sequential)
        assert_eq!(a1.id() + 1, a2.id());
        assert_eq!(a2.id() + 1, a3.id());
    }

    #[test]
    fn test_atom_id_uniqueness() {
        // Even atoms with the same value should have different IDs
        let a1 = atom(42);
        let a2 = atom(42);
        let a3 = atom(42);

        assert_ne!(a1.id(), a2.id());
        assert_ne!(a2.id(), a3.id());
        assert_ne!(a1.id(), a3.id());
    }

    #[test]
    fn test_different_types_share_id_counter() {
        // IDs should be global across all atom types
        let int_atom = atom(1);
        let string_atom = atom("hello".to_string());
        let bool_atom = atom(true);

        // All should have different IDs from the same counter
        assert!(int_atom.id() < string_atom.id());
        assert!(string_atom.id() < bool_atom.id());
    }

    // ========================================================================
    // Phase 1.1: Debug Labels and String Representation
    // ========================================================================

    #[test]
    fn test_atom_without_label() {
        // Atoms without labels should have None
        let count = atom(0);
        assert_eq!(count.as_atom().debug_label(), None);
    }

    #[test]
    fn test_atom_with_label() {
        // Test that with_label sets the debug label correctly
        let count = atom(0).with_label("count");
        assert_eq!(count.as_atom().debug_label(), Some("count"));

        let name = atom("John".to_string()).with_label("firstName");
        assert_eq!(name.as_atom().debug_label(), Some("firstName"));
    }

    #[test]
    fn test_atom_label_from_string() {
        // Test that with_label accepts different string types
        let a1 = atom(0).with_label("static_str");
        let a2 = atom(0).with_label(String::from("owned_string"));
        let a3 = atom(0).with_label("borrowed".to_string());

        assert_eq!(a1.as_atom().debug_label(), Some("static_str"));
        assert_eq!(a2.as_atom().debug_label(), Some("owned_string"));
        assert_eq!(a3.as_atom().debug_label(), Some("borrowed"));
    }

    #[test]
    fn test_atom_to_string_without_label() {
        // Atoms without labels should format as "atom{id}"
        let unnamed = atom(42);
        let s = unnamed.as_atom().to_string();

        assert!(s.starts_with("atom"));
        assert!(!s.contains(':'));

        // Should be able to parse the ID from the string
        let id_str = s.strip_prefix("atom").unwrap();
        let parsed_id: usize = id_str.parse().unwrap();
        assert_eq!(parsed_id, unnamed.id());
    }

    #[test]
    fn test_atom_to_string_with_label() {
        // Atoms with labels should format as "atom{id}:{label}"
        let named = atom(42).with_label("answer");
        let s = named.as_atom().to_string();

        assert!(s.starts_with("atom"));
        assert!(s.contains(':'));
        assert!(s.contains("answer"));

        // Format should be exactly "atom{id}:answer"
        let expected = format!("atom{}:answer", named.id());
        assert_eq!(s, expected);
    }

    #[test]
    fn test_atom_display_trait() {
        // Test that Display trait uses to_string()
        let atom1 = atom(100);
        let atom2 = atom(200).with_label("test");

        assert_eq!(format!("{}", atom1.as_atom()), atom1.as_atom().to_string());
        assert_eq!(format!("{}", atom2.as_atom()), atom2.as_atom().to_string());
    }

    #[test]
    fn test_atom_debug_trait() {
        // Test that Debug trait is implemented
        let count = atom(5).with_label("counter");
        let debug_str = format!("{:?}", count);

        assert!(debug_str.contains("WritableAtom"));
        assert!(debug_str.contains("id"));
        assert!(debug_str.contains("debug_label"));
    }

    // ========================================================================
    // Phase 1.1: Atom Types and Factory Functions
    // ========================================================================

    #[test]
    fn test_primitive_atom_creation() {
        // Test creating primitive atoms with different types
        let _int_atom = atom(42);
        let _float_atom = atom(3.14);
        let _bool_atom = atom(true);
        let _string_atom = atom(String::from("hello"));
        let _vec_atom = atom(vec![1, 2, 3]);

        // If we got here without panicking, primitive atoms work
    }

    // NOTE: Tests for atom_derived, atom_writable, and atom_write_only are
    // disabled because they require dyn-compatible Getter/Setter traits.
    // These will be testable in Phase 2 when we implement the Store.

    // TODO: Phase 2.2 - Re-enable these tests when Store is implemented
    // #[test]
    // fn test_derived_atom_creation() { ... }
    // #[test]
    // fn test_writable_atom_creation() { ... }
    // #[test]
    // fn test_write_only_atom_creation() { ... }
    // #[test]
    // fn test_derived_atom_has_unique_id() { ... }
    // #[test]
    // fn test_derived_atom_with_label() { ... }
    // #[test]
    // fn test_writable_atom_with_label() { ... }

    // ========================================================================
    // Phase 1.1: Atom Cloning and Ownership
    // ========================================================================

    #[test]
    fn test_atom_clone() {
        // Atoms should be cloneable
        let original = atom(42).with_label("original");
        let cloned = original.clone();

        // Both should have the same ID (they're the same atom)
        assert_eq!(original.id(), cloned.id());
        assert_eq!(original.as_atom().debug_label(), cloned.as_atom().debug_label());
    }

    // TODO: Phase 2.2 - Re-enable when Store is implemented
    // #[test]
    // fn test_derived_atom_clone() {
    //     // Derived atoms should be cloneable
    //     let original = atom_derived(|_get| Ok(100)).with_label("test");
    //     let cloned = original.clone();
    //
    //     assert_eq!(original.id(), cloned.id());
    //     assert_eq!(original.debug_label(), cloned.debug_label());
    // }

    #[test]
    fn test_atom_as_atom() {
        // WritableAtom should provide access to underlying Atom
        let writable = atom(42).with_label("test");
        let atom_ref = writable.as_atom();

        assert_eq!(atom_ref.id(), writable.id());
        assert_eq!(atom_ref.debug_label(), Some("test"));
    }

    // ========================================================================
    // Phase 1.1: Type Safety Tests
    // ========================================================================

    #[test]
    fn test_atom_with_complex_types() {
        // Test that atoms work with complex types
        #[derive(Clone, Debug, PartialEq)]
        struct User {
            name: String,
            age: u32,
        }

        let user_atom = atom(User {
            name: "Alice".to_string(),
            age: 30,
        }).with_label("currentUser");

        assert_eq!(user_atom.as_atom().debug_label(), Some("currentUser"));
    }

    #[test]
    fn test_atom_with_option() {
        // Test atoms with Option types
        let some_atom = atom(Some(42));
        let none_atom = atom::<Option<i32>>(None);

        assert_ne!(some_atom.id(), none_atom.id());
    }

    #[test]
    fn test_atom_with_result() {
        // Test atoms with Result types
        let ok_atom = atom::<Result<i32>>(Ok(42));
        let err_atom = atom::<Result<i32>>(Err(crate::error::AtomError::Generic("test error".to_string())));

        assert_ne!(ok_atom.id(), err_atom.id());
    }

    // ========================================================================
    // Phase 1.1: Builder Pattern Tests
    // ========================================================================

    #[test]
    fn test_builder_pattern_chaining() {
        // Test that with_label returns self for chaining
        let atom1 = atom(42).with_label("answer");

        // Should be able to use immediately
        assert_eq!(atom1.as_atom().debug_label(), Some("answer"));
    }

    // ========================================================================
    // Edge Cases and Validation
    // ========================================================================

    #[test]
    fn test_atom_with_empty_string_label() {
        // Empty labels should work (though not recommended)
        let atom1 = atom(42).with_label("");
        assert_eq!(atom1.as_atom().debug_label(), Some(""));

        let s = atom1.as_atom().to_string();
        assert!(s.contains(':')); // Should still format with colon
    }

    #[test]
    fn test_atom_with_unicode_label() {
        // Unicode labels should work
        let atom1 = atom(42).with_label("数量");
        assert_eq!(atom1.as_atom().debug_label(), Some("数量"));

        let s = atom1.as_atom().to_string();
        assert!(s.contains("数量"));
    }

    #[test]
    fn test_atom_with_long_label() {
        // Long labels should work
        let long_label = "a".repeat(1000);
        let atom1 = atom(42).with_label(&long_label);
        assert_eq!(atom1.as_atom().debug_label(), Some(long_label.as_str()));
    }

    #[test]
    fn test_on_mount_none_by_default() {
        // Atoms should have no onMount callback by default
        let atom1 = atom(42);

        // We can't directly check on_mount (it's private), but we can verify
        // it doesn't panic when accessed internally
        let result = atom1.on_mount();
        assert!(result.is_none());
    }

    // TODO: Phase 1.3 - Add tests for atom read function with Store
    // TODO: Phase 1.4 - Add tests for atom write function with Store
    // TODO: Phase 2.2 - Add tests for derived atoms with dependencies
    // TODO: Phase 5.1 - Add tests for writable derived atoms
    // TODO: Phase 8.1 - Add tests for onMount lifecycle
}
