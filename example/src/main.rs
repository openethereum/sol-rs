extern crate ethabi;
#[macro_use]
extern crate ethabi_derive;
#[macro_use]
extern crate ethabi_contract;
extern crate solaris;

fn main() {
    solaris::main(include_bytes!("../res/eip20.abi"));
}

use_contract!(eip20, "Eip20", "res/eip20.abi");

#[test]
fn first_contract_test() {
    use eip20::Eip20;
    let contract = Eip20::default();
    let mut evm = solaris::evm();

    let a: ethabi::Address = [0u8; 20];

    // Assert initial balance of 0
    assert_eq!(contract.functions().balance_of().call(a, &mut evm).unwrap(), [0u8; 32]);

    // Perform a transaction
    evm.set_sender([1u8; 20]);
    let value: ethabi::Uint = [1u8; 32];
    assert_eq!(contract.functions().transfer().transact(a, value, &mut evm).unwrap(), ());

    // Check the balance again
    assert_eq!(contract.functions().balance_of().call(a, &mut evm).unwrap(), value);
}
