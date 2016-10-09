use blake2_rfc::blake2b::Blake2b;


const HASH_BYTES: usize = 32;


pub struct Hash([u8; HASH_BYTES]);


impl Hash {
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
    #[allow(non_snake_case)]
    mod Hash {
        use hash::{HASH_BYTES, Hash};

        #[test]
        fn encoded() {
            let h = Hash([7; HASH_BYTES]);
            let enc = h.encoded();
            assert_eq!("BwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwc", enc);
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
            assert_eq!("v1bAco_U6c9kv69tq6uBVUEDKYze5cxNWAQzqiXpiwA", henc);
        }
    }
}
