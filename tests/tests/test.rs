extern crate ethabi;
#[macro_use]
extern crate ethabi_derive;
#[macro_use]
extern crate ethabi_contract;
extern crate ethereum_types as types;
extern crate rustc_hex;
extern crate solaris;

use rustc_hex::FromHex;
use ethabi::Caller;
use types::{Address};

use_contract!(get_sender_test, "GetSenderTest", "contracts/test_sol_GetSenderTest.abi");

#[test]
fn msg_sender_should_match_value_passed_into_with_sender() {
	let contract = get_sender_test::GetSenderTest::default();
	let code_hex = include_str!("../contracts/test_sol_GetSenderTest.bin");
	let code_bytes = code_hex.from_hex().unwrap();

	let mut evm = solaris::evm();

	let contract_owner_address: Address = 3.into();

	let _contract_address = evm
		.with_sender(contract_owner_address)
		.deploy(&code_bytes)
		.expect("contract deployment should succeed");

	let fns = contract.functions();

	let input: Address = 5.into();

	let output: Address = evm
		.with_sender(input.clone())
		.call(fns.get_sender().input())
		.unwrap()
		.as_slice()
		.into();

	assert_eq!(output, input);

	let output: Address = evm
		.with_sender(input.clone())
		.transact(fns.get_sender().input())
		.unwrap()
		.as_slice()
		.into();

	assert_eq!(output, input);
}
