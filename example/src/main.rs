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
use solaris::wei;
#[cfg(test)]
use solaris::convert;

#[cfg(test)]
use types::{Address, H256, U256};

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
        evm.logs(badgereg::events::Registered::default()).len(),
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

    Ok(())
}
