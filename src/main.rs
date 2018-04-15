extern crate binarr;

use std::env;
use std::process;
use binarr::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    binarr::run(config).unwrap_or_else(|err| {
        println!("Something went wrong: {}", err);
        process::exit(1);
    });
}
