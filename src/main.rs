extern crate exec;
extern crate pty;
extern crate siphasher;

mod redactor;
mod runner;

use pty::fork::Fork;
use redactor::{noop, scan, Secret};
use runner::Runner;
use std::{env, ffi, io, process};

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
    let cmd = get_cmd();
    let mut runner = Runner::new(cmd.to_str().unwrap());
    if let Some(mut pty) = fork.is_parent().ok() {
        let mut stdout = io::stdout();
        noop(&mut pty, &mut stdout);
        let _ = fork.wait();
        process::exit(runner.cleanup_status().unwrap_or(0));
    } else {
        let error = runner.exec();
        panic!("Command failed: {:?}", error);
    }
}

fn run(mut secrets: &mut Vec<Secret>) {
    let fork = Fork::from_ptmx().unwrap_or_else(|error| panic!("Fork error: {:?}", error));
    let cmd = get_cmd();
    let mut runner = Runner::new(cmd.to_str().unwrap());
    if let Some(mut pty) = fork.is_parent().ok() {
        let mut stdout = io::stdout();
        scan(&mut pty, &mut stdout, &mut secrets);
        let _ = fork.wait();
        process::exit(runner.cleanup_status().unwrap_or(0));
    } else {
        let error = runner.exec();
        panic!("Command failed: {:?}", error);
    }
}

fn get_cmd() -> ffi::OsString {
    let mut args = env::args_os().skip(1);
    if args.len() < 1 { panic!("No command given"); }
    args.next().unwrap()
}
