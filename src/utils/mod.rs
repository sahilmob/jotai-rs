//! Utility atom factories and helpers
//!
//! Reference: `jotai/src/vanilla/utils/`
//!
//! This module provides higher-level atom utilities built on top of the
//! core atom primitives. These demonstrate common patterns for state management.
//!
//! ## Functional Programming Patterns
//! - Factory functions creating configured atoms
//! - Memoization for efficiency
//! - Higher-order functions (functions returning atoms)
//! - Composition patterns

pub mod atom_family;
pub mod select_atom;

// TODO: Phase 7 - Add more utility modules
// pub mod atom_with_reducer;
// pub mod atom_with_default;
// pub mod atom_with_storage;
// pub mod loadable;
// pub mod split_atom;
