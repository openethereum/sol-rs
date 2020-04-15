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
#[macro_use]
extern crate ethabi_contract;
extern crate ethabi_derive;
extern crate ethereum_types;
extern crate rustc_hex;
extern crate solaris;

fn main() {
    solaris::main(include_bytes!("../res/BadgeReg.abi"));
}

use_contract!(badgereg, "res/BadgeReg.abi");

#[cfg(test)]
fn setup() -> solaris::evm::Evm {
    let code = include_str!("../res/BadgeReg.bin");
    let mut evm = solaris::evm();

    let owner = Address::from_low_u64_be(3);
    let _address = evm.with_sender(owner).deploy(&code.from_hex().unwrap());

    evm
}

#[cfg(test)]
use rustc_hex::FromHex;
#[cfg(test)]
use solaris::convert;
#[cfg(test)]
use solaris::wei;

#[cfg(test)]
use ethereum_types::{Address, U256};

#[test]
fn badge_reg_test_fee() {
    let mut evm = setup();

    use badgereg::functions;

    let result_data = evm.call(functions::fee::encode_input()).unwrap();
    // Initial fee is 1 ETH
    assert_eq!(
        functions::fee::decode_output(&result_data).unwrap(),
        wei::from_ether(1)
    );

    // The owner should be able to set the fee
    evm.transact(functions::set_fee::encode_input(wei::from_gwei(10))).unwrap();
    
    let result_data = evm.call(functions::fee::encode_input()).unwrap();
    // Fee should be updated
    assert_eq!(
        functions::fee::decode_output(&result_data).unwrap(),
        wei::from_gwei(10)
    );

    // Other address should not be allowed to change the fee
    evm.with_sender(Address::from_low_u64_be(10))
        .transact(functions::set_fee::encode_input(wei::from_gwei(15)))
        .unwrap();

    let result_data = evm.call(functions::fee::encode_input()).unwrap();
    // Fee should not be updated
    assert_eq!(
        functions::fee::decode_output(&result_data).unwrap(),
        wei::from_gwei(10)
    );
}

#[test]
fn anyone_should_be_able_to_register_a_badge() {
    let mut evm = setup();

    use badgereg::functions;

    evm.with_value(wei::from_ether(2))
        .with_sender(Address::from_low_u64_be(5))
        .ensure_funds()
        .transact(
            functions::register::encode_input(Address::from_low_u64_be(10), convert::bytes32("test")),
        )
        .unwrap();

    use badgereg::events;

    let registerd_logs: Vec<badgereg::logs::Registered> = evm.raw_logs()
        .iter()
        .filter_map(|log| events::registered::parse_log(log.clone()).ok())
        .collect();

    assert_eq!(
        registerd_logs.len(),
        1
    );

    // TODO [ToDr] Perhaps `with_` should not be persistent?
    let result_data = evm.with_value(0.into())
        .call(functions::from_name::encode_input(convert::bytes32("test")))
        .unwrap();

    // Test that it was registered correctly
    assert_eq!(
        functions::from_name::decode_output(&result_data).unwrap(),
        (
            U256::from(0).into(),
            Address::from_low_u64_be(10).into(),
            Address::from_low_u64_be(5).into()
        )
    );
}
