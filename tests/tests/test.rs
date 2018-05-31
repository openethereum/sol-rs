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

    let output: Address = evm
        .with_sender(sender)
        .call(contract.functions().get_sender())
        .unwrap();

    assert_eq!(output, sender);
}

use_contract!(get_value_test, "GetValueTest", "contracts/test_sol_GetValueTest.abi");

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

    let output: U256 = evm
        .with_value(value)
        .ensure_funds()
        .call(contract.functions().get_value())
        .unwrap();

    assert_eq!(output, value);
}
