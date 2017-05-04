use std::io::{Read, Write};

#[derive(Debug)]
enum Error {
    ByteError,
    EndOfInput
}

pub type Secret = String;

const REDACTED_BYTE: u8 = 7; // bell
const REDACTION_MSG: &'static [u8; 9] = b"[secure] ";

pub struct Redactor<'a, R: 'a, W: 'a> {
    input: &'a mut R,
    output: &'a mut W,
    buf: &'a mut [u8],
    size: usize,
    redacting: usize,
    secrets: &'a Vec<Secret>
}

impl<'a, R: 'a + Read, W: 'a + Write> Redactor<'a, R, W> {
    pub fn new(input: &'a mut R, output: &'a mut W, buf: &'a mut [u8], secrets: &'a Vec<Secret>) -> Redactor<'a, R, W> {
        let size = buf.len();
        Redactor {
            input: input,
            output: output,
            buf: buf,
            size: size,
            redacting: size,
            secrets: secrets
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
        for s in self.secrets {
            let bytes = s.as_bytes();
            if bytes == self.buf[0..bytes.len()].as_ref() {
                for i in 0..bytes.len() {
                    self.buf[i] = REDACTED_BYTE;
                }
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

    fn not_redacting(&self) -> bool {
        self.redacting == self.size
    }

    fn is_redacting(&self) -> bool {
        self.redacting > 0 && self.redacting < self.size
    }

    fn emit_byte(&mut self, i: usize) {
        let head = self.buf[i];

        if self.not_redacting() {
            if head == REDACTED_BYTE {
                // output message
                self.output.write(REDACTION_MSG);
                self.redacting -= 1;
            } else {
                self.output.write(&[head]);
            }
        } else if self.is_redacting() {
            // drop byte
            self.redacting -= 1;
        } else {
            // reset
            self.redacting = self.size;
        }
    }

    fn emit_tail(&mut self) {
        for i in 1..self.size {
            self.emit_byte(i);
        }
    }
}

pub fn scan<'a, R, W>(mut input: &'a mut R, mut output: &'a mut W, mut secrets: &'a mut Vec<Secret>)
    where R: 'a + Read, W: 'a + Write
{
    secrets.sort_by(|a, b| {
        b.as_bytes().len().cmp(&a.as_bytes().len())
    });
    let max = secrets.get(0).unwrap_or(&String::from("")).as_bytes().len();
    let mut buf = vec![0; max];
    let mut redactor = Redactor::new(&mut input, &mut output, &mut buf, &mut secrets);
    redactor.scan();
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    fn assert_output(mut secrets: &mut Vec<Secret>, input: &[u8], expected: &[u8]) {
        let mut input = Cursor::new(&input[..]);
        let mut output = Cursor::new(vec![]);
        {
            scan(&mut input, &mut output, &mut secrets);
        }
        assert_eq!(expected.to_vec(), output.into_inner());
    }

    #[test]
    fn secret_at_start() {
        let mut secrets = vec![String::from("abcdefghij")];
        assert_output(&mut secrets, b"abcdefghij rest of input", b"[secure] rest of input");
    }

    #[test]
    fn secret_at_end() {
        let mut secrets = vec![String::from("abcdefghijk")];
        assert_output(&mut secrets, b"rest of input abcdefghijk", b"rest of input [secure] ");
    }

    #[test]
    fn overlapping_secrets() {
        let mut secrets = vec![String::from("abcxxxxxxxx"), String::from("xxxabcxxx"), String::from("abc")];
        assert_output(&mut secrets,
                      b"input abcxxxxxxxx abc input input xxxabcxxx input input",
                      b"input [secure] [secure] put [secure] put input");
    }
}
