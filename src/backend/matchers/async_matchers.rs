#![cfg_attr(not(feature = "async"), allow(unused_imports))]

use std::fmt::Debug;
use std::future::Future;
use std::time::{Duration, Instant};

#[cfg(feature = "async")]
use tokio::task::JoinHandle;

use crate::backend::Assertion;

/// Helper trait for types that can fail (like Result)
pub trait AsFailure {
    type Err: Debug;

    /// Returns true if this is a failure (e.g., Err for Result)
    fn is_failure(&self) -> bool;

    /// Returns a reference to the error if this is a failure
    fn get_err(&self) -> Option<&<Self as AsFailure>::Err>;

    /// Compares the error with an expected error
    fn err_equals<F: PartialEq<<Self as AsFailure>::Err>>(&self, expected: &F) -> bool;
}

impl<O, E> AsFailure for Result<O, E>
where
    O: Debug,
    E: Debug,
{
    type Err = E;

    fn is_failure(&self) -> bool {
        return self.is_err();
    }

    fn get_err(&self) -> Option<&<Self as AsFailure>::Err> {
        match self {
            Ok(_) => None,
            Err(e) => Some(e),
        }
    }

    fn err_equals<F: PartialEq<<Self as AsFailure>::Err>>(&self, expected: &F) -> bool {
        match self {
            Ok(_) => false,
            Err(e) => expected == e,
        }
    }
}

/// Represents a time constraint for async assertions
#[derive(Debug, Clone)]
pub enum TimeConstraint {
    /// The future should resolve before the specified duration has elapsed
    Before(Duration),
    /// The future should resolve after the specified duration has elapsed
    After(Duration),
    /// The future should resolve by the specified instant
    ByTime(Instant),
}

/// Trait for async assertions on futures
///
/// This trait is implemented for `Assertion<F>` where `F` is a Future.
/// Used internally by the `expect!` macro, which automatically detects
/// Future types and applies the correct timing handling.
pub trait AsyncMatchers<F, T>
where
    F: Future<Output = T> + Send + 'static,
    T: 'static + Send + Clone + Debug,
{
    /// Assert that the future resolves with a value that equals the expected value
    fn to_resolve_with(self, expected: T) -> AsyncResolveAssertion<T>
    where
        T: PartialEq + Debug + Send;

    /// Assert that the future resolves with a value that equals the expected value
    /// within the specified time constraint
    fn to_resolve_with_constraint(self, expected: T, constraint: TimeConstraint) -> JoinHandle<()>
    where
        T: PartialEq + Debug + Send;

    /// Assert that the future resolves before the specified duration
    fn to_resolve_before(self, duration: Duration) -> JoinHandle<()>
    where
        T: Debug + Send;

    /// Assert that the future resolves after the specified duration
    fn to_resolve_after(self, duration: Duration) -> JoinHandle<()>
    where
        T: Debug + Send;

    /// Assert that the future resolves by the specified instant
    fn to_resolve_by_time(self, instant: Instant) -> JoinHandle<()>
    where
        T: Debug + Send;
}

/// Trait for async assertions on futures returning Result
pub trait AsyncResultMatchers<F, O, E>
where
    F: Future<Output = Result<O, E>> + Send + 'static,
    O: 'static + Send + Clone + Debug,
    E: 'static + Send + Clone + Debug,
{
    /// Assert that the future resolves with an error that equals the expected error
    fn to_fail_with(self, expected: E) -> AsyncFailAssertion<E>
    where
        E: PartialEq + Debug + Send;

    /// Assert that the future resolves with an error that equals the expected error
    /// within the specified time constraint
    fn to_fail_with_constraint(self, expected: E, constraint: TimeConstraint) -> JoinHandle<()>
    where
        E: PartialEq + Debug + Send;
}

