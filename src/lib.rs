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

/* Library Public API: */
pub use hashstore::{
    HashInserter,
    HashStore,
};
