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
use types::{Address, U256};

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

    let output: Address = evm
        .with_sender(input.clone())
        .transact(fns.get_sender().input())
        .unwrap()
        .as_slice()[12..]
        .into();

    assert_eq!(output, input);
}

use_contract!(get_value_test, "GetValueTest", "contracts/test_sol_GetValueTest.abi");

#[test]
fn msg_value_should_match_value_passed_into_with_value() {
    let contract = get_value_test::GetValueTest::default();
    let code_hex = include_str!("../contracts/test_sol_GetValueTest.bin");
    let code_bytes = code_hex.from_hex().unwrap();

    let mut evm = solaris::evm();

    let contract_owner_address: Address = 3.into();

    let _contract_address = evm
        .with_sender(contract_owner_address)
        .deploy(&code_bytes)
        .expect("contract deployment should succeed");

    let fns = contract.functions();

    let input = solaris::wei::from_ether(1);

    let output: U256 = evm
        .with_value(input.clone())
        .ensure_funds()
        .call(fns.get_value().input())
        .unwrap()
        .as_slice()
        .into();

    assert_eq!(output, input);

    let output: U256 = evm
        .with_value(input.clone())
        .ensure_funds()
        .transact(fns.get_value().input())
        .unwrap()
        .as_slice()
        .into();

    assert_eq!(output, input);
}

use_contract!(event_log_test, "EventLogTest", "contracts/test_sol_EventLogTest.abi");

#[test]
fn logs_should_get_collected_and_retrieved_correctly() {
    let contract = event_log_test::EventLogTest::default();
    let code_hex = include_str!("../contracts/test_sol_EventLogTest.bin");
    let code_bytes = code_hex.from_hex().unwrap();

    let mut evm = solaris::evm();

    let contract_owner_address: Address = 3.into();

    let _contract_address = evm
        .with_sender(contract_owner_address)
        .deploy(&code_bytes)
        .expect("contract deployment should succeed");

    let fns = contract.functions();

    let first_sender_address = 10.into();
    evm
        .with_sender(first_sender_address)
        .transact(fns.emit_foo().input())
        .unwrap();

    let second_sender_address = 11.into();
    evm
        .with_sender(second_sender_address)
        .transact(fns.emit_foo().input())
        .unwrap();

    evm
        .transact(fns.emit_bar().input(U256::from(100)))
        .unwrap();
    evm
        .transact(fns.emit_bar().input(U256::from(101)))
        .unwrap();
    evm
        .transact(fns.emit_bar().input(U256::from(102)))
        .unwrap();

    // call should not show up in logs
    evm
        .call(fns.emit_foo().input())
        .unwrap();

    assert_eq!(evm.raw_logs().len(), 5);

    let foo_logs = evm.logs(contract.events().foo());
    assert_eq!(foo_logs.len(), 2);
    assert_eq!(Address::from(foo_logs[0].sender), first_sender_address);
    assert_eq!(Address::from(foo_logs[1].sender), second_sender_address);

    let bar_logs = evm.logs(contract.events().bar());
    assert_eq!(bar_logs.len(), 3);
    assert_eq!(U256::from(bar_logs[0].value), U256::from(100));
    assert_eq!(U256::from(bar_logs[1].value), U256::from(101));
    assert_eq!(U256::from(bar_logs[2].value), U256::from(102));

    let baz_logs = evm.logs(contract.events().baz());
    assert_eq!(baz_logs.len(), 0);
}
