use std::ffi::{c_void, CStr};

use common::rtcall::{RtCall, RtCallLogArg};
use libloading::{Library, Symbol};
use log::trace;

type SetupFunc<'a> = Symbol<
    'a,
    unsafe extern "C" fn(
        *mut c_void,
        extern "C" fn(*mut c_void, RtCall, *mut c_void) -> usize,
    ) -> u32,
>;

pub struct Module {
    library: Library,
}

impl Module {
    pub fn new(library: Library) -> Self {
        Self { library }
    }

    pub fn setup(&mut self) {
        let func: SetupFunc = unsafe { self.library.get("__lplc_setup") }.unwrap();

        unsafe { func(self as *mut _ as *mut c_void, syscall_function) };
    }

    fn rtcall(&mut self, rtcall: RtCall, arg: *mut c_void) -> usize {
        trace!("rtcall({rtcall:?}, 0x{:x})", unsafe {
            core::mem::transmute::<*mut c_void, usize>(arg)
        });

        match rtcall {
            RtCall::Log => {
                let arg: &RtCallLogArg = unsafe { core::mem::transmute(arg) };
                let c_str = unsafe { CStr::from_ptr(arg.ptr) };
                let string = c_str.to_string_lossy();

                println!("{:?} => {}", arg.loglevel, string);

                0
            }
            _ => 0,
        }
    }
}

pub extern "C" fn syscall_function(
    context: *mut c_void,
    rtcall: RtCall,
    arg: *mut c_void,
) -> usize {
    //TODO: Make this less fragile by keeping track of handed-out context pointers
    let s: &mut Module = unsafe { core::mem::transmute(context) };
    s.rtcall(rtcall, arg)
}
