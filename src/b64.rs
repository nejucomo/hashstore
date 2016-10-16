use serialize::base64::{CharacterSet, Config, Newline, ToBase64, FromBase64, FromBase64Error};


// Like ToBase64 but hardwired with our preferred format: url safe,
// no whitespace, no padding.
pub trait ToB64 {
    fn to_b64(&self) -> String;
}


impl ToB64 for [u8] {
    fn to_b64(&self) -> String {
        self.to_base64(Config {
            char_set: CharacterSet::UrlSafe,
            newline: Newline::LF,
            pad: false,
            line_length: None,
        })
    }
}


pub trait FromB64 {
    fn from_b64(&self) -> Result<Vec<u8>, FromBase64Error>;
}


impl FromB64 for str {
    fn from_b64(&self) -> Result<Vec<u8>, FromBase64Error> {
        self.from_base64()
    }
}


#[cfg(test)]
mod tests {
    mod to_b64 {
        use b64::ToB64;

        #[test]
        fn empty_u8() {
            let arr: [u8; 0] = [0; 0];
            assert_eq!("", arr.to_b64());
        }

        #[test]
        fn one_u8() {
            let arr: [u8; 1] = [42];
            assert_eq!("Kg", arr.to_b64());
        }
    }

    mod from_b64 {
        use b64::FromB64;

        #[test]
        fn empty_u8() {
            assert_eq!(Some(vec![]), "".from_b64().ok());
        }

        #[test]
        fn one_u8() {
            assert_eq!(Some(vec![42]), "Kg".from_b64().ok());
        }

        #[test]
        fn malformed_single_char() {
            use serialize::base64::FromBase64Error;

            match "K".from_b64() {
                Err(FromBase64Error::InvalidBase64Length) => {}
                other => {
                    assert!(false, "Unexpected success or error: {:?}", other);
                }
            }
        }

        #[test]
        fn malformed_unrecognized_char() {
            use serialize::base64::FromBase64Error;

            match "K!".from_b64() {
                Err(FromBase64Error::InvalidBase64Byte(b'!', 1)) => {}
                other => {
                    assert!(false, "Unexpected success or error: {:?}", other);
                }
            }
        }
    }
}
