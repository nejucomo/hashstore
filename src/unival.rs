// A Token is a high-entropy value, large enough to be resistant to
// birthday attack collisions. They may be randomly generated, the output
// of secure hashes, and used as symmetric cipher keys.
//
// This abstracts away size, encoding, and constant time comparison from
// application code. This is not a user abstraction boundary, since
// encoding and size and user-facing.
use std::io;


const TOKEN_BYTES: usize = 32;


pub struct UniqueValue([u8; TOKEN_BYTES]);

impl UniqueValue {
    pub fn generate() -> io::Result<UniqueValue> {
        use std::fs;
        use std::io::Read;

        let mut bytes = [0; TOKEN_BYTES];
        let mut f = try!(fs::File::open("/dev/urandom"));

        let n = try!(f.read(&mut bytes));
        if n == TOKEN_BYTES {
            Ok(UniqueValue(bytes))
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData,
                               format!("short urandom read: wanted {}, got {}", TOKEN_BYTES, n)))
        }
    }

    pub fn encoded(&self) -> String {
        use b64::ToB64;

        self.0.to_b64()
    }
}
