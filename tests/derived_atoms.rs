//! Derived atom and dependency tracking tests - Phase 2
//!
//! These tests cover:
//! - Creating derived atoms
//! - Dependency tracking
//! - Automatic recomputation
//! - Epoch-based caching

use jotai_rs::{atom, atom_derived, Store};

// ============================================================================
// PHASE 2.2: Derived Atom Creation
// ============================================================================

#[test]
#[ignore = "Phase 2.2 - Implement derived atoms"]
fn test_simple_derived_atom() {
    // TODO: Phase 2.2 - Basic derived atom
    // Reference: `jotai/tests/vanilla/derived-atom.test.tsx` line 10

    let store = Store::new();
    let count = atom(3);

    let doubled = atom_derived(move |get| {
        let c = get(&count.as_atom())?;
        Ok(c * 2)
    });

    assert_eq!(store.get(&doubled).unwrap(), 6);
}

#[test]
#[ignore = "Phase 2.2 - Test derived atom updates"]
fn test_derived_atom_updates_with_dependency() {
    // TODO: Phase 2.2 - Derived atoms recompute when dependencies change

    let store = Store::new();
    let count = atom(3);
    let doubled = atom_derived(move |get| {
        let c = get(&count.as_atom())?;
        Ok(c * 2)
    });

    assert_eq!(store.get(&doubled).unwrap(), 6);

    store.set(&count, 5).unwrap();
    assert_eq!(store.get(&doubled).unwrap(), 10);

    store.set(&count, 0).unwrap();
    assert_eq!(store.get(&doubled).unwrap(), 0);
}

#[test]
#[ignore = "Phase 2.2 - Test chained derived atoms"]
fn test_chained_derived_atoms() {
    // TODO: Phase 2.2 - Derived atoms depending on other derived atoms
    // Reference: `jotai/tests/vanilla/derived-atom.test.tsx` line 38

    let store = Store::new();
    let count = atom(1);

    let doubled = atom_derived(move |get| {
        let c = get(&count.as_atom())?;
        Ok(c * 2)
    });

    let quadrupled = atom_derived(move |get| {
        let d = get(&doubled)?;
        Ok(d * 2)
    });

    assert_eq!(store.get(&quadrupled).unwrap(), 4);

    store.set(&count, 2).unwrap();
    assert_eq!(store.get(&doubled).unwrap(), 4);
    assert_eq!(store.get(&quadrupled).unwrap(), 8);
}

#[test]
#[ignore = "Phase 2.2 - Test diamond dependency"]
fn test_diamond_dependency_pattern() {
    // TODO: Phase 2.2 - Multiple paths to same atom
    //
    // Dependency graph:
    //     count
    //    /     \
    //  +1      +2
    //    \     /
    //     sum

    let store = Store::new();
    let count = atom(10);

    let plus_one = atom_derived(move |get| {
        let c = get(&count.as_atom())?;
        Ok(c + 1)
    });

    let plus_two = atom_derived(move |get| {
        let c = get(&count.as_atom())?;
        Ok(c + 2)
    });

    let sum = atom_derived(move |get| {
        let a = get(&plus_one)?;
        let b = get(&plus_two)?;
        Ok(a + b)
    });

    // 10 + 1 + 10 + 2 = 23
    assert_eq!(store.get(&sum).unwrap(), 23);

    store.set(&count, 5).unwrap();
    // 5 + 1 + 5 + 2 = 13
    assert_eq!(store.get(&sum).unwrap(), 13);
}

// ============================================================================
// PHASE 2.1: Dependency Tracking
// ============================================================================

#[test]
#[ignore = "Phase 2.1 - Verify dependencies are tracked"]
fn test_dependency_tracking() {
    // TODO: Phase 2.1 - Internal test to verify dependency tracking

    let store = Store::new();
    let a = atom(1);
    let b = atom(2);

    let sum = atom_derived(move |get| {
        let av = get(&a.as_atom())?;
        let bv = get(&b.as_atom())?;
        Ok(av + bv)
    });

    // Read the derived atom
    store.get(&sum).unwrap();

    // TODO: Check internal state
    // - sum's AtomState should have dependencies [a.id(), b.id()]
    // - a's Mounted should have dependents containing sum.id()
    // - b's Mounted should have dependents containing sum.id()
}

// ============================================================================
// PHASE 2.3: Invalidation
// ============================================================================

#[test]
#[ignore = "Phase 2.3 - Test invalidation propagation"]
fn test_invalidation_cascade() {
    // TODO: Phase 2.3 - Changing one atom invalidates all dependents

    let store = Store::new();
    let base = atom(1);
    let derived1 = atom_derived(move |get| {
        let v = get(&base.as_atom())?;
        Ok(v + 1)
    });
    let derived2 = atom_derived(move |get| {
        let v = get(&derived1)?;
        Ok(v + 1)
    });
    let derived3 = atom_derived(move |get| {
        let v = get(&derived2)?;
        Ok(v + 1)
    });

    // Chain: base (1) -> derived1 (2) -> derived2 (3) -> derived3 (4)
    assert_eq!(store.get(&derived3).unwrap(), 4);

    // Change base
    store.set(&base, 10).unwrap();

    // All derived atoms should recompute
    // base (10) -> derived1 (11) -> derived2 (12) -> derived3 (13)
    assert_eq!(store.get(&derived1).unwrap(), 11);
    assert_eq!(store.get(&derived2).unwrap(), 12);
    assert_eq!(store.get(&derived3).unwrap(), 13);
}

