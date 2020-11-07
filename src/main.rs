use clap::{App, Arg, SubCommand};
use std::process;

fn main() {
    let matches = App::new("MyApp")
        .version("1.0")
        .arg(Arg::with_name("version").long("version").short("V"))
        .get_matches();

    if matches.args.is_empty() {
        process::exit(1);
    }

    if matches.is_present("version") {
        println!(env!("CARGO_PKG_VERSION"));
    }
}
