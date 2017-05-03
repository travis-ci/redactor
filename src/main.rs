use std::io;
use std::io::{Read, Write};

#[derive(Debug)]
enum Error {
    ByteError,
    EndOfInput
}

type Secret = String;

pub struct Scanner<'a, R: 'a, W: 'a> {
    input: &'a mut R,
    output: &'a mut W,
    buf: &'a mut [u8],
    pointer: usize,
    size: usize,
    secret: &'a Secret
}

impl<'a, R: 'a + Read, W: 'a + Write> Scanner<'a, R, W> {
    pub fn new(input: &'a mut R, output: &'a mut W, buf: &'a mut [u8], secret: &'a Secret) -> Scanner<'a, R, W> {
        let size = buf.len();
        Scanner {
            input: input,
            output: output,
            buf: buf,
            pointer: 1,
            size: size,
            secret: secret
        }
    }

    pub fn scan(&mut self) {
        self.setup();
        loop {
            match self.maybe_redact() {
                Ok(_) => continue,
                Err(Error::EndOfInput) => {
                    self.emit_tail();
                    break
                },
                Err(_) => break
            }
        }
    }

    fn setup(&mut self) {
        self.input.read_exact(&mut self.buf);
        self.pointer = self.size;
    }

    fn maybe_redact(&mut self) -> Result<(), Error> {
        if self.secret.as_bytes() == self.buf {
            for i in 0..self.size {
                self.buf[i] = 95; // _
            }
        }
        self.advance()
    }

    fn advance(&mut self) -> Result<(), Error> {
        self.emit_head();

        match self.input.bytes().next() {
            // A byte was read from stdin so we'll shift the buffer
            Some(Ok(byte)) => {
                for i in 0..(self.size - 1) {
                    self.buf[i] = self.buf[i+1];
                }
                self.buf[self.size - 1] = byte;
                self.pointer += 1;
                Ok(())
            },
            // No bytes left to read
            None => Err(Error::EndOfInput),
            // There was an error reading the next byte
            _ => Err(Error::ByteError)
        }
    }

    fn emit_head(&mut self) {
        // TODO here is where we could drop bytes to obscure the length of the secret
        self.output.write(&[self.buf[0]]);
    }

    fn emit_tail(&mut self) {
        self.output.write(&self.buf[1..(self.size - 1)]);
    }
}

fn main() {
    // TODO load the secrets from somewhere on the build machine
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buf = [0; 6]; 
    let secret = String::from("abcdef");
    let mut scanner = Scanner::new(&mut stdin, &mut stdout, &mut buf, &secret);
    scanner.scan();
}