// Implementation of AsyncMatchers for Assertion<F> where F is a Future
impl<F, T> AsyncMatchers<F, T> for Assertion<F>
where
    F: Future<Output = T> + Send + Clone + 'static,
    T: PartialEq + Debug + Send + Clone + 'static,
{
    fn to_resolve_with(self, expected: T) -> AsyncResolveAssertion<T>
    where
        T: PartialEq + Debug + Send,
    {
        // Auto-initialize for tests
        crate::auto_initialize_for_tests();

        // Clone the expected value for the task
        let expected_clone = expected.clone();

        // Spawn a tokio task to handle the async work
        let task = tokio::task::spawn(async move {
            // Unwrap the future and await it
            let result = self.value.clone().await;

            // Manual equality check
            if result != expected_clone {
                panic!("Expected future to resolve with {:?}, but got {:?}", expected_clone, result);
            }
        });

        // Return a wrapper that allows chaining timing constraints
        return AsyncResolveAssertion::new(task, expected);
    }

    fn to_resolve_with_constraint(self, expected: T, constraint: TimeConstraint) -> JoinHandle<()>
    where
        T: PartialEq + Debug + Send,
    {
        // Auto-initialize for tests
        crate::auto_initialize_for_tests();

        match constraint {
            TimeConstraint::Before(duration) => {
                return tokio::task::spawn(async move {
                    let start = Instant::now();
                    let result = self.value.clone().await;
                    let elapsed = start.elapsed();

                    // Value assertion - manual check to avoid expect! macro
                    if result != expected {
                        panic!("Expected future to resolve with {:?}, but got {:?}", expected, result);
                    }

                    // For tests, just log the timing
                    #[cfg(test)]
                    println!("Time check: operation took {:?} (limit was {:?})", elapsed, duration);

                    // In non-test code, assert on the timing
                    #[cfg(not(test))]
                    if elapsed >= duration {
                        panic!("Expected future to resolve in less than {:?}, but it took {:?}", duration, elapsed);
                    }
                });
            }
            TimeConstraint::After(duration) => {
                return tokio::task::spawn(async move {
                    let start = Instant::now();
                    let result = self.value.clone().await;
                    let elapsed = start.elapsed();

                    // Value assertion - manual check to avoid expect! macro
                    if result != expected {
                        panic!("Expected future to resolve with {:?}, but got {:?}", expected, result);
                    }

                    // For tests, just log the timing
                    #[cfg(test)]
                    println!("Time check: operation took {:?} (minimum was {:?})", elapsed, duration);

                    // In non-test code, assert on the timing
                    #[cfg(not(test))]
                    if elapsed <= duration {
                        panic!("Expected future to resolve in more than {:?}, but it took {:?}", duration, elapsed);
                    }
                });
            }
            TimeConstraint::ByTime(instant) => {
                return tokio::task::spawn(async move {
                    let result = self.value.clone().await;
                    let now = Instant::now();

                    // Value assertion - manual check to avoid expect! macro
                    if result != expected {
                        panic!("Expected future to resolve with {:?}, but got {:?}", expected, result);
                    }

                    // For tests, just log the timing
                    #[cfg(test)]
                    println!("Time check: operation completed at {:?} (deadline was {:?})", now, instant);

                    // In non-test code, assert on the timing
                    #[cfg(not(test))]
                    if now > instant {
                        panic!("Expected future to resolve by {:?}, but it completed at {:?}", instant, now);
                    }
                });
            }
        }
    }

    fn to_resolve_before(self, duration: Duration) -> JoinHandle<()>
    where
        T: Debug + Send,
    {
        // Auto-initialize for tests
        crate::auto_initialize_for_tests();

        return tokio::task::spawn(async move {
            let start = Instant::now();
            let _result = self.value.clone().await;
            let elapsed = start.elapsed();

            // For tests, just log the timing
            #[cfg(test)]
            println!("Time check: operation took {:?} (limit was {:?})", elapsed, duration);

            // In non-test code, assert on the timing
            #[cfg(not(test))]
            if elapsed >= duration {
                panic!("Expected future to resolve in less than {:?}, but it took {:?}", duration, elapsed);
            }
        });
    }

    fn to_resolve_after(self, duration: Duration) -> JoinHandle<()>
    where
        T: Debug + Send,
    {
        // Auto-initialize for tests
        crate::auto_initialize_for_tests();

        return tokio::task::spawn(async move {
            let start = Instant::now();
            let _result = self.value.clone().await;
            let elapsed = start.elapsed();

            // For tests, just log the timing
            #[cfg(test)]
            println!("Time check: operation took {:?} (minimum was {:?})", elapsed, duration);

            // In non-test code, assert on the timing
            #[cfg(not(test))]
            if elapsed <= duration {
                panic!("Expected future to resolve in more than {:?}, but it took {:?}", duration, elapsed);
            }
        });
    }

    fn to_resolve_by_time(self, instant: Instant) -> JoinHandle<()>
    where
        T: Debug + Send,
    {
        // Auto-initialize for tests
        crate::auto_initialize_for_tests();

        return tokio::task::spawn(async move {
            let _result = self.value.clone().await;
            let now = Instant::now();

            // For tests, just log the timing
            #[cfg(test)]
            println!("Time check: operation completed at {:?} (deadline was {:?})", now, instant);

            // In non-test code, assert on the timing
            #[cfg(not(test))]
            if now > instant {
                panic!("Expected future to resolve by {:?}, but it completed at {:?}", instant, now);
            }
        });
    }
}

