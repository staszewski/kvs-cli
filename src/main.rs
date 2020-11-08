mod parsers;

use parsers::parse_users_input;
use std::io;
use std::io::prelude::*;
use std::num::ParseIntError;

fn main() {
    println!("Instruction");
    io::stdout().flush().unwrap();
    let mut val = String::new();

    io::stdin()
        .read_line(&mut val)
        .expect("Error getting guess");

    let parsed_value: Result<i32, ParseIntError> = parse_users_input(val);

    match parsed_value.unwrap() {
        1 => println!("Example 1"),
        _ => unimplemented!(),
    }
}
