use super::blake2_rfc::blake2b::Blake2b;


const HASH_BYTES: usize = 32;


pub struct Hash([u8; HASH_BYTES]);


impl Hash {
    pub fn unwrap(self) -> [u8; HASH_BYTES]
    {
        self.0
    }

    pub fn encoded(&self) -> String
    {
        use super::b64::ToB64;

        self.0.to_b64()
    }
}


pub struct Hasher(Blake2b);

impl Hasher {
    pub fn new() -> Hasher
    {
        Hasher(Blake2b::new(HASH_BYTES))
    }

    pub fn update(&mut self, data: &[u8])
    {
        self.0.update(data)
    }

    pub fn finalize(self) -> Hash
    {
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
        use super::super::{HASH_BYTES, Hash};

        #[test]
        fn unwrap()
        {
            let bytes = [7; HASH_BYTES];
            let h = Hash(bytes.clone());
            assert_eq!(bytes, h.unwrap());
        }

        #[test]
        fn encoded()
        {
            let h = Hash([7; HASH_BYTES]);
            let enc = h.encoded();
            assert_eq!("BwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwc", enc);
        }
    }

    #[allow(non_snake_case)]
    mod Hasher {
        use super::super::Hasher;

        /* FIXME: Verify test vectors against the reference implementation
         * of blake2.
         */

        #[test]
        fn empty()
        {
            let henc = Hasher::new().finalize().encoded();
            assert_eq!("DldRwCblQ7Loqy6wYJnaodHl30d3j3eH-qtFzfEv46g", henc);
        }

        #[test]
        fn hello_world()
        {
            let mut hasher = Hasher::new();
            hasher.update(b"Hello World!");
            let henc = hasher.finalize().encoded();
            assert_eq!("v1bAco_U6c9kv69tq6uBVUEDKYze5cxNWAQzqiXpiwA", henc);
        }
    }
}
