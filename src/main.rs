extern crate exec;
extern crate pty;
extern crate tempdir;

mod redactor;
mod wrapper;

use pty::fork::Fork;
use redactor::{noop, scan, Secret};
use std::{env, ffi, io, process};
use tempdir::TempDir;
use wrapper::Wrapper;

fn main() {
    let fork = Fork::from_ptmx().unwrap_or_else(|error| panic!("Error creating pty: {:?}", error));
    let cmd = get_cmd();
    let tmpdir = TempDir::new("redactor").unwrap_or_else(|error| panic!("Could not create tmpdir: {}", error));
    let mut wrapper = Wrapper::new(cmd.to_str().unwrap(), &tmpdir);

    // Parent branch handles scanning
    if let Some(mut pty) = fork.is_parent().ok() {
        match env::var("TRAVIS_SECRETS") {
            // Found secrets, starting to scan input
            Ok(ref value) if !value.is_empty() => {
                let mut secrets = value.split(",").map(|s| String::from(s)).collect::<Vec<Secret>>();
                scan(&mut pty, &mut io::stdout(), &mut secrets);
                let _ = fork.wait();
                process::exit(wrapper.status().unwrap_or(0));
            },
            // No secrets, sending input through noop
            _ => {
                noop(&mut pty, &mut io::stdout());
                let _ = fork.wait();
                process::exit(wrapper.status().unwrap_or(0));
            }
        }
    // Child branch execs command, replacing current process
    } else {
        panic!("Command failed: {:?}", wrapper.exec());
    }
}

fn get_cmd() -> ffi::OsString {
    let mut args = env::args_os().skip(1);
    if args.len() == 0 { panic!("No command given"); }
    args.next().unwrap()
}
