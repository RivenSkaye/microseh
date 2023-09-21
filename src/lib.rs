use std::ffi::c_void;

use exception::Exception;

mod code;
mod exception;

type HandledProc = unsafe extern "system" fn(*mut c_void);

extern "system" {
    #[link_name = "HandlerStub"]
    fn handler_stub(proc: HandledProc, closure: *mut c_void, exception: *mut Exception) -> bool;
}

unsafe extern "system" fn handled_proc<F>(closure: *mut c_void)
where
    F: FnMut(),
{
    closure.cast::<F>().as_mut().unwrap()();
}

/// Executes a closure or function within a SEH-handled context.
///
/// # Arguments
///
/// * `closure` - The closure or function to be executed within the SEH-handled context.
///
/// # Returns
///
/// * `Ok(())` - If the closure executed without throwing any exceptions.
/// * `Err(Exception)` - If an exception occurred during the execution of the closure.
pub fn try_seh<F>(mut closure: F) -> Result<(), Exception>
where
    F: FnMut(),
{
    let mut exception = Exception::empty();
    let closure = &mut closure as *mut _ as *mut c_void;

    unsafe {
        match handler_stub(handled_proc::<F>, closure, &mut exception) {
            false => Err(exception),
            true => Ok(()),
        }
    }
}

/// Macro that provides a more convenient way to use the `try_seh` function
/// with error propagation.
///
/// This is the same as doing:
/// ```rust
/// microseh::try_seh(|| { /* ... some code ... */} )?
/// ```
#[macro_export]
macro_rules! try_seh {
    ($body:block) => {
        microseh::try_seh(|| $body)?
    };
}
