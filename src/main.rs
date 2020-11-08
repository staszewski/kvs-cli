use std::io;
use std::io::prelude::*;

fn main() {
    print!("Instruction");
    io::stdout().flush().unwrap();
    let mut val = String::new();

    io::stdin()
        .read_line(&mut val)
        .expect("Error getting guess");
}
