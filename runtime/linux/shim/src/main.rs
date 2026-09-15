use std::{env::args, process::exit};

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 3 {
        println!("Expected two arguments: <shm_name> <module>");
        exit(1);
    }
}
