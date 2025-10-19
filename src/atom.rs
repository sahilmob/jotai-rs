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
/// TODO: Phase 1.1 - Use in atom() factory
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
    /// TODO: Phase 1.1 - Useful for debugging
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
    pub fn to_string(&self) -> String {
        match &self.debug_label {
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
    pub(crate) fn write(&self, value: T) -> Result<()> {
        (self.write_fn)(value)
    }

    /// Call the onMount callback if present
    ///
    /// TODO: Phase 8.1 - Use in store subscription mounting
    pub(crate) fn on_mount(&self) -> Option<OnUnmount> {
        self.on_mount.as_ref().and_then(|f| f())
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
/// TODO: Phase 1.1 - Implement
/// TODO: Phase 1.2 - Add to Store initialization logic
pub fn atom<T: Clone + Send + Sync + 'static>(initial_value: T) -> PrimitiveAtom<T> {
    let id = next_atom_id();

    // Create a read function that returns the stored value
    // Reference: `jotai/src/vanilla/atom.ts:124-126` (defaultRead)
    let read_fn: ReadFn<T> = Arc::new(move || {
        // TODO: Phase 1.3 - Implement actual read logic
        // This should read from the store's atom_states map
        // The Store will need to pass itself to the read function somehow
        // (likely by capturing the atom ID and looking it up)
        todo!("Primitive atom read - Phase 1.3")
    });

    // Create a write function that updates the stored value
    // Reference: `jotai/src/vanilla/atom.ts:128-140` (defaultWrite)
    let write_fn: WriteFn<T> = Arc::new(move |_value: T| {
        // TODO: Phase 1.4 - Implement actual write logic
        // This should:
        // 1. Update the value in the store
        // 2. Increment epoch number
        // 3. Invalidate dependents
        // 4. Collect for notification
        // The Store will handle this by capturing the atom ID
        todo!("Primitive atom write - Phase 1.4")
    });

    let base_atom = Atom {
        id,
        read_fn,
        debug_label: None,
        _phantom: std::marker::PhantomData,
    };

    WritableAtom {
        atom: base_atom,
        write_fn,
        on_mount: None,
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
pub fn atom_derived<T, F>(_read: F) -> Atom<T>
where
    T: Clone + Send + Sync + 'static,
    F: Fn(&dyn Getter) -> Result<T> + Send + Sync + 'static,
{
    let id = next_atom_id();

    Atom {
        id,
        read_fn: Arc::new(|| todo!("atom_derived - Phase 2.2")),
        debug_label: None,
        _phantom: std::marker::PhantomData,
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
pub fn atom_writable<T, R, W>(_read: R, _write: W) -> WritableAtom<T>
where
    T: Clone + Send + Sync + 'static,
    R: Fn(&dyn Getter) -> Result<T> + Send + Sync + 'static,
    W: Fn(&dyn Getter, &dyn Setter, T) -> Result<()> + Send + Sync + 'static,
{
    let id = next_atom_id();

    let base_atom = Atom {
        id,
        read_fn: Arc::new(|| todo!("atom_writable read - Phase 5.1")),
        debug_label: None,
        _phantom: std::marker::PhantomData,
    };

    WritableAtom {
        atom: base_atom,
        write_fn: Arc::new(|_| todo!("atom_writable write - Phase 5.1")),
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
    fn test_atom_id_generation() {
        let a1 = atom(1);
        let a2 = atom(2);
        let a3 = atom(3);

        // IDs should be unique and sequential
        assert!(a1.id() < a2.id());
        assert!(a2.id() < a3.id());
    }

    #[test]
    fn test_atom_debug_label() {
        let count = atom(0).with_label("count");
        assert_eq!(count.as_atom().debug_label(), Some("count"));
        assert!(count.as_atom().to_string().contains("count"));
    }

    #[test]
    fn test_atom_to_string() {
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
