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

use rustc_hex::FromHex;
use types::{Address, U256};

use_contract!(
    get_sender_test,
    "GetSenderTest",
    "contracts/test_sol_GetSenderTest.abi"
);

use_contract!(
    event_log_test,
    "EventLogTest",
    "contracts/test_sol_EventLogTest.abi"
);

#[test]
fn msg_sender_should_match_value_passed_into_with_sender() {
    let mut evm = solaris::evm();

    let contract_owner_address: Address = 3.into();

    let code_hex = include_str!("../contracts/test_sol_GetSenderTest.bin");
    let code_bytes = code_hex.from_hex().unwrap();
    let _contract_address = evm.with_sender(contract_owner_address)
        .deploy(&code_bytes)
        .expect("contract deployment should succeed");

    let contract = get_sender_test::GetSenderTest::default();

    let sender = 5.into();

    let output: Address = evm.with_sender(sender)
        .call(contract.functions().get_sender())
        .unwrap();

    assert_eq!(output, sender);
}

use_contract!(
    get_value_test,
    "GetValueTest",
    "contracts/test_sol_GetValueTest.abi"
);

#[test]
fn msg_value_should_match_value_passed_into_with_value() {
    let mut evm = solaris::evm();

    let contract_owner_address: Address = 3.into();

    let code_hex = include_str!("../contracts/test_sol_GetValueTest.bin");
    let code_bytes = code_hex.from_hex().unwrap();
    let _contract_address = evm.with_sender(contract_owner_address)
        .deploy(&code_bytes)
        .expect("contract deployment should succeed");

    let contract = get_value_test::GetValueTest::default();

    let value = solaris::wei::from_ether(1);

    let output: U256 = evm.with_value(value)
        .ensure_funds()
        .call(contract.functions().get_value())
        .unwrap();

    assert_eq!(output, value);
}

#[test]
fn logs_should_get_collected_and_retrieved_correctly() {
    let contract = event_log_test::EventLogTest::default();
    let code_hex = include_str!("../contracts/test_sol_EventLogTest.bin");
    let code_bytes = code_hex.from_hex().unwrap();

    let mut evm = solaris::evm();

    let contract_owner_address: Address = 3.into();

    let _contract_address = evm.with_sender(contract_owner_address)
        .deploy(&code_bytes)
        .expect("contract deployment should succeed");

    let fns = contract.functions();

    let first_sender_address = 10.into();
    evm.with_sender(first_sender_address)
        .transact(fns.emit_foo())
        .unwrap();

    let second_sender_address = 11.into();
    evm.with_sender(second_sender_address)
        .transact(fns.emit_foo())
        .unwrap();

    evm.transact(fns.emit_bar(100)).unwrap();
    evm.transact(fns.emit_bar(101)).unwrap();
    evm.transact(fns.emit_bar(102)).unwrap();

    // call should not show up in logs
    evm.call(fns.emit_foo()).unwrap();

    assert_eq!(evm.raw_logs().len(), 5);

    let foo_logs = evm.logs_for_event(contract.events().foo());
    assert_eq!(foo_logs.len(), 2);
    assert_eq!(Address::from(foo_logs[0].sender), first_sender_address);
    assert_eq!(Address::from(foo_logs[1].sender), second_sender_address);

    let bar_logs = evm.logs_for_event(contract.events().bar());
    assert_eq!(bar_logs.len(), 3);
    assert_eq!(U256::from(bar_logs[0].value), U256::from(100));
    assert_eq!(U256::from(bar_logs[1].value), U256::from(101));
    assert_eq!(U256::from(bar_logs[2].value), U256::from(102));

    let baz_logs = evm.logs_for_event(contract.events().baz());
    assert_eq!(baz_logs.len(), 0);
}
