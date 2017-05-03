use std::env;
use std::io;

mod redactor;
use redactor::{Redactor, Secret};

fn main() {
    match env::var("TRAVIS_SECRETS") {
        Ok(value) => {
            let mut secrets = value.split(",").map(|s| String::from(s)).collect::<Vec<Secret>>();
            scan(&mut secrets);
        },
        _ => {
            // Probably don't want to exit here, but allow the output through
            println!("Environment variable TRAVIS_SECRETS not found");
        }
    }
}

fn scan(secrets: &mut Vec<Secret>) {
    secrets.sort_by(|a, b| {
        a.as_bytes().len().cmp(&b.as_bytes().len())
    });
    let max = secrets.get(0).unwrap().len();
    let mut s = secrets;
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buf = vec![0; max]; 
    let mut redactor = Redactor::new(&mut stdin, &mut stdout, &mut buf, &mut s);
    redactor.scan();
}
