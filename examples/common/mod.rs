use std::panic;

pub fn guard_test<F>(test_fn: F) -> bool
where
    F: FnOnce() + panic::UnwindSafe,
{
    // Set panic hook to suppress the backtrace output
    let old_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));

    let result = panic::catch_unwind(test_fn).is_ok();

    // Restore the original panic hook
    panic::set_hook(old_hook);

    return result;
}
