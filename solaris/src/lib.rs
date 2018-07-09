// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

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

pub mod convert;
pub mod error;
pub mod evm;
pub mod wei;

lazy_static! {
    pub static ref FOUNDATION: ethcore::spec::Spec =
        ethcore::ethereum::new_foundation(&::std::env::temp_dir());
}

pub fn main(_json_bytes: &[u8]) {
    println!("This might be a contract CLI in the future.");
}

pub fn evm() -> evm::Evm {
    evm::Evm::default()
}
