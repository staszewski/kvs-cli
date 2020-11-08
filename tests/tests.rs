use assert_cmd::prelude::*;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::str;

#[test]
fn initial_output() {
    let process = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .output()
        .expect("failed to execute process");
    assert_eq!(str::from_utf8(&process.stdout).unwrap(), "Instruction\n");
}

#[test]
fn input_feature() -> io::Result<()> {
    let mut child = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let input = b"Hello";
    {
        let child_stdin = child.stdin.as_mut().unwrap();
        child_stdin.write_all(input)?;
    }

    let output = child.wait_with_output()?;
    assert_eq!(
        str::from_utf8(&output.stdout).unwrap(),
        "Instruction\nHello"
    );
    Ok(())
}
