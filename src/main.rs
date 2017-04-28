use std::io;
use std::io::Read;

#[derive(Debug)]
enum Error {
    ByteError,
    EndOfInput,
    SecretFound(Secret)
}
type Secret = String;

pub struct Scanner<'a> {
    input: &'a mut io::Stdin,
    buf: &'a mut [u8],
    size: usize,
    secrets: &'a Vec<Secret>
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a mut io::Stdin, buf: &'a mut [u8], secrets: &'a Vec<Secret>) -> Scanner<'a> {
        let size = buf.len();
        Scanner {
            input: input,
            buf: buf,
            size: size,
            secrets: secrets
        }
    }

    pub fn scan(&mut self) {
        self.input.read_exact(&mut self.buf);
        loop {
            match self.compare() {
                Ok(_) => continue,
                Err(e) => {
                    println!("{:?}", e);
                    break
                }
            }
        }
    }

    fn compare(&mut self) -> Result<(), Error> {
        match self.compare_secrets() {
            Ok(_) => self.advance(),
            Err(e) => Err(e)
        }
    }

    fn compare_secrets(&self) -> Result<(), Error> {
        for s in self.secrets {
            // TODO match pattern instead of exact match?
            if s.as_bytes() == self.buf {
                return Err(Error::SecretFound(s.to_owned()))
            }
        }
        Ok(())
    }

    fn advance(&mut self) -> Result<(), Error> {
        match self.input.bytes().next() {
            // A byte was read from stdin so we'll shift the buffer
            Some(Ok(byte)) => {
                for i in 0..(self.size - 1) {
                    self.buf[i] = self.buf[i+1];
                }
                self.buf[self.size - 1] = byte;
                Ok(())
            },
            // No bytes left to read
            None => Err(Error::EndOfInput),
            // There was an error reading the next byte
            _ => Err(Error::ByteError)
        }
    }
}

fn main() {
    // TODO load the secrets from somewhere on the build machine
    let secrets = vec!["abc123def4".to_string(), "my-secret!".to_string()];
    let mut stdin = io::stdin();
    let mut buf = [0; 10]; 
    let mut scanner = Scanner::new(&mut stdin, &mut buf, &secrets);
    scanner.scan();
}
