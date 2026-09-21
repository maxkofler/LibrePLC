//! Runtime calls (RtCalls) that allow PLC modules to interact
//! with the host runtime in the setup phase

mod log;
pub use log::{RtCallLogArg, RtCallLogLevel};

/// All the available runtime calls
#[repr(u16)]
#[derive(Debug)]
pub enum RtCall {
    /// Do absolutely nothing and return 0
    Nop = 0x00,
    /// Place a new message in the log
    Log = 0x10,
}
