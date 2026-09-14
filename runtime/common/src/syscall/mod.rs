//! System calls that allow PLC modules to interact
//! with the host runtime in the setup phase

use core::ffi::c_void;

use crate::logging::trace;

/// All the available system calls
#[repr(u16)]
#[derive(Debug)]
pub enum Syscall {
    /// Do absolutely nothing and return 0
    Nop = 0,
}

/// The system call handler function
pub extern "C" fn syscall_function(context: usize, syscall: Syscall, arg: *mut c_void) -> usize {
    trace!(
        "syscall(context={context:x?}, syscall={syscall:?}, arg={:x});",
        arg as usize
    );

    match syscall {
        Syscall::Nop => 0,
    }
}
