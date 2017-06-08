#[deny(warnings)]

extern crate rustc_serialize as serialize;
extern crate blake2_rfc;

#[cfg(test)]
#[macro_use]
mod testutils;

pub mod b64;
mod hash;
mod hashspool;
mod hashstore;
mod unival;

// Library Public API:
#[cfg(not(test))]
pub use hash::{HASH_BYTES, Hash, Hasher};
pub use hashstore::{HashInserter, HashStore};
pub use unival::UniqueValue;

pub const EMPTY_HASH: &'static str = "DldRwCblQ7Loqy6wYJnaodHl30d3j3eH-qtFzfEv46g";
