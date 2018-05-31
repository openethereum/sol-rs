extern crate ethabi;
extern crate ethcore;
extern crate ethcore_bytes;
extern crate ethcore_transaction;
extern crate ethereum_types;
extern crate evm as ethcore_evm;
extern crate vm;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate lazy_static;

/// re-export these for now since they provide useful conversion from
/// integer primitives and to byte arrays (which are required by the ethabi for now)
pub use ethereum_types::{Address, U256};

mod trace;

pub mod evm;
pub mod convert;
pub mod wei;
pub mod error;

lazy_static! {
    pub static ref FOUNDATION: ethcore::spec::Spec = ethcore::ethereum::new_foundation(&::std::env::temp_dir());
}

pub fn main(_json_bytes: &[u8]) {
    println!("This might be a contract CLI in the future.");
}

pub fn evm() -> evm::Evm {
    evm::Evm::default()
}
