//! AZC Runtime Safety Library
//!
//! This module provides runtime safety checks for AZC programs.
//! All safety checks are designed to catch errors at runtime when
//! they cannot be caught at compile time.

use std::fmt;

/// Result type for AZC runtime operations
pub type AzcResult<T> = Result<T, AzcError>;

/// AZC runtime error type
#[derive(Debug, Clone)]
pub enum AzcError {
    /// Null pointer dereference
    NullPointer(String),
    /// Array index out of bounds
    IndexOutOfBounds { index: usize, len: usize },
    /// Division by zero
    DivisionByZero,
    /// Integer overflow
    IntegerOverflow(String),
    /// Invalid cast
    InvalidCast { from: String, to: String },
    /// Memory allocation failed
    AllocationFailed(String),
    /// Use after free
    UseAfterFree(String),
    /// Double free
    DoubleFree(String),
    /// Invalid operation
    InvalidOperation(String),
    /// Custom error
    Custom(String),
}

impl fmt::Display for AzcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AzcError::NullPointer(msg) => write!(f, "Null pointer: {}", msg),
            AzcError::IndexOutOfBounds { index, len } => {
                write!(f, "Index {} out of bounds (length {})", index, len)
            }
            AzcError::DivisionByZero => write!(f, "Division by zero"),
            AzcError::IntegerOverflow(msg) => write!(f, "Integer overflow: {}", msg),
            AzcError::InvalidCast { from, to } => {
                write!(f, "Invalid cast from {} to {}", from, to)
            }
            AzcError::AllocationFailed(msg) => write!(f, "Memory allocation failed: {}", msg),
            AzcError::UseAfterFree(msg) => write!(f, "Use after free: {}", msg),
            AzcError::DoubleFree(msg) => write!(f, "Double free: {}", msg),
            AzcError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            AzcError::Custom(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for AzcError {}

/// Safe array indexing with bounds checking
pub fn safe_index<T>(arr: &[T], index: usize) -> AzcResult<&T> {
    arr.get(index).ok_or(AzcError::IndexOutOfBounds {
        index,
        len: arr.len(),
    })
}

/// Safe array indexing with bounds checking (mutable)
pub fn safe_index_mut<T>(arr: &mut [T], index: usize) -> AzcResult<&mut T> {
    let len = arr.len();
    if index >= len {
        Err(AzcError::IndexOutOfBounds { index, len })
    } else {
        Ok(&mut arr[index])
    }
}

/// Safe division with zero check
pub fn safe_div<T: std::ops::Div<Output = T> + PartialEq + Default>(a: T, b: T) -> AzcResult<T> {
    if b == T::default() {
        Err(AzcError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

/// Safe integer addition with overflow check
pub fn safe_add_i32(a: i32, b: i32) -> AzcResult<i32> {
    a.checked_add(b)
        .ok_or_else(|| AzcError::IntegerOverflow(format!("{} + {}", a, b)))
}

/// Safe integer subtraction with overflow check
pub fn safe_sub_i32(a: i32, b: i32) -> AzcResult<i32> {
    a.checked_sub(b)
        .ok_or_else(|| AzcError::IntegerOverflow(format!("{} - {}", a, b)))
}

/// Safe integer multiplication with overflow check
pub fn safe_mul_i32(a: i32, b: i32) -> AzcResult<i32> {
    a.checked_mul(b)
        .ok_or_else(|| AzcError::IntegerOverflow(format!("{} * {}", a, b)))
}

/// Safe integer division with overflow check
pub fn safe_div_i32(a: i32, b: i32) -> AzcResult<i32> {
    if b == 0 {
        return Err(AzcError::DivisionByZero);
    }
    a.checked_div(b)
        .ok_or_else(|| AzcError::IntegerOverflow(format!("{} / {}", a, b)))
}

/// Safe pointer dereference
///
/// # Safety
/// The returned reference is valid for the lifetime of the pointer's data.
pub fn safe_deref<'a, T>(ptr: *const T) -> AzcResult<&'a T> {
    if ptr.is_null() {
        Err(AzcError::NullPointer("dereference".to_string()))
    } else {
        Ok(unsafe { &*ptr })
    }
}

/// Safe mutable pointer dereference
///
/// # Safety
/// The returned reference is valid for the lifetime of the pointer's data.
pub fn safe_deref_mut<'a, T>(ptr: *mut T) -> AzcResult<&'a mut T> {
    if ptr.is_null() {
        Err(AzcError::NullPointer("mutable dereference".to_string()))
    } else {
        Ok(unsafe { &mut *ptr })
    }
}


/// Runtime assertion that panics with a clear message
pub fn azc_assert(condition: bool, message: &str) {
    if !condition {
        panic!("AZC assertion failed: {}", message);
    }
}

/// Runtime precondition check
pub fn azc_precondition(condition: bool, message: &str) -> AzcResult<()> {
    if !condition {
        Err(AzcError::InvalidOperation(message.to_string()))
    } else {
        Ok(())
    }
}

/// Runtime postcondition check
pub fn azc_postcondition(condition: bool, message: &str) -> AzcResult<()> {
    if !condition {
        Err(AzcError::InvalidOperation(format!(
            "Postcondition failed: {}",
            message
        )))
    } else {
        Ok(())
    }
}

/// Safe memory allocation
pub fn safe_alloc<T>(value: T) -> AzcResult<Box<T>> {
    Ok(Box::new(value))
}

/// Safe memory deallocation (explicit drop)
pub fn safe_free<T>(_value: Box<T>) -> AzcResult<()> {
    // Box automatically drops when it goes out of scope
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_index() {
        let arr = [1, 2, 3];
        assert_eq!(*safe_index(&arr, 0).unwrap(), 1);
        assert!(safe_index(&arr, 3).is_err());
    }

    #[test]
    fn test_safe_div() {
        assert_eq!(safe_div(10, 2).unwrap(), 5);
        assert!(safe_div(10, 0).is_err());
    }

    #[test]
    fn test_safe_add_i32() {
        assert_eq!(safe_add_i32(1, 2).unwrap(), 3);
        assert!(safe_add_i32(i32::MAX, 1).is_err());
    }

    #[test]
    fn test_safe_mul_i32() {
        assert_eq!(safe_mul_i32(2, 3).unwrap(), 6);
        assert!(safe_mul_i32(i32::MAX, 2).is_err());
    }

    #[test]
    fn test_azc_precondition() {
        assert!(azc_precondition(true, "test").is_ok());
        assert!(azc_precondition(false, "test").is_err());
    }

    #[test]
    fn test_azc_assert() {
        azc_assert(true, "should not panic");
        // This would panic: azc_assert(false, "test");
    }
}
