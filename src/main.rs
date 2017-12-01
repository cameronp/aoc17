extern crate aoc17;

use std::env;
use std::process;

use aoc17::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else( |err| {
        println!("Problem parsing args: {}", err);
        process::exit(1)
    });

    dispatch(&config);
}
