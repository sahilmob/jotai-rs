//! Core type definitions for jotai-rs
//!
//! Reference: `jotai/src/vanilla/atom.ts:1-27`
//!
//! This module defines the fundamental types used throughout the library,
//! mirroring Jotai's TypeScript type definitions.
//!
//! ## Functional Programming Patterns
//! - First-class functions: Functions as types (Getter, Setter)
//! - Type-level programming: Complex trait bounds for safety

use std::sync::Arc;
use crate::atom::Atom;
use crate::error::Result;

/// Unique identifier for each atom
///
/// Reference: `jotai/src/vanilla/atom.ts:73` (keyCount)
///
/// In Jotai, each atom gets a unique string key like "atom0", "atom1", etc.
/// We use usize for efficiency in Rust.
pub type AtomId = usize;

/// Version number for atom state (used for cache invalidation)
///
/// Reference: `jotai/src/vanilla/internals.ts` (epoch in AtomState)
///
/// **FP Pattern**: Immutability - instead of mutating values, we increment
/// version numbers to track changes.
///
/// Each time an atom's value changes, its epoch increments. Dependencies
/// store the epoch number they were computed with, enabling smart cache
/// invalidation.
pub type EpochNumber = u64;

/// Getter trait for reading atom values
///
/// Reference: `jotai/src/vanilla/atom.ts:3`
///
/// **FP Pattern**: Reader monad - provides read-only access to state
///
/// The Getter is passed to atom read functions, allowing them to access
/// other atom values and automatically register dependencies.
///
/// TODO: Implement dependency tracking during get() calls
/// TODO: Handle type erasure for heterogeneous atom types
/// TODO: Add error handling for missing/uninitialized atoms
pub trait Getter: Send + Sync {
    /// Read the current value of an atom
    ///
    /// This function:
    /// 1. Looks up the atom's current state in the store
    /// 2. If not computed, evaluates the atom's read function
    /// 3. Registers a dependency relationship
    /// 4. Returns the cached value
    ///
    /// # Type Safety
    ///
    /// The `T: 'static` bound ensures we can use type erasure safely.
    ///
    /// TODO: Add caching based on epoch numbers
    /// TODO: Implement lazy evaluation
    /// TODO: Track dependencies for invalidation
    fn get<T: Clone + Send + Sync + 'static>(&self, atom: &Atom<T>) -> Result<T>;
}

/// Setter trait for writing atom values
///
/// Reference: `jotai/src/vanilla/atom.ts:5-8`
///
/// **FP Pattern**: State monad - provides write access to state
///
/// The Setter is passed to atom write functions, allowing them to update
/// the values of atoms (including other atoms).
///
/// TODO: Implement invalidation of dependent atoms on set
/// TODO: Increment epoch numbers when values change
/// TODO: Collect changed atoms for notification
pub trait Setter: Send + Sync {
    /// Update the value of an atom
    ///
    /// This function:
    /// 1. Updates the atom's value in the store
    /// 2. Increments the atom's epoch number
    /// 3. Marks all dependent atoms as invalidated
    /// 4. Collects the atom for listener notification
    ///
    /// TODO: Support SetStateAction pattern (value or updater function)
    /// TODO: Handle async/promise values
    /// TODO: Trigger cascading updates
    fn set<T: Clone + Send + Sync + 'static>(&self, atom: &Atom<T>, value: T) -> Result<()>;
}

// TODO: Add set_state_action method in future phase
// fn set_state_action<T, F>(&self, atom: &Atom<T>, action: SetStateAction<T, F>) -> Result<()>
// where
//     F: FnOnce(T) -> T;

