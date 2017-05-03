use std::io;
use std::io::{Read, Write};

#[derive(Debug)]
enum Error {
    ByteError,
    EndOfInput
}

type Secret = String;

const REDACTED: u8 = 7; // bell
const OUTPUT: u8 = 95;  // underscore

pub struct Redactor<'a, R: 'a, W: 'a> {
    input: &'a mut R,
    output: &'a mut W,
    buf: &'a mut [u8],
    size: usize,
    redacting: usize,
    secret: &'a Secret
}

impl<'a, R: 'a + Read, W: 'a + Write> Redactor<'a, R, W> {
    pub fn new(input: &'a mut R, output: &'a mut W, buf: &'a mut [u8], secret: &'a Secret) -> Redactor<'a, R, W> {
        let size = buf.len();
        Redactor {
            input: input,
            output: output,
            buf: buf,
            size: size,
            redacting: 0,
            secret: secret
        }
    }

    pub fn scan(&mut self) {
        self.setup();
        loop {
            match self.run() {
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
    }

    fn run(&mut self) -> Result<(), Error> {
        if self.secret.as_bytes() == self.buf {
            for i in 0..self.size {
                self.buf[i] = REDACTED;
            }
        }
        self.advance()
    }

    fn advance(&mut self) -> Result<(), Error> {
        self.emit_byte(0);

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

    fn emit_byte(&mut self, i: usize) {
        let head = self.buf[i];
        self.redacting = if head == REDACTED { self.redacting + 1 } else { 0 };
        match self.redacting {
            0 => {
                self.output.write(&[head]);
            },
            1...5 => {
                self.output.write(&[OUTPUT]);
            },
            _ => {}
        }
    }

    fn emit_tail(&mut self) {
        for i in 1..self.size {
            self.emit_byte(i);
        }
    }
}

fn main() {
    // TODO load the secrets from somewhere on the build machine
    // TODO chain redactors, one for each secret
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buf = [0; 6]; 
    let secret = String::from("abcdef");
    let mut redactor = Redactor::new(&mut stdin, &mut stdout, &mut buf, &secret);
    redactor.scan();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn secret_at_start() {
        let mut input = Cursor::new(&b"abcdefghij rest of input"[..]);
        let mut output = Cursor::new(vec![]);
        let mut buf = [0; 10];
        let secret = String::from("abcdefghij");
        {
            let mut redactor = Redactor::new(&mut input, &mut output, &mut buf, &secret);
            redactor.scan();
        }
        assert_eq!(b"_____ rest of input".to_vec(), output.into_inner());
    }

    #[test]
    fn secret_at_end() {
        let mut input = Cursor::new(&b"rest of input abcdefghijk"[..]);
        let mut output = Cursor::new(vec![]);
        let mut buf = [0; 11];
        let secret = String::from("abcdefghijk");
        {
            let mut redactor = Redactor::new(&mut input, &mut output, &mut buf, &secret);
            redactor.scan();
        }
        assert_eq!(b"rest of input _____".to_vec(), output.into_inner());
    }
}
