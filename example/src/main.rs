extern crate ethabi;
#[macro_use]
extern crate ethabi_derive;
#[macro_use]
extern crate ethabi_contract;
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
use solaris::{U256, Address};

#[cfg(test)]
use ethabi::DelegateCall;

#[test]
fn badge_reg_test_fee() {
	let (mut evm, contract) = setup();
	let reg = contract.functions();

	// Initial fee is 1 ETH
	assert_eq!(U256::from(reg.fee().call(&mut evm).unwrap().as_slice()), wei::from_ether(1));

	// The owner should be able to set the fee
	reg.set_fee(wei::from_gwei(10)).transact(&mut evm).unwrap();

	// Fee should be updated
	assert_eq!(U256::from(reg.fee().call(&mut evm).unwrap().as_slice()), wei::from_gwei(10));

	// Other address should not be allowed to change the fee
	evm.with_sender(10.into());
	reg.set_fee(wei::from_gwei(10)).transact(&mut evm).unwrap_err();
}

#[test]
fn anyone_should_be_able_to_register_a_badge() {
	let (evm, contract) = setup();
	let reg = contract.functions();

	evm.run(move |mut evm| {
		// Register new entry
		reg.register(Address::from(10), convert::bytes32("test")).transact(
			evm
			.with_value(wei::from_ether(2))
			.with_sender(5.into())
			.ensure_funds()
		)?;

		// TODO [ToDr] The API here is crap, we need to work on sth better.
		// Check that the event has been fired.
		assert_eq!(
			evm.logs(badgereg::events::Registered::default().create_filter(
					convert::bytes32("test"),
					ethabi::Topic::Any,
					)).len(),
					1
				  );

		// TODO [ToDr] Perhaps `with_` should not be persistent?
		evm.with_value(0.into());
		// Test that it was registered correctly
		assert_eq!(
			reg.from_name(convert::bytes32("test")).call(&mut evm)?,
			(
				U256::from(0).into(),
				Address::from(10).into(),
				Address::from(5).into()
			)
		);

		Ok(())
	})
}
