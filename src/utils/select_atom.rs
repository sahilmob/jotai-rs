//! Select atom utility for efficient derived state
//!
//! Reference: `jotai/src/vanilla/utils/selectAtom.ts`
//!
//! SelectAtom creates a derived atom that selects a slice of another atom's value
//! and only updates when that slice changes (using an equality function).
//!
//! ## Functional Programming Patterns
//! - Function composition (selector function)
//! - Memoization (equality-based caching)
//! - Higher-order functions
//! - Pure functions (selectors should be pure)

use std::sync::Arc;
use crate::atom::{Atom, atom_derived};
use crate::types::Getter;
use crate::error::Result;

/// Create a derived atom that selects and memoizes a slice of another atom
///
/// Reference: `jotai/src/vanilla/utils/selectAtom.ts:18-57`
///
/// ```typescript
/// export function selectAtom<Value, Slice>(
///   anAtom: Atom<Value>,
///   selector: (v: Value, prevSlice?: Slice) => Slice,
///   equalityFn: (prevSlice: Slice, slice: Slice) => boolean = Object.is,
/// ): Atom<Slice>
/// ```
///
/// The selectAtom utility is extremely important for performance. It prevents
/// unnecessary recomputation by using an equality function to check if the
/// selected slice has actually changed.
///
/// **FP Pattern**: Function composition, memoization, pure functions
///
/// # Example
///
/// ```rust,ignore
/// use jotai_rs::{atom, select_atom, Store};
///
/// #[derive(Clone)]
/// struct User {
///     name: String,
///     email: String,
///     age: i32,
/// }
///
/// let store = Store::new();
/// let user_atom = atom(User {
///     name: "John".to_string(),
///     email: "john@example.com".to_string(),
///     age: 30,
/// });
///
/// // Only re-render when name changes, not email or age
/// let name_atom = select_atom(
///     user_atom,
///     |user: &User| user.name.clone(),
///     |a, b| a == b,
/// );
/// ```
///
/// TODO: Phase 7.2 - Implement select_atom
pub fn select_atom<T, S, F, E>(
    source_atom: Atom<T>,
    selector: F,
    equality_fn: E,
) -> Atom<S>
where
    T: Clone + Send + Sync + 'static,
    S: Clone + Send + Sync + 'static,
    F: Fn(&T) -> S + Send + Sync + 'static,
    E: Fn(&S, &S) -> bool + Send + Sync + 'static,
{
    // Reference: Implementation approach from selectAtom.ts
    //
    // The trick is to create a derived atom that:
    // 1. Reads its own previous value
    // 2. Reads the source atom
    // 3. Applies the selector
    // 4. Compares with previous using equality_fn
    // 5. Returns previous if equal, new if different
    //
    // This requires a self-referential atom, which is tricky.
    //
    // Jotai uses a hack: `derivedAtom.init = EMPTY`
    // to allow reading the atom before it's initialized.

    // TODO: Phase 7.2 - Implement with memoization
    // Challenges:
    // - Need to store previous value somehow
    // - Need self-reference in read function
    // - Need to use equality_fn for comparison

    todo!("select_atom - Phase 7.2")
}

/// Select atom with default Object.is equality
///
/// TODO: Phase 7.2 - Convenience wrapper
pub fn select_atom_default<T, S, F>(
    source_atom: Atom<T>,
    selector: F,
) -> Atom<S>
where
    T: Clone + Send + Sync + 'static,
    S: Clone + PartialEq + Send + Sync + 'static,
    F: Fn(&T) -> S + Send + Sync + 'static,
{
    select_atom(source_atom, selector, |a, b| a == b)
}

/// Memoization helper for select_atom
///
/// Reference: `jotai/src/vanilla/utils/selectAtom.ts:4-16`
///
/// ```typescript
/// const getCached = <T>(c: () => T, m: WeakMap<object, T>, k: object): T =>
///   (m.has(k) ? m : m.set(k, c())).get(k) as T
///
/// const cache1 = new WeakMap()
/// const memo3 = <T>(
///   create: () => T,
///   dep1: object,
///   dep2: object,
///   dep3: object,
/// ): T => {
///   const cache2 = getCached(() => new WeakMap(), cache1, dep1)
///   const cache3 = getCached(() => new WeakMap(), cache2, dep2)
///   return getCached(create, cache3, dep3)
/// }
/// ```
///
/// Jotai uses nested WeakMaps for multi-key memoization.
/// In Rust, we might use a different approach (e.g., Arc<Mutex<HashMap>>).
///
/// **FP Pattern**: Memoization with multiple keys
///
/// TODO: Phase 7.2 - Implement memoization helper if needed
struct MemoCache {
    // TODO: Design cache structure for Rust
    // Options:
    // 1. HashMap with tuple keys
    // 2. Nested HashMaps
    // 3. LRU cache
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atom::atom;
    use crate::store::Store;

    // TODO: Phase 7.2 - Add tests for select_atom
    //
    // #[test]
    // fn test_select_atom_basic() {
    //     let store = Store::new();
    //     let source = atom((1, 2));
    //     let first = select_atom(source, |(a, _)| *a, |x, y| x == y);
    //
    //     assert_eq!(store.get(&first).unwrap(), 1);
    // }
    //
    // #[test]
    // fn test_select_atom_memoization() {
    //     let store = Store::new();
    //     let source = atom((1, 2));
    //     let first = select_atom(source, |(a, _)| *a, |x, y| x == y);
    //
    //     // Change second element
    //     store.set(&source, (1, 3)).unwrap();
    //
    //     // First should not recompute (value didn't change)
    //     // TODO: How to verify recomputation didn't happen?
    // }
}
