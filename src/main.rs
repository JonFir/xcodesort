extern crate xcodesort;

use xcodesort::{run};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1);
    let result = match path {
        Some(path) => run(path),
        None => {
            eprintln!("Usage: {} path/to/xcodeproj or pbxproj",  args[0]);
            process::exit(1);
        }
    };
    result.unwrap();
}
