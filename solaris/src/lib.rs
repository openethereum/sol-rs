extern crate ethabi;
extern crate ethcore;
extern crate ethcore_bigint as bigint;
extern crate ethcore_bytes;
extern crate ethereum_types as types;
extern crate evm as ethcore_evm;
extern crate vm;

#[macro_use]
extern crate lazy_static;

mod trace;

pub mod evm;
pub mod sol;
pub mod wei;

lazy_static! {
    pub static ref FOUNDATION: ethcore::spec::Spec = ethcore::ethereum::new_foundation(&::std::env::temp_dir());
}

pub fn main(_json_bytes: &[u8]) {
    println!("This might be a contract CLI in the future.");
}

pub fn evm() -> evm::Evm {
    evm::Evm::default()
}

