use std::{env::args, ffi::c_void};

use common::syscall;
use libloading::{Library, Symbol};

fn main() {
    let args: Vec<String> = args().collect();

    unsafe {
        let lib = Library::new(&args[1]).unwrap();

        let f_setup: Symbol<
            unsafe extern "C" fn(
                usize,
                extern "C" fn(usize, syscall::Syscall, *mut c_void) -> usize,
            ) -> u32,
        > = lib.get(b"setup").unwrap();

        let v = f_setup(0xDEADBEEF, syscall::syscall_function);

        println!("Setup: {}", v);
    }
}
