use std::{
    env::args,
    process::{Command, Stdio},
};

use seccomp::{Action, Context};

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        unsafe { std::env::set_var("RUST_LOG", "trace") };
    }
    pretty_env_logger::init();

    let args: Vec<String> = args().collect();
    let cmd = args[1].to_string();
    let mut cmd = Command::new(cmd);
    cmd.stderr(Stdio::inherit());

    let mut child = cmd.spawn().unwrap();
    println!("Yay!");
    child.wait().unwrap();

    /*

    let mut module = unsafe {
        let lib = Library::new(&args[1]).unwrap();
        Module::new(lib)

    };

    module.setup();
    */
}
