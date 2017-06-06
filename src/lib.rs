#[deny(warnings)]

extern crate rustc_serialize as serialize;
extern crate blake2_rfc;

#[cfg(test)]
#[macro_use]
mod testutils;

mod b64;
mod hash;
mod hashspool;
mod hashstore;
mod unival;

// Library Public API:
#[cfg(not(test))]
pub use hash::{Hash, Hasher};
pub use hashstore::{HashInserter, HashStore};
pub use unival::UniqueValue;

pub const EMPTY_HASH: &'static str = "yuZpQdnvvUBOTYh1jqZ2cA";
