//! Runtime Calls (RtCalls) allow the running module
//! to interact with the host runtime

/// All the available runtime calls
#[repr(u16)]
#[derive(Debug)]
pub enum Syscall {
    /// Do absolutely nothing and return 0
    Nop = 0,
}
