//! Basic atom tests - Phase 1
//!
//! These tests cover the fundamental atom operations:
//! - Creating primitive atoms
//! - Reading atom values
//! - Writing atom values
//! - Basic store operations

use jotai_rs::{atom, Store};

// ============================================================================
// PHASE 1.1: Atom Creation Tests
// ============================================================================

#[test]
#[ignore = "Phase 1.1 - Implement atom creation"]
fn test_create_primitive_atom() {
    // TODO: Phase 1.1 - Basic atom creation
    // Reference: `jotai/tests/vanilla/atom.test.tsx` line 7

    let count = atom(0);
    assert!(count.id() > 0, "Atom should have a positive ID");
}

#[test]
#[ignore = "Phase 1.1 - Implement atom with label"]
fn test_atom_with_label() {
    // TODO: Phase 1.1 - Atom debug labels

    let count = atom(0).with_label("counter");
    assert_eq!(count.as_atom().debug_label(), Some("counter"));
    assert!(count.as_atom().to_string().contains("counter"));
}

#[test]
#[ignore = "Phase 1.1 - Test atom ID uniqueness"]
fn test_atom_ids_are_unique() {
    // TODO: Phase 1.1 - Verify each atom gets unique ID

    let a1 = atom(1);
    let a2 = atom(2);
    let a3 = atom(3);

    assert_ne!(a1.id(), a2.id());
    assert_ne!(a2.id(), a3.id());
    assert_ne!(a1.id(), a3.id());
}

// ============================================================================
// PHASE 1.2: Store Creation Tests
// ============================================================================

#[test]
fn test_store_creation() {
    // TODO: Phase 1.2 - Store initialization

    let _store = Store::new();
    // Store should be empty on creation
    // Note: atom_states is private, so we can't check length directly
    // The store is created successfully if this doesn't panic
}

// ============================================================================
// PHASE 1.3: Basic Read Operations
// ============================================================================

#[test]
#[ignore = "Phase 1.3 - Implement store.get()"]
fn test_read_primitive_atom() {
    // TODO: Phase 1.3 - Basic get operation
    // Reference: `jotai/tests/vanilla/basic.test.tsx` line 11

    let store = Store::new();
    let count = atom(0);

    let value = store.get(&count.as_atom()).expect("Should read atom value");
    assert_eq!(value, 0);
}

#[test]
#[ignore = "Phase 1.3 - Test reading multiple times"]
fn test_read_atom_multiple_times() {
    // TODO: Phase 1.3 - Verify caching works

    let store = Store::new();
    let count = atom(42);

    let v1 = store.get(&count.as_atom()).unwrap();
    let v2 = store.get(&count.as_atom()).unwrap();

    assert_eq!(v1, 42);
    assert_eq!(v2, 42);
}

#[test]
#[ignore = "Phase 1.3 - Test reading multiple atoms"]
fn test_read_multiple_atoms() {
    // TODO: Phase 1.3 - Multiple independent atoms

    let store = Store::new();
    let a = atom(1);
    let b = atom(2);
    let c = atom(3);

    assert_eq!(store.get(&a.as_atom()).unwrap(), 1);
    assert_eq!(store.get(&b.as_atom()).unwrap(), 2);
    assert_eq!(store.get(&c.as_atom()).unwrap(), 3);
}

// ============================================================================
// PHASE 1.4: Basic Write Operations
// ============================================================================

#[test]
#[ignore = "Phase 1.4 - Implement store.set()"]
fn test_write_primitive_atom() {
    // TODO: Phase 1.4 - Basic set operation
    // Reference: `jotai/tests/vanilla/basic.test.tsx` line 18

    let store = Store::new();
    let count = atom(0);

    store.set(&count, 5).expect("Should set atom value");

    let value = store.get(&count.as_atom()).unwrap();
    assert_eq!(value, 5);
}

#[test]
#[ignore = "Phase 1.4 - Test multiple writes"]
fn test_write_atom_multiple_times() {
    // TODO: Phase 1.4 - Sequential writes

    let store = Store::new();
    let count = atom(0);

    store.set(&count, 1).unwrap();
    assert_eq!(store.get(&count.as_atom()).unwrap(), 1);

    store.set(&count, 2).unwrap();
    assert_eq!(store.get(&count.as_atom()).unwrap(), 2);

    store.set(&count, 100).unwrap();
    assert_eq!(store.get(&count.as_atom()).unwrap(), 100);
}

#[test]
#[ignore = "Phase 1.4 - Test writing multiple atoms"]
fn test_write_multiple_independent_atoms() {
    // TODO: Phase 1.4 - Independent atoms don't interfere

    let store = Store::new();
    let a = atom(1);
    let b = atom(2);

    store.set(&a, 10).unwrap();
    store.set(&b, 20).unwrap();

    assert_eq!(store.get(&a.as_atom()).unwrap(), 10);
    assert_eq!(store.get(&b.as_atom()).unwrap(), 20);
}

// ============================================================================
// PHASE 1.4: SetStateAction Tests
// ============================================================================

#[test]
#[ignore = "Phase 1.4 - Implement SetStateAction"]
fn test_set_with_updater_function() {
    // TODO: Phase 1.4 - Support updater functions
    // Reference: `jotai/tests/vanilla/basic.test.tsx` line 35

    let store = Store::new();
    let count = atom(0);

    // Using a closure to update based on previous value
    // store.set_with_updater(&count, |prev| prev + 1).unwrap();
    // assert_eq!(store.get(&count.as_atom()).unwrap(), 1);
    //
    // store.set_with_updater(&count, |prev| prev * 2).unwrap();
    // assert_eq!(store.get(&count.as_atom()).unwrap(), 2);
}

// ============================================================================
// PHASE 1: Edge Cases and Error Handling
// ============================================================================

#[test]
#[ignore = "Phase 1.3 - Handle reading uninitialized atom"]
fn test_read_uninitialized_atom() {
    // TODO: Phase 1.3 - What happens when reading atom that was never set?
    // For primitive atoms, should return initial value

    let store = Store::new();
    let count = atom(42);

    // Should return initial value without explicit set
    assert_eq!(store.get(&count.as_atom()).unwrap(), 42);
}

// ============================================================================
// FP PATTERN DEMONSTRATIONS
// ============================================================================

#[test]
#[ignore = "Phase 1 - Demonstrate immutability"]
fn test_atom_immutability() {
    // TODO: Phase 1 - Atoms themselves are immutable
    // Only their values in the store change

    let count = atom(0);
    let id1 = count.id();

    let store1 = Store::new();
    let store2 = Store::new();

    store1.set(&count, 10).unwrap();
    store2.set(&count, 20).unwrap();

    // Same atom in different stores has different values
    assert_eq!(store1.get(&count.as_atom()).unwrap(), 10);
    assert_eq!(store2.get(&count.as_atom()).unwrap(), 20);

    // Atom ID hasn't changed
    assert_eq!(count.id(), id1);
}

#[test]
#[ignore = "Phase 1 - Demonstrate lazy evaluation"]
fn test_lazy_evaluation() {
    // TODO: Phase 1 - Atoms don't compute until accessed

    let store = Store::new();
    let expensive = atom(42); // Value not computed yet

    // Only when we call get() is the value initialized
    let value = store.get(&expensive.as_atom()).unwrap();
    assert_eq!(value, 42);
}
