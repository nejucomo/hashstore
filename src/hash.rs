use blake2_rfc::blake2b::Blake2b;


const HASH_BYTES: usize = 16;


pub struct Hash([u8; HASH_BYTES]);


#[derive(PartialEq, Debug)]
pub enum HashDecodeError {
    InvalidLength(usize),
    InvalidBase64Byte(u8, usize),
}


impl Hash {
    pub fn decode(s: &str) -> Result<Hash, HashDecodeError> {
        use b64::{FromB64, FromBase64Error};

        let encoded_chars = ((HASH_BYTES as f64) * 4f64 / 3f64).ceil() as usize;

        if s.len() == encoded_chars {
            match s.from_b64() {
                Ok(bvec) => {
                    assert_eq!(HASH_BYTES, bvec.len());
                    let mut buf = [0; HASH_BYTES];
                    for i in 0..buf.len() {
                        buf[i] = bvec[i];
                    }
                    Ok(Hash(buf))
                }
                Err(FromBase64Error::InvalidBase64Byte(b, i)) => {
                    Err(HashDecodeError::InvalidBase64Byte(b, i))
                }
                Err(e) => {
                    unreachable!(
                        "Length precondition check inconsistency; error {:?}; input {:?}",
                        e,
                        s,
                    )
                }
            }
        } else {
            Err(HashDecodeError::InvalidLength(s.len()))
        }
    }

    pub fn encoded(&self) -> String {
        use b64::ToB64;

        self.0.to_b64()
    }
}


pub struct Hasher(Blake2b);

impl Hasher {
    pub fn new() -> Hasher {
        Hasher(Blake2b::new(HASH_BYTES))
    }

    pub fn update(&mut self, data: &[u8]) {
        self.0.update(data)
    }

    pub fn finalize(self) -> Hash {
        // FIXME: find a standard std/core routine for this:
        let mut outbytes = [0; HASH_BYTES];
        let hashout = self.0.finalize();
        let inbytes = hashout.as_bytes();

        for i in 0..HASH_BYTES {
            outbytes[i] = inbytes[i];
        }

        Hash(outbytes)
    }
}


#[cfg(test)]
mod tests {
    use hash::Hash;
    use std::fmt;

    // PartialEq and Debug only for testing:
    impl PartialEq for Hash {
        fn eq(&self, other: &Hash) -> bool {
            self.0 == other.0
        }
    }

    impl fmt::Debug for Hash {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Hash:{}", self.encoded())
        }
    }

    #[allow(non_snake_case)]
    mod Hash_type {
        use hash::{HASH_BYTES, Hash, HashDecodeError};
        const SEVENS_HASH: Hash = Hash([7; HASH_BYTES]);
        const SEVENS_ENC: &'static str = "BwcHBwcHBwcHBwcHBwcHBw";

        #[test]
        fn encoded() {
            assert_eq!(SEVENS_ENC, SEVENS_HASH.encoded());
        }

        #[test]
        fn decode_ok() {
            assert_eq!(Ok(SEVENS_HASH), Hash::decode(SEVENS_ENC));
        }

        #[test]
        fn decode_wrong_size() {
            let s = "WrongLength";
            assert_eq!(Err(HashDecodeError::InvalidLength(s.len())),
                       Hash::decode(s));
        }

        #[test]
        fn decode_bad_byte() {
            let junk = "BwcHBwcH*wcHBwcHBwcHBw";
            assert_eq!(Err(HashDecodeError::InvalidBase64Byte(b'*', 8)),
                       Hash::decode(junk));
        }
    }

    #[allow(non_snake_case)]
    mod Hasher {
        use hash::Hasher;

        // FIXME: Verify test vectors against the reference implementation
        // of blake2.

        #[test]
        fn empty() {
            use testval::EMPTY_HASH_ENC;

            let henc = Hasher::new().finalize().encoded();
            assert_eq!(EMPTY_HASH_ENC, henc);
        }

        #[test]
        fn hello_world() {
            let mut hasher = Hasher::new();
            hasher.update(b"Hello World!");
            let henc = hasher.finalize().encoded();
            assert_eq!("Gj88HM-a6ffnU1yUY7Bq_w", henc);
        }
    }
}