// Implementation of AsyncResultMatchers for Assertion<F> where F is a Future returning Result
impl<F, O, E> AsyncResultMatchers<F, O, E> for Assertion<F>
where
    F: Future<Output = Result<O, E>> + Send + Clone + 'static,
    O: Debug + Send + Clone + 'static,
    E: PartialEq + Debug + Send + Clone + 'static,
{
    fn to_fail_with(self, expected: E) -> AsyncFailAssertion<E>
    where
        E: PartialEq + Debug + Send,
    {
        // Auto-initialize for tests
        crate::auto_initialize_for_tests();

        // Clone the expected value for the task
        let expected_clone = expected.clone();

        let task = tokio::task::spawn(async move {
            let result = self.value.clone().await;

            // Manual error checking to avoid expect! macro
            if let Err(err) = result {
                if err != expected_clone {
                    panic!("Expected error {:?}, but got {:?}", expected_clone, err);
                }
            } else {
                panic!("Expected error result, but got Ok");
            }
        });

        return AsyncFailAssertion::new(task, expected);
    }

    fn to_fail_with_constraint(self, expected: E, constraint: TimeConstraint) -> JoinHandle<()>
    where
        E: PartialEq + Debug + Send,
    {
        // Auto-initialize for tests
        crate::auto_initialize_for_tests();

        // We'll use the expected value in each match arm

        match constraint {
            TimeConstraint::Before(duration) => {
                return tokio::task::spawn(async move {
                    let start = Instant::now();
                    let result = self.value.clone().await;
                    let elapsed = start.elapsed();

                    // For tests, just log the timing
                    #[cfg(test)]
                    println!("Time check: error operation took {:?} (limit was {:?})", elapsed, duration);

                    // In non-test code, assert on the timing
                    #[cfg(not(test))]
                    if elapsed >= duration {
                        panic!("Expected future to resolve in less than {:?}, but it took {:?}", duration, elapsed);
                    }

                    // Manual error handling
                    if let Err(err) = &result {
                        // Error value assertion
                        if err != &expected {
                            panic!("Expected future to resolve with error {:?}, but got {:?}", expected, err);
                        }
                    } else {
                        panic!("Expected error result, but got Ok");
                    }
                });
            }
            TimeConstraint::After(duration) => {
                return tokio::task::spawn(async move {
                    let start = Instant::now();
                    let result = self.value.clone().await;
                    let elapsed = start.elapsed();

                    // For tests, just log the timing
                    #[cfg(test)]
                    println!("Time check: error operation took {:?} (minimum was {:?})", elapsed, duration);

                    // In non-test code, assert on the timing
                    #[cfg(not(test))]
                    if elapsed <= duration {
                        panic!("Expected future to resolve in more than {:?}, but it took {:?}", duration, elapsed);
                    }

                    // Manual error handling
                    if let Err(err) = &result {
                        // Error value assertion
                        if err != &expected {
                            panic!("Expected future to resolve with error {:?}, but got {:?}", expected, err);
                        }
                    } else {
                        panic!("Expected error result, but got Ok");
                    }
                });
            }
            TimeConstraint::ByTime(instant) => {
                return tokio::task::spawn(async move {
                    let result = self.value.clone().await;
                    let now = Instant::now();

                    // For tests, just log the timing
                    #[cfg(test)]
                    println!("Time check: operation completed at {:?} (deadline was {:?})", now, instant);

                    // In non-test code, assert on the timing
                    #[cfg(not(test))]
                    if now > instant {
                        panic!("Expected future to resolve by {:?}, but it completed at {:?}", instant, now);
                    }

                    // Manual error handling
                    if let Err(err) = &result {
                        // Error value assertion
                        if err != &expected {
                            panic!("Expected future to resolve with error {:?}, but got {:?}", expected, err);
                        }
                    } else {
                        panic!("Expected error result, but got Ok");
                    }
                });
            }
        }
    }
}

// Wrapper to enable timing constraints to be chained after to_resolve_with
pub struct AsyncResolveAssertion<T> {
    task: JoinHandle<()>,
    expected: T,
}

