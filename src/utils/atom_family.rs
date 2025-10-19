//! Atom family utility for creating parameterized atoms
//!
//! Reference: `jotai/src/vanilla/utils/atomFamily.ts`
//!
//! An atom family is a factory function that creates and caches atoms based
//! on parameters. It's useful for managing collections of similar state.
//!
//! ## Functional Programming Patterns
//! - Higher-order functions (returns a function)
//! - Memoization (caches created atoms)
//! - Closures (captures state in returned function)
//! - Factory pattern

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::hash::Hash;
use crate::atom::Atom;

/// Atom family function type
///
/// Reference: `jotai/src/vanilla/utils/atomFamily.ts:15-25`
///
/// ```typescript
/// export interface AtomFamily<Param, AtomType> {
///   (param: Param): AtomType
///   getParams(): Iterable<Param>
///   remove(param: Param): void
///   setShouldRemove(shouldRemove: ShouldRemove<Param> | null): void
/// }
/// ```
///
/// **FP Pattern**: Function with attached methods (closure with state)
///
/// TODO: Phase 7.1 - Implement atom family
pub struct AtomFamily<P, T>
where
    P: Clone + Eq + Hash + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
{
    /// The initialization function for creating atoms
    ///
    /// **FP Pattern**: Higher-order function stored as data
    initialize_atom: Arc<dyn Fn(P) -> Atom<T> + Send + Sync>,

    /// Cache of created atoms, keyed by parameter
    ///
    /// **FP Pattern**: Memoization with HashMap
    ///
    /// TODO: Phase 7.1 - Use for atom caching
    cache: Arc<Mutex<HashMap<P, (Atom<T>, i64)>>>,

    /// Optional custom equality function
    ///
    /// TODO: Phase 7.1 - Support custom equality
    are_equal: Option<Arc<dyn Fn(&P, &P) -> bool + Send + Sync>>,

    /// Optional function to determine if cached atoms should be removed
    ///
    /// Reference: `jotai/src/vanilla/utils/atomFamily.ts:7`
    ///
    /// ```typescript
    /// type ShouldRemove<Param> = (createdAt: CreatedAt, param: Param) => boolean
    /// ```
    ///
    /// TODO: Phase 7.1 - Support automatic cleanup
    should_remove: Arc<Mutex<Option<Arc<dyn Fn(i64, &P) -> bool + Send + Sync>>>>,
}

impl<P, T> AtomFamily<P, T>
where
    P: Clone + Eq + Hash + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
{
    /// Get or create an atom for the given parameter
    ///
    /// Reference: `jotai/src/vanilla/utils/atomFamily.ts:39-64`
    ///
    /// ```typescript
    /// const createAtom = (param: Param) => {
    ///   let item = atoms.get(param)
    ///   if (item !== undefined) {
    ///     if (shouldRemove?.(item[1], param)) {
    ///       createAtom.remove(param)
    ///     } else {
    ///       return item[0]
    ///     }
    ///   }
    ///   const newAtom = initializeAtom(param)
    ///   atoms.set(param, [newAtom, Date.now()])
    ///   return newAtom
    /// }
    /// ```
    ///
    /// **FP Pattern**: Memoization, lazy initialization
    ///
    /// TODO: Phase 7.1 - Implement with caching logic
    pub fn get(&self, param: P) -> Atom<T> {
        // TODO: Check cache for existing atom
        // TODO: If exists and not expired, return it
        // TODO: Otherwise, call initialize_atom
        // TODO: Cache the new atom with timestamp
        // TODO: Return the atom
        todo!("AtomFamily::get - Phase 7.1")
    }

    /// Get all parameters that have atoms created
    ///
    /// Reference: `jotai/src/vanilla/utils/atomFamily.ts:84`
    ///
    /// ```typescript
    /// createAtom.getParams = () => atoms.keys()
    /// ```
    ///
    /// TODO: Phase 7.1 - Return iterator over cached params
    pub fn get_params(&self) -> Vec<P> {
        // TODO: Get all keys from cache
        todo!("AtomFamily::get_params - Phase 7.1")
    }

    /// Remove an atom from the family
    ///
    /// Reference: `jotai/src/vanilla/utils/atomFamily.ts:86-101`
    ///
    /// ```typescript
    /// createAtom.remove = (param: Param) => {
    ///   if (!atoms.has(param)) return
    ///   const [atom] = atoms.get(param)!
    ///   atoms.delete(param)
    ///   notifyListeners('REMOVE', param, atom)
    /// }
    /// ```
    ///
    /// TODO: Phase 7.1 - Implement removal from cache
    pub fn remove(&self, param: &P) {
        // TODO: Remove from cache
        // TODO: Notify listeners if implemented
        todo!("AtomFamily::remove - Phase 7.1")
    }

    /// Set the function that determines if atoms should be auto-removed
    ///
    /// Reference: `jotai/src/vanilla/utils/atomFamily.ts:103-112`
    ///
    /// ```typescript
    /// createAtom.setShouldRemove = (fn: ShouldRemove<Param> | null) => {
    ///   shouldRemove = fn
    ///   if (!shouldRemove) return
    ///   for (const [key, [atom, createdAt]] of atoms) {
    ///     if (shouldRemove(createdAt, key)) {
    ///       atoms.delete(key)
    ///       notifyListeners('REMOVE', key, atom)
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// TODO: Phase 7.1 - Implement with automatic cleanup
    pub fn set_should_remove<F>(&self, should_remove: Option<F>)
    where
        F: Fn(i64, &P) -> bool + Send + Sync + 'static,
    {
        // TODO: Store the should_remove function
        // TODO: Immediately run cleanup on existing atoms
        todo!("AtomFamily::set_should_remove - Phase 7.1")
    }
}

