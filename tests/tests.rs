use assert_cmd::prelude::*;
use std::process::Command;
use std::str;

#[test]
fn feature() {
    let process = Command::cargo_bin("kvs")
        .unwrap()
        .output()
        .expect("failed to execute process");
    assert_eq!(str::from_utf8(&process.stdout).unwrap(), "Instruction");
}
