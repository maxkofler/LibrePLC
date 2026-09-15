use std::ffi::c_void;

use common::rtcall::RtCall;
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

    fn syscall(&mut self, syscall: RtCall, arg: *mut c_void) -> usize {
        trace!("Called real syscall function of self, library");

        0
    }
}

pub extern "C" fn syscall_function(
    context: *mut c_void,
    syscall: RtCall,
    arg: *mut c_void,
) -> usize {
    //TODO: Make this less fragile by keeping track of handed-out context pointers
    let s: &mut Module = unsafe { core::mem::transmute(context) };
    s.syscall(syscall, arg)
}
