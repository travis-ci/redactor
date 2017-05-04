use std::env;
use std::io;

mod redactor;
use redactor::{scan, Secret};

fn main() {
    match env::var("TRAVIS_SECRETS") {
        Ok(value) => {
            let mut secrets = value.split(",").map(|s| String::from(s)).collect::<Vec<Secret>>();
            let mut stdin = io::stdin();
            let mut stdout = io::stdout();
            scan(&mut stdin, &mut stdout, &mut secrets);        
        },
        _ => {
            // Probably don't want to exit here, but allow the output through
            println!("Environment variable TRAVIS_SECRETS not found");
        }
    }
}
