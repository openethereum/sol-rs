extern crate ethabi;
#[macro_use]
extern crate ethabi_contract;
#[macro_use]
extern crate ethabi_derive;
extern crate ethereum_types as types;
extern crate rustc_hex;
extern crate solaris;

use rustc_hex::FromHex;
use ethabi::Caller;
use types::{Address, U256};

use_contract!(
    get_sender_test,
    "GetSenderTest",
    "contracts/test_sol_GetSenderTest.abi"
);

#[test]
fn msg_sender_should_match_value_passed_into_with_sender() {
    let contract = get_sender_test::GetSenderTest::default();
    let code_hex = include_str!("../contracts/test_sol_GetSenderTest.bin");
    let code_bytes = code_hex.from_hex().unwrap();

    let mut evm = solaris::evm();

    let contract_owner_address: Address = 3.into();

    let _contract_address = evm.with_sender(contract_owner_address)
        .deploy(&code_bytes)
        .expect("contract deployment should succeed");

    let fns = contract.functions();

    let input: Address = 5.into();

    let output: Address = evm
		.with_sender(input.clone())
		.call(fns.get_sender().input())
		.unwrap()
		// TODO [snd]
		// if the return type is an address ethabi currently returns a 32 byte
		// vector with the address in the last 20 bytes.
		// naively converting it into an Address takes the first 20 bytes
		// which results in a different address from the one returned by the contract.
		// modify ethabi so it returns an Address if that's the return type
		// and the following 2 unintuitive lines can be removed.
		.as_slice()[12..]
        .into();

    assert_eq!(output, input);

    let output: Address = evm.with_sender(input.clone())
        .transact(fns.get_sender().input())
        .unwrap()
        .as_slice()[12..]
        .into();

    assert_eq!(output, input);
}

use_contract!(
    get_value_test,
    "GetValueTest",
    "contracts/test_sol_GetValueTest.abi"
);

#[test]
fn msg_value_should_match_value_passed_into_with_value() {
    let contract = get_value_test::GetValueTest::default();
    let code_hex = include_str!("../contracts/test_sol_GetValueTest.bin");
    let code_bytes = code_hex.from_hex().unwrap();

    let mut evm = solaris::evm();

    let contract_owner_address: Address = 3.into();

    let _contract_address = evm.with_sender(contract_owner_address)
        .deploy(&code_bytes)
        .expect("contract deployment should succeed");

    let fns = contract.functions();

    let input = solaris::wei::from_ether(1);

    let output: U256 = evm.with_value(input.clone())
        .ensure_funds()
        .call(fns.get_value().input())
        .unwrap()
        .as_slice()
        .into();

    assert_eq!(output, input);

    let output: U256 = evm.with_value(input.clone())
        .ensure_funds()
        .transact(fns.get_value().input())
        .unwrap()
        .as_slice()
        .into();

    assert_eq!(output, input);
}
