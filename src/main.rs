extern crate getopts;
extern crate pty;
extern crate tempdir;

mod redactor;
mod secret;
mod wrapper;

use getopts::Options;
use pty::fork::Fork;
use redactor::{noop, scan};
use secret::{decode, Secret};
use std::{env, io, process};
use tempdir::TempDir;
use wrapper::Wrapper;

fn main() {
    let (script, mut secrets) = parse_command();

    let fork = Fork::from_ptmx().unwrap_or_else(|error| panic!("Error creating pty: {:?}", error));
    let tmpdir = TempDir::new("redactor").unwrap_or_else(|error| panic!("Could not create tmpdir: {}", error));
    let mut wrapper = Wrapper::new(&script, &tmpdir);

    // Parent branch handles scanning
    if let Some(mut pty) = fork.is_parent().ok() {
        if !secrets.is_empty() {
            // Found secrets, starting to scan input
            scan(&mut pty, &mut io::stdout(), &mut secrets);
            let _ = fork.wait();
            process::exit(wrapper.status().unwrap_or(0));
        } else {
            // No secrets, sending input through noop
            noop(&mut pty, &mut io::stdout());
            let _ = fork.wait();
            process::exit(wrapper.status().unwrap_or(0));
        }
    // Child branch execs command, replacing current process
    } else {
        panic!("Command failed: {:?}", wrapper.exec());
    }
}

fn parse_command() -> (String, Vec<Secret>) {
    let mut args = env::args();
    let command = args.next().unwrap();
    let args = args.collect::<Vec<String>>();

    let mut options = Options::new();
    options.optopt("r", "run", "The script to be run", "./build.sh");
    options.optmulti("s", "secret", "Secret to be redacted", "SECRET");
    options.optflag("h", "help", "Show help");

    let matches = options.parse(&args[..]).unwrap_or_else(|error| {
        panic!("Could not parse options: {:?}", error)
    });

    // Show instructions and exit
    if matches.opt_present("h") {
        println!("{}", options.short_usage(&command));
        process::exit(0);
    }

    // Script is mandatory
    if !matches.opt_present("r") {
        panic!("Script not given");
    }

    // Pull secrets from env var
    let mut secrets = match env::var("TRAVIS_SECRETS") {
        Ok(ref value) => {
            value.split(",").flat_map(|s| decode(s)).collect::<Vec<Secret>>()
        },
        _ => vec![]
    };

    // Pull secrets from opts
    if matches.opt_present("s") {
        secrets.extend(matches.opt_strs("s").iter().flat_map(|s| {
            decode(s)
        }).collect::<Vec<Secret>>());
    }

    (matches.opt_str("r").unwrap(), secrets)
}