/// Create an atom family
///
/// Reference: `jotai/src/vanilla/utils/atomFamily.ts:27-114`
///
/// ```typescript
/// export function atomFamily<Param, AtomType>(
///   initializeAtom: (param: Param) => AtomType,
///   areEqual?: (a: Param, b: Param) => boolean,
/// ): AtomFamily<Param, AtomType>
/// ```
///
/// **FP Pattern**: Higher-order function, factory pattern, closure
///
/// # Example
///
/// ```rust,ignore
/// use jotai_rs::{atom, atom_family};
///
/// // Create a family of counter atoms
/// let counter_family = atom_family(|id: i32| {
///     atom(0).with_label(format!("counter-{}", id))
/// });
///
/// // Get atoms for different IDs
/// let counter1 = counter_family.get(1);
/// let counter2 = counter_family.get(2);
/// let counter1_again = counter_family.get(1); // Returns cached atom
/// ```
///
/// TODO: Phase 7.1 - Implement atom_family
pub fn atom_family<P, T, F>(initialize_atom: F) -> AtomFamily<P, T>
where
    P: Clone + Eq + Hash + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
    F: Fn(P) -> Atom<T> + Send + Sync + 'static,
{
    // TODO: Create AtomFamily with:
    // - initialize_atom function
    // - Empty cache
    // - No custom equality
    // - No should_remove
    todo!("atom_family - Phase 7.1")
}

/// Create an atom family with custom equality
///
/// TODO: Phase 7.1 - Support custom equality for complex parameter types
pub fn atom_family_with_equality<P, T, F, E>(
    initialize_atom: F,
    are_equal: E,
) -> AtomFamily<P, T>
where
    P: Clone + Eq + Hash + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
    F: Fn(P) -> Atom<T> + Send + Sync + 'static,
    E: Fn(&P, &P) -> bool + Send + Sync + 'static,
{
    // TODO: Similar to atom_family but with custom equality
    todo!("atom_family_with_equality - Phase 7.1")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atom::atom;

    // TODO: Phase 7.1 - Add tests for atom family
    //
    // #[test]
    // fn test_atom_family_caching() {
    //     let family = atom_family(|id: i32| atom(id * 10));
    //     let a1 = family.get(1);
    //     let a2 = family.get(1);
    //     assert_eq!(a1.id(), a2.id()); // Same atom returned
    // }
    //
    // #[test]
    // fn test_atom_family_different_params() {
    //     let family = atom_family(|id: i32| atom(id));
    //     let a1 = family.get(1);
    //     let a2 = family.get(2);
    //     assert_ne!(a1.id(), a2.id()); // Different atoms
    // }
}
