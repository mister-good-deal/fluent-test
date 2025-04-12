use crate::backend::Assertion;
use std::cell::RefCell;

/// Event types that can be emitted within the testing system
#[derive(Debug, Clone)]
pub enum AssertionEvent {
    /// A successful assertion
    Success(Assertion<()>),
    /// A failed assertion
    Failure(Assertion<()>),
    /// Test session completed
    SessionCompleted,
}

// Thread-local registry of success handlers
// Define type aliases to reduce complexity
type AssertionHandler = Box<dyn Fn(Assertion<()>)>;

thread_local! {
    static SUCCESS_HANDLERS: RefCell<Vec<AssertionHandler>> = RefCell::new(Vec::new());
    static FAILURE_HANDLERS: RefCell<Vec<AssertionHandler>> = RefCell::new(Vec::new());
    static SESSION_COMPLETED_HANDLERS: RefCell<Vec<Box<dyn Fn()>>> = RefCell::new(Vec::new());
    static INITIALIZED: RefCell<bool> = const { RefCell::new(false) };
}

/// EventEmitter is responsible for sending events and managing event handlers
pub struct EventEmitter;

impl EventEmitter {
    /// Initialize the event system
    pub fn init() {
        INITIALIZED.with(|initialized| {
            let mut initialized = initialized.borrow_mut();
            if !*initialized {
                *initialized = true;
            }
        });
    }

    /// Emit an event to all registered handlers
    pub fn emit(event: AssertionEvent) {
        match event {
            AssertionEvent::Success(assertion) => {
                SUCCESS_HANDLERS.with(|handlers| {
                    let handlers = handlers.borrow();
                    for handler in handlers.iter() {
                        handler(assertion.clone());
                    }
                });
            }
            AssertionEvent::Failure(assertion) => {
                FAILURE_HANDLERS.with(|handlers| {
                    let handlers = handlers.borrow();
                    for handler in handlers.iter() {
                        handler(assertion.clone());
                    }
                });
            }
            AssertionEvent::SessionCompleted => {
                SESSION_COMPLETED_HANDLERS.with(|handlers| {
                    let handlers = handlers.borrow();
                    for handler in handlers.iter() {
                        handler();
                    }
                });
            }
        }
    }
}

/// Register a handler for success events
pub fn on_success<F>(handler: F)
where
    F: Fn(Assertion<()>) + 'static,
{
    SUCCESS_HANDLERS.with(|handlers| {
        handlers.borrow_mut().push(Box::new(handler));
    });
}

/// Register a handler for failure events
pub fn on_failure<F>(handler: F)
where
    F: Fn(Assertion<()>) + 'static,
{
    FAILURE_HANDLERS.with(|handlers| {
        handlers.borrow_mut().push(Box::new(handler));
    });
}

/// Register a handler for session completion events
pub fn on_session_completed<F>(handler: F)
where
    F: Fn() + 'static,
{
    SESSION_COMPLETED_HANDLERS.with(|handlers| {
        handlers.borrow_mut().push(Box::new(handler));
    });
}

// This is an internal function, deprecated in favor of using Config.apply()
// but kept for compatibility with example and test code
#[doc(hidden)]
pub fn initialize_event_system() {
    EventEmitter::init();

    // Register default handlers from the Reporter
    crate::Reporter::init();
}
