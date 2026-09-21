//! Runtime calls (RtCalls) that allow PLC modules to interact
//! with the host runtime in the setup phase

/// All the available runtime calls
#[repr(u16)]
#[derive(Debug)]
pub enum RtCall {
    /// Do absolutely nothing and return 0
    Nop = 0,
}
