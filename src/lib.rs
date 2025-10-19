//! # Jotai-RS: A Rust Implementation of Jotai State Management
//!
//! This library is a learning project that rebuilds Jotai's core state management
//! primitives in Rust. It focuses on understanding the internal architecture through
//! implementation.
//!
//! **Reference**: This implementation is based on the TypeScript Jotai library
//! available in the `jotai/` submodule.
//!
//! ## Core Concepts
//!
//! - **Atoms**: Immutable configuration objects representing pieces of state
//! - **Store**: Runtime state container managing atom values and dependencies
//! - **Dependency Tracking**: Automatic tracking of relationships between atoms
//! - **Reactivity**: Subscription system for reactive updates
//!
//! ## Functional Programming Patterns
//!
//! This library demonstrates extensive use of FP patterns:
//! - Higher-order functions
//! - Closures and lexical scoping
//! - Function composition
//! - Immutability
//! - Pure functions
//! - Lazy evaluation
//! - Memoization
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use jotai_rs::{atom, Store};
//!
//! // Create a store
//! let store = Store::new();
//!
//! // Create primitive atoms
//! let count = atom(0);
//!
//! // Read value
//! assert_eq!(store.get(&count), 0);
//!
//! // Write value
//! store.set(&count, 5);
//! assert_eq!(store.get(&count), 5);
//!
//! // Create derived atom
//! let double = atom(|get| get(&count) * 2);
//! assert_eq!(store.get(&double), 10);
//!
//! // Subscribe to changes
//! let unsub = store.sub(&count, || {
//!     println!("Count changed!");
//! });
//! ```

// Public modules
pub mod atom;
pub mod store;
pub mod types;
pub mod error;
pub mod utils;

// Internal implementation (not public API)
mod internals;

// Re-export commonly used types
pub use atom::{Atom, PrimitiveAtom, WritableAtom, atom};
pub use store::Store;
pub use types::{AtomId, EpochNumber, Getter, Setter};
pub use error::{AtomError, Result};

// Re-export utility functions
pub use utils::{
    atom_family::atom_family,
    select_atom::select_atom,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_import() {
        // Ensure basic types can be imported
        let _store = Store::new();
    }
}
