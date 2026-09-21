use core::ffi::c_char;

/// The loglevel to use for logging events
#[repr(u8)]
#[derive(Debug)]
pub enum RtCallLogLevel {
    Error = 0x0,
    Warning = 0x10,
    Info = 0x20,
    Debug = 0x30,
    Trace = 0x40,
}

/// The structure to be subitted as `arg` to [RtCall::Log](super::RtCall::Log)
#[repr(C)]
pub struct RtCallLogArg {
    /// The loglevel to use for logging
    pub loglevel: RtCallLogLevel,
    /// A pointer to the null-terminated string to be logged
    pub ptr: *const c_char,
}
