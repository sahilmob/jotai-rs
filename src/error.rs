//! Error types for jotai-rs
//!
//! Reference: `jotai/src/vanilla/internals.ts` (error handling in readAtomState)
//!
//! Jotai stores errors in AtomState when atom read functions throw.
//! This module defines the error types for Rust.
//!
//! ## Functional Programming Patterns
//! - Result/Either type for error handling (vs exceptions)
//! - Explicit error types for better type safety

use thiserror::Error;
use std::any::type_name;

/// Main error type for jotai-rs operations
///
/// **FP Pattern**: Algebraic data type for error representation
///
/// Reference: Jotai stores errors as `e?: AnyError` in AtomState
#[derive(Error, Debug, Clone)]
pub enum AtomError {
    /// Atom was accessed before being initialized
    ///
    /// TODO: Phase 1.3 - Detect and handle uninitialized atoms
    #[error("Atom {atom_id} has not been initialized")]
    Uninitialized {
        atom_id: usize,
    },

    /// Type mismatch when reading atom value
    ///
    /// This can occur due to type erasure with `Box<dyn Any>`.
    ///
    /// TODO: Phase 1.3 - Add runtime type checking
    #[error("Type mismatch for atom {atom_id}: expected {expected}, got {actual}")]
    TypeMismatch {
        atom_id: usize,
        expected: &'static str,
        actual: String,
    },

    /// Circular dependency detected
    ///
    /// Reference: `jotai/src/vanilla/internals.ts` (cycle detection in DFS)
    ///
    /// TODO: Phase 4.1 - Implement cycle detection in topological sort
    #[error("Circular dependency detected involving atom {atom_id}")]
    CircularDependency {
        atom_id: usize,
        dependency_chain: Vec<usize>,
    },

    /// Error occurred in atom read function
    ///
    /// TODO: Phase 8.3 - Catch and wrap errors from user read functions
    #[error("Error reading atom {atom_id}: {message}")]
    ReadError {
        atom_id: usize,
        message: String,
    },

    /// Error occurred in atom write function
    ///
    /// TODO: Phase 5.2 - Catch and wrap errors from user write functions
    #[error("Error writing atom {atom_id}: {message}")]
    WriteError {
        atom_id: usize,
        message: String,
    },

    /// Atom is not writable (no write function)
    ///
    /// TODO: Phase 1.4 - Check writability before attempting set
    #[error("Atom {atom_id} is read-only and cannot be written to")]
    NotWritable {
        atom_id: usize,
    },

    /// Promise/async operation failed
    ///
    /// TODO: Phase 6 - Handle async errors
    #[error("Async operation failed for atom {atom_id}: {message}")]
    AsyncError {
        atom_id: usize,
        message: String,
    },

    /// Promise/async operation was cancelled
    ///
    /// Reference: AbortSignal handling in Jotai
    ///
    /// TODO: Phase 6.2 - Implement cancellation with AbortSignal equivalent
    #[error("Async operation cancelled for atom {atom_id}")]
    Cancelled {
        atom_id: usize,
    },

    /// Store operation failed
    ///
    /// TODO: Add as needed for store-level errors
    #[error("Store operation failed: {message}")]
    StoreError {
        message: String,
    },

    /// Generic error wrapper
    ///
    /// Used to wrap other error types
    #[error("Error: {0}")]
    Generic(String),
}

/// Result type alias for jotai-rs operations
///
/// **FP Pattern**: Using Result instead of exceptions for explicit error handling
pub type Result<T> = std::result::Result<T, AtomError>;

impl AtomError {
    /// Create a type mismatch error with type information
    ///
    /// TODO: Phase 1.3 - Use in type casting operations
    pub fn type_mismatch<T: 'static>(atom_id: usize, actual_type: &str) -> Self {
        AtomError::TypeMismatch {
            atom_id,
            expected: type_name::<T>(),
            actual: actual_type.to_string(),
        }
    }

    /// Create a read error from any error type
    ///
    /// TODO: Phase 8.3 - Use to wrap errors in readAtomState
    pub fn read_error(atom_id: usize, error: impl std::fmt::Display) -> Self {
        AtomError::ReadError {
            atom_id,
            message: error.to_string(),
        }
    }

    /// Create a write error from any error type
    ///
    /// TODO: Phase 5.2 - Use to wrap errors in writeAtomState
    pub fn write_error(atom_id: usize, error: impl std::fmt::Display) -> Self {
        AtomError::WriteError {
            atom_id,
            message: error.to_string(),
        }
    }

    /// Create an async error from any error type
    ///
    /// TODO: Phase 6.3 - Use for promise rejection handling
    pub fn async_error(atom_id: usize, error: impl std::fmt::Display) -> Self {
        AtomError::AsyncError {
            atom_id,
            message: error.to_string(),
        }
    }
}

/// Helper trait to convert errors to AtomError
///
/// TODO: Implement for common error types as needed
pub trait IntoAtomError {
    fn into_atom_error(self, atom_id: usize) -> AtomError;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = AtomError::Uninitialized { atom_id: 1 };
        assert!(err.to_string().contains("Atom 1"));
        assert!(err.to_string().contains("not been initialized"));
    }

    #[test]
    fn test_type_mismatch() {
        let err = AtomError::type_mismatch::<i32>(2, "String");
        assert!(err.to_string().contains("i32"));
        assert!(err.to_string().contains("String"));
    }

    #[test]
    fn test_circular_dependency() {
        let err = AtomError::CircularDependency {
            atom_id: 3,
            dependency_chain: vec![1, 2, 3],
        };
        assert!(err.to_string().contains("Circular dependency"));
    }

    #[test]
    fn test_read_error() {
        let err = AtomError::read_error(4, "Something went wrong");
        assert!(err.to_string().contains("Error reading atom 4"));
        assert!(err.to_string().contains("Something went wrong"));
    }

    // TODO: Add more error tests as implementation progresses
}
