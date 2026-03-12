//! AZC Async Types
//!
//! Implements async/await types for asynchronous programming.

use std::fmt;

use super::Type;

/// Future type representing an asynchronous computation
#[derive(Debug, Clone, PartialEq)]
pub struct FutureType {
    /// The type that will be produced when the future completes
    pub output: Box<Type>,
}

impl FutureType {
    pub fn new(output: Type) -> Self {
        FutureType {
            output: Box::new(output),
        }
    }

    /// Get the output type
    pub fn output_type(&self) -> &Type {
        &self.output
    }
}

impl fmt::Display for FutureType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Future<{}>", self.output)
    }
}

/// Async function type
#[derive(Debug, Clone, PartialEq)]
pub struct AsyncFunctionType {
    /// Parameter types
    pub params: Vec<Type>,
    /// Return type (wrapped in Future)
    pub return_type: Box<Type>,
}

impl AsyncFunctionType {
    pub fn new(params: Vec<Type>, return_type: Type) -> Self {
        AsyncFunctionType {
            params,
            return_type: Box::new(return_type),
        }
    }
}

impl fmt::Display for AsyncFunctionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params: Vec<String> = self.params.iter().map(|t| format!("{}", t)).collect();
        write!(
            f,
            "async fn({}) -> Future<{}>",
            params.join(", "),
            self.return_type
        )
    }
}

/// Poll state for async execution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Poll<T> {
    /// The future is ready with a value
    Ready(T),
    /// The future is not ready yet
    Pending,
}

impl<T> Poll<T> {
    pub fn is_ready(&self) -> bool {
        matches!(self, Poll::Ready(_))
    }

    pub fn is_pending(&self) -> bool {
        matches!(self, Poll::Pending)
    }

    pub fn unwrap(self) -> T {
        match self {
            Poll::Ready(v) => v,
            Poll::Pending => panic!("called unwrap on Pending"),
        }
    }
}

/// Waker for async runtime
#[derive(Debug, Clone)]
pub struct Waker {
    /// Unique identifier for the waker
    pub id: usize,
}

impl Waker {
    pub fn new(id: usize) -> Self {
        Waker { id }
    }

    pub fn wake(&self) {
        // In a real implementation, this would wake the task
    }
}

/// Context for async polling
#[derive(Debug, Clone)]
pub struct AsyncContext {
    pub waker: Waker,
}

impl AsyncContext {
    pub fn new(waker: Waker) -> Self {
        AsyncContext { waker }
    }

    pub fn from_waker(waker: Waker) -> Self {
        Self::new(waker)
    }

    pub fn waker(&self) -> &Waker {
        &self.waker
    }
}

/// Join handle for concurrent async operations
#[derive(Debug, Clone, PartialEq)]
pub struct JoinHandle<T> {
    pub inner: T,
}

impl<T> JoinHandle<T> {
    pub fn new(inner: T) -> Self {
        JoinHandle { inner }
    }
}

/// Async runtime configuration
#[derive(Debug, Clone)]
pub struct AsyncRuntime {
    /// Number of worker threads
    pub worker_threads: usize,
    /// Maximum blocking threads
    pub max_blocking_threads: usize,
}

impl Default for AsyncRuntime {
    fn default() -> Self {
        AsyncRuntime {
            worker_threads: 4,
            max_blocking_threads: 512,
        }
    }
}

impl AsyncRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_workers(mut self, n: usize) -> Self {
        self.worker_threads = n;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::super::IntSize;
    use super::*;

    #[test]
    fn test_future_type() {
        let future = FutureType::new(Type::Int(IntSize::I32));
        assert_eq!(future.output_type(), &Type::Int(IntSize::I32));
    }

    #[test]
    fn test_future_display() {
        let future = FutureType::new(Type::Int(IntSize::I32));
        assert_eq!(format!("{}", future), "Future<i32>");
    }

    #[test]
    fn test_async_function_type() {
        let async_fn = AsyncFunctionType::new(vec![Type::Int(IntSize::I32)], Type::Bool);
        assert_eq!(async_fn.params.len(), 1);
    }

    #[test]
    fn test_async_function_display() {
        let async_fn = AsyncFunctionType::new(vec![Type::Int(IntSize::I32)], Type::Bool);
        assert_eq!(format!("{}", async_fn), "async fn(i32) -> Future<Bool>");
    }

    #[test]
    fn test_poll_ready() {
        let poll: Poll<i32> = Poll::Ready(42);
        assert!(poll.is_ready());
        assert!(!poll.is_pending());
        assert_eq!(poll.unwrap(), 42);
    }

    #[test]
    fn test_poll_pending() {
        let poll: Poll<i32> = Poll::Pending;
        assert!(poll.is_pending());
        assert!(!poll.is_ready());
    }

    #[test]
    fn test_waker() {
        let waker = Waker::new(1);
        assert_eq!(waker.id, 1);
    }

    #[test]
    fn test_async_context() {
        let waker = Waker::new(1);
        let ctx = AsyncContext::new(waker);
        assert_eq!(ctx.waker().id, 1);
    }

    #[test]
    fn test_async_runtime() {
        let runtime = AsyncRuntime::new();
        assert_eq!(runtime.worker_threads, 4);

        let runtime = runtime.with_workers(8);
        assert_eq!(runtime.worker_threads, 8);
    }
}
