extern crate base64;

pub type Secret = String;

enum EncodedSecret<'a> {
    Ascii(&'a str),
    Base64(&'a str)
}

impl<'a> EncodedSecret<'a> {
    pub fn new(raw: &'a str) -> Option<EncodedSecret<'a>> {
        let mut parts = raw.split(":");
        match (parts.next(), parts.next()) {
            // Tagged as base 64
            (Some("base64"), Some(secret)) => {
                Some(EncodedSecret::Base64(secret))
            },
            // Tagged as ascii
            (Some("ascii"), Some(secret)) => {
                Some(EncodedSecret::Ascii(secret))
            },
            // Not tagged, assume ascii
            (Some(secret), None) if !secret.is_empty() => {
                Some(EncodedSecret::Ascii(secret))
            },
            (_, _) => None
        }
    }

    pub fn decode(&self) -> Secret {
        match *self {
            EncodedSecret::Base64(secret) => {
                let vec = base64::decode(secret).unwrap_or_else(|error| panic!(error));
                String::from_utf8(vec).unwrap_or_else(|error| panic!(error))
            },
            EncodedSecret::Ascii(secret) => String::from(secret)
        }
    }
}

pub fn decode<'a>(raw: &'a str) -> Option<Secret> {
    let es = EncodedSecret::new(raw);
    es.and_then(|e| Some(e.decode()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base64() {
        let raw = "base64:aGVsbG8=";
        assert_eq!(String::from("hello"), decode(raw).unwrap());
    }

    #[test]
    fn ascii() {
        let raw = "ascii:hello";
        assert_eq!(String::from("hello"), decode(raw).unwrap());
    }

    #[test]
    fn ascii_untagged() {
        let raw = "hello";
        assert_eq!(String::from("hello"), decode(raw).unwrap());
    }

    #[test]
    fn empty() {
        let raw = "";
        assert_eq!(None, decode(raw));
    }
}
