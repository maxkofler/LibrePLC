use std::{env::args, ffi::c_void};

use common::syscall;
use libloading::{Library, Symbol};

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        unsafe { std::env::set_var("RUST_LOG", "trace") };
    }
    pretty_env_logger::init();

    let args: Vec<String> = args().collect();

    unsafe {
        let lib = Library::new(&args[1]).unwrap();

        let f_setup: Symbol<
            unsafe extern "C" fn(
                usize,
                extern "C" fn(usize, syscall::Syscall, *mut c_void) -> usize,
            ) -> u32,
        > = lib.get(b"__lplc_setup").unwrap();

        let v = f_setup(0xDEADBEEF, syscall::syscall_function);

        println!("Setup: {}", v);
    }
}
