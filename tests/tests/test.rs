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
extern crate ethereum_types;
extern crate rustc_hex;
extern crate solaris;

use rustc_hex::FromHex;
use ethereum_types::{Address, U256};
use ethabi::Token;

use_contract!(
    get_sender_test,
    "contracts/test_sol_GetSenderTest.abi"
);

use_contract!(
    event_log_test,
    "contracts/test_sol_EventLogTest.abi"
);

#[test]
fn msg_sender_should_match_value_passed_into_with_sender() {
    let mut evm = solaris::evm();

    let contract_owner_address: Address = Address::from_low_u64_be(3);

    let code_hex = include_str!("../contracts/test_sol_GetSenderTest.bin");
    let code_bytes = code_hex.from_hex().unwrap();
    let _contract_address = evm.with_sender(contract_owner_address)
        .deploy(&code_bytes)
        .expect("contract deployment should succeed");

    use get_sender_test::functions;

    let sender = Address::from_low_u64_be(5);

    let result_data = evm.with_sender(sender)
        .call(functions::get_sender::encode_input())
        .unwrap();

    let output: Address = functions::get_sender::decode_output(&result_data)
        .unwrap();
            
    assert_eq!(output, sender);
}

use_contract!(
    get_value_test,
    "contracts/test_sol_GetValueTest.abi"
);

#[test]
fn msg_value_should_match_value_passed_into_with_value() {
    let mut evm = solaris::evm();

    let contract_owner_address: Address = Address::from_low_u64_be(3);

    let code_hex = include_str!("../contracts/test_sol_GetValueTest.bin");
    let code_bytes = code_hex.from_hex().unwrap();
    let _contract_address = evm.with_sender(contract_owner_address)
        .deploy(&code_bytes)
        .expect("contract deployment should succeed");

    use get_value_test::functions;

    let value = solaris::wei::from_ether(1);

    let result_data = evm.with_value(value)
        .ensure_funds()
        .call(functions::get_value::encode_input())
        .unwrap();
    
    let output: U256 = functions::get_value::decode_output(&result_data)
        .unwrap();

    assert_eq!(output, value);
}

#[test]
fn logs_should_get_collected_and_retrieved_correctly() {
    let code_hex = include_str!("../contracts/test_sol_EventLogTest.bin");
    let code_bytes = code_hex.from_hex().unwrap();

    let mut evm = solaris::evm();

    let contract_owner_address: Address = Address::from_low_u64_be(3);

    let _contract_address = evm.with_sender(contract_owner_address)
        .deploy(&code_bytes)
        .expect("contract deployment should succeed");

    use event_log_test::functions;

    let first_sender_address = Address::from_low_u64_be(10);
    evm.with_sender(first_sender_address)
        .transact(functions::emit_foo::encode_input())
        .unwrap();

    let second_sender_address = Address::from_low_u64_be(11);
    evm.with_sender(second_sender_address)
        .transact(functions::emit_foo::encode_input())
        .unwrap();

    evm.transact(functions::emit_bar::encode_input(100)).unwrap();
    evm.transact(functions::emit_bar::encode_input(101)).unwrap();
    evm.transact(functions::emit_bar::encode_input(102)).unwrap();

    // call should not show up in logs
    evm.call(functions::emit_foo::encode_input())
        .unwrap();

    assert_eq!(evm.raw_logs().len(), 5);

    use event_log_test::events;

    let foo_logs: Vec<event_log_test::logs::Foo> = evm.raw_logs()
        .iter()
        .filter_map(|log| events::foo::parse_log(log.clone()).ok())
        .collect();

    assert_eq!(foo_logs.len(), 2);
    assert_eq!(Address::from(foo_logs[0].sender), first_sender_address);
    assert_eq!(Address::from(foo_logs[1].sender), second_sender_address);

    let bar_logs: Vec<event_log_test::logs::Bar> = evm.raw_logs()
        .iter()
        .filter_map(|log| events::bar::parse_log(log.clone()).ok())
        .collect();

    assert_eq!(bar_logs.len(), 3);
    assert_eq!(U256::from(bar_logs[0].value), U256::from(100));
    assert_eq!(U256::from(bar_logs[1].value), U256::from(101));
    assert_eq!(U256::from(bar_logs[2].value), U256::from(102));

    let baz_logs: Vec<event_log_test::logs::Baz> = evm.raw_logs()
        .iter()
        .filter_map(|log| events::baz::parse_log(log.clone()).ok())
        .collect();

    assert_eq!(baz_logs.len(), 0);
}