/// Action that can either be a direct value or an updater function
///
/// Reference: `jotai/src/vanilla/atom.ts:65`
///
/// **FP Pattern**: Algebraic data type (enum with variants)
///
/// This matches Jotai's `SetStateAction<Value> = Value | ((prev: Value) => Value)`
///
/// TODO: Phase 1.4 - Implement in atom write handling
#[derive(Clone)]
pub enum SetStateAction<T, F>
where
    F: FnOnce(T) -> T,
{
    /// Direct value to set
    Value(T),
    /// Updater function that receives current value and returns new value
    ///
    /// **FP Pattern**: Higher-order function (function as data)
    Updater(F),
}

/// Type alias for read functions
///
/// Reference: `jotai/src/vanilla/atom.ts:17-20`
///
/// **FP Pattern**: First-class functions, pure functions
///
/// Read functions should be pure - given the same dependencies,
/// they should always return the same result.
///
/// Note: We can't use `&dyn Getter` because Getter has generic methods.
/// Instead, we'll pass a concrete type that implements read operations.
/// For now, we use a placeholder that will be resolved during implementation.
///
/// TODO: Phase 1.3 - Decide on final type (likely &Store or similar)
/// TODO: Add AbortSignal support for async operations
/// TODO: Add SetSelf parameter for writable atoms
pub type ReadFn<T> = Arc<dyn Fn() -> Result<T> + Send + Sync>;

/// Type alias for write functions
///
/// Reference: `jotai/src/vanilla/atom.ts:22-26`
///
/// **FP Pattern**: Higher-order functions, state transformations
///
/// Write functions receive both getter and setter capabilities, allowing them to:
/// 1. Read current state
/// 2. Update multiple atoms
/// 3. Perform complex state transformations
///
/// Note: We can't use `&dyn Getter/Setter` due to generic methods.
/// The actual implementation will pass the Store reference.
///
/// TODO: Phase 1.4 - Finalize signature with proper getter/setter access
/// TODO: Support generic Args tuple for different write signatures
pub type WriteFn<T> = Arc<dyn Fn(T) -> Result<()> + Send + Sync>;

/// Cleanup function returned by onMount callbacks
///
/// Reference: `jotai/src/vanilla/atom.ts:34`
///
/// **FP Pattern**: Closures for cleanup
///
/// Note: Using Fn instead of FnOnce for now to satisfy Sync requirement
/// TODO: Phase 8.1 - Implement lifecycle management with proper once semantics
pub type OnUnmount = Box<dyn Fn() + Send + Sync>;

/// Listener callback for subscriptions
///
/// Reference: `jotai/src/vanilla/internals.ts` (listeners in Mounted)
///
/// **FP Pattern**: Observer pattern, callbacks
///
/// Listeners are called when an atom's value changes.
/// They should not accept parameters - they should call store.get()
/// to read the new value if needed.
///
/// TODO: Phase 3 - Implement subscription system
pub type Listener = Box<dyn Fn() + Send + Sync>;

/// Unsubscribe function returned by store.sub()
///
/// Reference: `jotai/src/vanilla/internals.ts` (return value of storeSub)
///
/// **FP Pattern**: Higher-order function returns cleanup function
///
/// Calling this function removes the listener and potentially unmounts the atom.
///
/// Note: Using Fn instead of FnOnce for now to satisfy Sync requirement
/// TODO: Phase 3.2 - Implement in store.sub() with proper once semantics
pub type Unsubscribe = Box<dyn Fn() + Send + Sync>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_state_action_value() {
        let action: SetStateAction<i32, fn(i32) -> i32> = SetStateAction::Value(42);
        match action {
            SetStateAction::Value(v) => assert_eq!(v, 42),
            SetStateAction::Updater(_) => panic!("Expected Value"),
        }
    }

    #[test]
    fn test_set_state_action_updater() {
        let action: SetStateAction<i32, fn(i32) -> i32> = SetStateAction::Updater(|x| x + 1);
        match action {
            SetStateAction::Value(_) => panic!("Expected Updater"),
            SetStateAction::Updater(f) => assert_eq!(f(41), 42),
        }
    }

    // TODO: Add tests for Getter and Setter traits once implemented
}