// ============================================================================
// PHASE 2.4: Epoch-Based Caching
// ============================================================================

#[test]
#[ignore = "Phase 2.4 - Test cache invalidation with epochs"]
fn test_epoch_based_caching() {
    // TODO: Phase 2.4 - Verify atoms use epoch numbers for cache validation

    let store = Store::new();
    let a = atom(1);
    let b = atom(2);
    let sum = atom_derived(move |get| {
        let av = get(&a.as_atom())?;
        let bv = get(&b.as_atom())?;
        Ok(av + bv)
    });

    // First read - computes
    assert_eq!(store.get(&sum).unwrap(), 3);

    // Second read - should use cache (no dependencies changed)
    assert_eq!(store.get(&sum).unwrap(), 3);

    // TODO: Verify internally that sum wasn't recomputed
    // (e.g., by checking epoch number)

    // Change dependency
    store.set(&a, 5).unwrap();

    // Should recompute because epoch changed
    assert_eq!(store.get(&sum).unwrap(), 7);
}

#[test]
#[ignore = "Phase 2.4 - Test selective recomputation"]
fn test_only_affected_atoms_recompute() {
    // TODO: Phase 2.4 - Only atoms depending on changed atoms recompute

    let store = Store::new();
    let a = atom(1);
    let b = atom(2);

    let a_plus_10 = atom_derived(move |get| {
        let v = get(&a.as_atom())?;
        Ok(v + 10)
    });

    let b_plus_10 = atom_derived(move |get| {
        let v = get(&b.as_atom())?;
        Ok(v + 10)
    });

    assert_eq!(store.get(&a_plus_10).unwrap(), 11);
    assert_eq!(store.get(&b_plus_10).unwrap(), 12);

    // Change only a
    store.set(&a, 5).unwrap();

    // a_plus_10 recomputes, b_plus_10 doesn't
    assert_eq!(store.get(&a_plus_10).unwrap(), 15);
    assert_eq!(store.get(&b_plus_10).unwrap(), 12); // Still cached

    // TODO: Verify b_plus_10 didn't recompute (check epoch)
}

// ============================================================================
// FP PATTERN DEMONSTRATIONS
// ============================================================================

#[test]
#[ignore = "Phase 2 - Demonstrate function composition"]
fn test_function_composition_pattern() {
    // TODO: Phase 2 - Derived atoms are function composition

    let store = Store::new();
    let x = atom(5);

    // f(x) = x + 1
    let f = atom_derived(move |get| {
        let v = get(&x.as_atom())?;
        Ok(v + 1)
    });

    // g(x) = x * 2
    let g = atom_derived(move |get| {
        let v = get(&f)?;
        Ok(v * 2)
    });

    // g(f(x)) = (x + 1) * 2
    assert_eq!(store.get(&g).unwrap(), 12); // (5 + 1) * 2 = 12
}

#[test]
#[ignore = "Phase 2 - Demonstrate pure functions"]
fn test_pure_functions_in_derivation() {
    // TODO: Phase 2 - Read functions should be pure

    let store = Store::new();
    let count = atom(5);

    // Pure: same inputs always produce same output
    let doubled = atom_derived(move |get| {
        let c = get(&count.as_atom())?;
        Ok(c * 2)
    });

    let v1 = store.get(&doubled).unwrap();
    let v2 = store.get(&doubled).unwrap();

    assert_eq!(v1, v2); // Always returns same value for same input
}

// ============================================================================
// PHASE 2: Edge Cases
// ============================================================================

#[test]
#[ignore = "Phase 2 - Handle unused dependencies"]
fn test_conditional_dependencies() {
    // TODO: Phase 2.4 - Dependencies can change between reads

    let store = Store::new();
    let use_a = atom(true);
    let a = atom(10);
    let b = atom(20);

    let conditional = atom_derived(move |get| {
        let should_use_a = get(&use_a.as_atom())?;
        if should_use_a {
            get(&a.as_atom())
        } else {
            get(&b.as_atom())
        }
    });

    assert_eq!(store.get(&conditional).unwrap(), 10);

    // Switch to using b
    store.set(&use_a, false).unwrap();
    assert_eq!(store.get(&conditional).unwrap(), 20);

    // Changing a shouldn't matter anymore
    store.set(&a, 999).unwrap();
    assert_eq!(store.get(&conditional).unwrap(), 20); // Still 20, not affected
}
