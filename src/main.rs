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

    let parsed_value: Result<i32, ParseIntError> = match val.trim().parse::<i32>() {
        Ok(n) => Ok(n),
        Err(e) => Err(e),
    };

    match parsed_value.unwrap() {
        1 => println!("Example 1"),
        _ => unimplemented!(),
    }
}
