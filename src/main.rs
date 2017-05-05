extern crate exec;
extern crate pty;

mod redactor;

use exec::Command;
use pty::fork::Fork;
use redactor::{noop, scan, Secret};
use std::{env, ffi, io};

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
        let (cmd, args) = get_cmd();
        let error = Command::new(&cmd).args(&args).exec();
        panic!("Command failed: {:?}", error);
    }
    
}

fn run(mut secrets: &mut Vec<Secret>) {
    let fork = Fork::from_ptmx().unwrap_or_else(|error| panic!("Fork error: {:?}", error));
    if let Some(mut pty) = fork.is_parent().ok() {
        let mut stdout = io::stdout();
        scan(&mut pty, &mut stdout, &mut secrets);
    } else {
        let (cmd, args) = get_cmd();
        let error = Command::new(&cmd).args(&args).exec();
        panic!("Command failed: {:?}", error);
    }
}

fn get_cmd() -> (ffi::OsString, Vec<ffi::OsString>) {
    let mut args = env::args_os().skip(1);
    if args.len() < 2 { panic!("No command given"); }
    let cmd = args.next().unwrap();
    let args = args.collect::<Vec<_>>();
    (cmd, args)
}
