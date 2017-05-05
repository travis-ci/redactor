extern crate exec;
extern crate pty;

mod redactor;

use exec::Command;
use pty::fork::Fork;
use redactor::{scan, Secret};
use std::env;
use std::io;

fn main() {
    match env::var("TRAVIS_SECRETS") {
        Ok(value) => {
            let mut secrets = value.split(",").map(|s| String::from(s)).collect::<Vec<Secret>>();
            run(&mut secrets);
        },
        _ => {
            // Probably don't want to exit here, but allow the output through
            println!("Environment variable TRAVIS_SECRETS not found");
        }
    }
}

fn run(mut secrets: &mut Vec<Secret>) {
    let fork = Fork::from_ptmx().unwrap_or_else(|error| panic!("Fork error: {:?}", error));
    if let Some(mut pty) = fork.is_parent().ok() {
        let mut stdout = io::stdout();
        scan(&mut pty, &mut stdout, &mut secrets);
    } else {
        let error = Command::new("ruby").arg("stream.rb").exec();
        panic!("Command failed: {:?}", error);
    }
}
