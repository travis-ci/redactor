extern crate exec;
extern crate pty;

mod redactor;

use exec::Command;
use pty::fork::Fork;
use redactor::{noop, scan, Secret};
use std::env;
use std::io;

fn main() {
    match env::var("TRAVIS_SECRETS") {
        Ok(ref value) if !value.is_empty() => {
            let mut secrets = value.split(",").map(|s| String::from(s)).collect::<Vec<Secret>>();
            run(&mut secrets);
        },
        _ => pass_through()
    }
}

fn pass_through() {
    let fork = Fork::from_ptmx().unwrap_or_else(|error| panic!("Fork error: {:?}", error));
    if let Some(mut pty) = fork.is_parent().ok() {
        let mut stdout = io::stdout();
        noop(&mut pty, &mut stdout);
    } else {
        let error = Command::new("ruby").arg("stream.rb").exec();
        panic!("Command failed: {:?}", error);
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
