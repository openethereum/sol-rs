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
#[macro_use]
extern crate ethabi_derive;
extern crate ethereum_types as types;
extern crate rustc_hex;
extern crate solaris;

fn main() {
    solaris::main(include_bytes!("../res/BadgeReg_sol_BadgeReg.abi"));
}

use_contract!(badgereg, "BadgeReg", "res/BadgeReg_sol_BadgeReg.abi");

#[cfg(test)]
fn setup() -> (solaris::evm::Evm, badgereg::BadgeReg) {
    let contract = badgereg::BadgeReg::default();
    let code = include_str!("../res/BadgeReg_sol_BadgeReg.bin");
    let mut evm = solaris::evm();

    let owner = 3.into();
    let _address = evm.with_sender(owner).deploy(&code.from_hex().unwrap());

    (evm, contract)
}

#[cfg(test)]
use rustc_hex::FromHex;
#[cfg(test)]
use solaris::convert;
#[cfg(test)]
use solaris::wei;

#[cfg(test)]
use common_types::{Address, H256, U256};

#[test]
fn badge_reg_test_fee() {
    let (mut evm, contract) = setup();

    // Initial fee is 1 ETH
    assert_eq!(
        evm.call(contract.functions().fee()).unwrap(),
        wei::from_ether(1)
    );

    // The owner should be able to set the fee
    evm.call(contract.functions().set_fee(wei::from_gwei(10)))
        .unwrap();

    // Fee should be updated
    assert_eq!(
        evm.call(contract.functions().fee()).unwrap(),
        wei::from_gwei(10)
    );

    // Other address should not be allowed to change the fee
    evm.with_sender(10.into())
        .transact(contract.functions().set_fee(wei::from_gwei(10)))
        .unwrap();
}

#[test]
fn anyone_should_be_able_to_register_a_badge() {
    let (mut evm, contract) = setup();

    evm.with_value(wei::from_ether(2))
        .with_sender(5.into())
        .ensure_funds()
        .transact(
            contract
                .functions()
                .register(Address::from(10), convert::bytes32("test")),
        )
        .unwrap();

    assert_eq!(
        evm.logs_for_event(badgereg::events::Registered::default())
            .len(),
        1
    );

    // TODO [ToDr] Perhaps `with_` should not be persistent?
    let output = evm.with_value(0.into())
        .call(contract.functions().from_name(convert::bytes32("test")))
        .unwrap();

    // Test that it was registered correctly
    assert_eq!(
        output,
        (
            U256::from(0).into(),
            Address::from(10).into(),
            Address::from(5).into()
        )
    );
}