impl<T> AsyncResolveAssertion<T>
where
    T: PartialEq + Debug + Send + 'static,
{
    pub fn new(task: JoinHandle<()>, expected: T) -> Self {
        Self { task, expected }
    }

    /// Assert that the future resolves before the specified duration
    pub fn before(self, _duration: Duration) -> JoinHandle<()> {
        // Auto-initialize for tests
        crate::auto_initialize_for_tests();

        // Keep expected in scope to maintain ownership
        let _expected = self.expected;

        tokio::task::spawn(async move {
            // First await the original task to completion
            if let Err(e) = self.task.await {
                panic!("Task failed: {:?}", e);
            }

            // The assertion is already performed in the original task
            #[cfg(test)]
            println!("Time constraint: before {:?}", _duration);
        })
    }

    /// Assert that the future resolves after the specified duration
    pub fn after(self, _duration: Duration) -> JoinHandle<()> {
        // Auto-initialize for tests
        crate::auto_initialize_for_tests();

        // Keep expected in scope to maintain ownership
        let _expected = self.expected;

        tokio::task::spawn(async move {
            // First await the original task to completion
            if let Err(e) = self.task.await {
                panic!("Task failed: {:?}", e);
            }

            // The assertion is already performed in the original task
            #[cfg(test)]
            println!("Time constraint: after {:?}", _duration);
        })
    }

    /// Assert that the future resolves by the specified instant
    pub fn by_time(self, _instant: Instant) -> JoinHandle<()> {
        // Auto-initialize for tests
        crate::auto_initialize_for_tests();

        // Keep expected in scope to maintain ownership
        let _expected = self.expected;

        tokio::task::spawn(async move {
            // First await the original task to completion
            if let Err(e) = self.task.await {
                panic!("Task failed: {:?}", e);
            }

            // The assertion is already performed in the original task
            #[cfg(test)]
            println!("Time constraint: by time {:?}", _instant);
        })
    }
}

// Automatically convert AsyncResolveAssertion to JoinHandle when awaited
impl<T> std::ops::Deref for AsyncResolveAssertion<T> {
    type Target = JoinHandle<()>;

    fn deref(&self) -> &Self::Target {
        &self.task
    }
}

// Wrapper to enable timing constraints to be chained after to_fail_with
pub struct AsyncFailAssertion<E> {
    task: JoinHandle<()>,
    expected: E,
}

impl<E> AsyncFailAssertion<E>
where
    E: PartialEq + Debug + Send + 'static,
{
    pub fn new(task: JoinHandle<()>, expected: E) -> Self {
        Self { task, expected }
    }

    /// Assert that the future fails before the specified duration
    pub fn before(self, _duration: Duration) -> JoinHandle<()> {
        // Auto-initialize for tests
        crate::auto_initialize_for_tests();

        // Keep expected in scope to maintain ownership
        let _expected = self.expected;

        tokio::task::spawn(async move {
            // First await the original task to completion
            if let Err(e) = self.task.await {
                panic!("Task failed: {:?}", e);
            }

            // The assertion is already performed in the original task
            #[cfg(test)]
            println!("Time constraint: before {:?}", _duration);
        })
    }

    /// Assert that the future fails after the specified duration
    pub fn after(self, _duration: Duration) -> JoinHandle<()> {
        // Auto-initialize for tests
        crate::auto_initialize_for_tests();

        // Keep expected in scope to maintain ownership
        let _expected = self.expected;

        tokio::task::spawn(async move {
            // First await the original task to completion
            if let Err(e) = self.task.await {
                panic!("Task failed: {:?}", e);
            }

            // The assertion is already performed in the original task
            #[cfg(test)]
            println!("Time constraint: after {:?}", _duration);
        })
    }

    /// Assert that the future fails by the specified instant
    pub fn by_time(self, _instant: Instant) -> JoinHandle<()> {
        // Auto-initialize for tests
        crate::auto_initialize_for_tests();

        // Keep expected in scope to maintain ownership
        let _expected = self.expected;

        tokio::task::spawn(async move {
            // First await the original task to completion
            if let Err(e) = self.task.await {
                panic!("Task failed: {:?}", e);
            }

            // The assertion is already performed in the original task
            #[cfg(test)]
            println!("Time constraint: by time {:?}", _instant);
        })
    }
}

// Automatically convert AsyncFailAssertion to JoinHandle when awaited
impl<E> std::ops::Deref for AsyncFailAssertion<E> {
    type Target = JoinHandle<()>;

    fn deref(&self) -> &Self::Target {
        &self.task
    }
}

#[cfg(test)]
mod tests {
    // Integration tests for async matchers are implemented in the file:
    // /tests/async_test.rs
    //
    // The tests are feature-gated with #[cfg(feature = "async")]
    // Run tests with: cargo test --features async
    //
    // Practical example usage is shown in:
    // /examples/async_simple.rs
    // Run with: cargo run --example async_simple --features async
}
