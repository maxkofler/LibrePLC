use std::{env::args, thread::sleep, time::Duration};

use libloading::Library;
use rt_linux::Module;

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        unsafe { std::env::set_var("RUST_LOG", "trace") };
    }
    pretty_env_logger::init();

    let args: Vec<String> = args().collect();

    let mut module = unsafe {
        let lib = Library::new(&args[1]).unwrap();
        Module::new(lib)
    };

    module.setup();

    loop {
        module.cycle();

        sleep(Duration::from_millis(1000));
    }
}
