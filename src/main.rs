use xgrep::run;
use xgrep::Config;
use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("parse arguments error: {}", err);
        process::exit(-1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {}", err);
        process::exit(-1);
    }
}
