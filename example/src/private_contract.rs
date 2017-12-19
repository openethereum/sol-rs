extern crate ethabi;
#[macro_use]
extern crate ethabi_derive;
#[macro_use]
extern crate ethabi_contract;
extern crate ethereum_types as types;
extern crate solaris;
extern crate rustc_hex;
extern crate rand;
extern crate secp256k1;
extern crate tiny_keccak;

fn main() {
	solaris::main(include_bytes!("../res/PrivateContract.abi"));
}

#[cfg(test)]
mod tests {
	use super::*;
	use rustc_hex::{FromHex, ToHex};
	use secp256k1::key::{SecretKey, PublicKey};

	use_contract!(private_contract, "PrivateContract", "res/PrivateContract.abi");

	pub struct Account {
		address: types::Address,
		private: SecretKey,
		public: PublicKey,
	}

	pub struct Secp256k1Parts {
		v: [u8; 32], // Uint
		r: [u8; 32], // Bytes
		s: [u8; 32], // Bytes
	}

	pub fn secp256k1_signature_parts(secret_key: &SecretKey, message_hash: &[u8]) -> Secp256k1Parts {
		use secp256k1::{Message, Secp256k1};
		use solaris::sol;

		let secp = Secp256k1::new();
		let sp_msg = &Message::from_slice(message_hash).unwrap();
		let (recid, sigdata) = secp
			.sign_recoverable(sp_msg, secret_key)
			.unwrap()
			.serialize_compact(&secp);

		let mut r = [0u8; 32];
		let mut s = [0u8; 32];

		r.copy_from_slice(&sigdata[0..32]);
		s.copy_from_slice(&sigdata[32..64]);
        let raw_v = match recid.to_i32() { 
            0 => 0,
            1 => 1,
            _ => panic!("Recovery ID should always be 0 or 1. ID 2 & 3 are invalid, qed."),
        };
        // Convert recovery id to solidity uint
        // add 27: electrum standard for negative (27) / positive (28) ec curves 
		let v: [u8; 32] = sol::raw::uint(raw_v as u64 + 27); 

		Secp256k1Parts{v: v, r: r, s: s}
	}

	pub fn create_account() -> Account { 
		use rand::os::OsRng;
		use secp256k1::Secp256k1;
		use tiny_keccak::keccak256;

		let secp = Secp256k1::new();
		let mut rng = OsRng::new().unwrap();

		let vprv = SecretKey::new(&secp, &mut rng);
		let vpub = PublicKey::from_secret_key(&secp, &vprv).unwrap();

		let mut vaddr = [0u8; 20];
		vaddr.copy_from_slice(
			&keccak256(vpub.serialize_vec(&secp, false).as_slice())[12..]
			);

		Account {
			address: vaddr.into(),
			private: vprv, 
			public: vpub,
		}
	}

    pub fn nonced_hash(data: &[u8], nonce: u64) -> [u8; 32] {
        use solaris::sol;
		use tiny_keccak::keccak256;

        let mut datanonce = Vec::new();
        datanonce.extend(keccak256(data).iter());
        datanonce.extend(sol::raw::uint(nonce).iter());

        keccak256(&datanonce)
    }

	fn setup() -> (solaris::evm::Evm, private_contract::PrivateContract, Vec<Account>) {
		let contract = private_contract::PrivateContract::default();
		let code = include_str!("../res/PrivateContract.bin");
	
		// PrivateContract initialization arguments
		let init_code = vec![];
		let init_state = vec![];
	
		let mut evm = solaris::evm();

		let mut vals: Vec<Account> = Vec::new();

		for _i in 0..3 {
			vals.push(create_account());
		}

		let owner = 666.into();
		let _address = evm.with_sender(owner).deploy(
			&contract.constructor(
				code.from_hex().unwrap(),
				vec![vals[0].address, vals[1].address, vals[2].address], 
				init_code, 
				init_state,
				)
			);
	
		(evm, contract, vals)
	}
	
	#[test]
	fn it_should_have_inited() {
		let (_evm, _contract, _validators) = setup();
	}

    #[test]
    fn it_should_set_validators() {
        use solaris::sol;

        let (mut evm, contract, validators) = setup();
		let pcon = contract.functions();

		assert_eq!(pcon.state().call(&mut evm).unwrap().to_hex(), "", "Initial State should be empty");

        for (i, val) in validators.iter().enumerate() {
            let loc_val = val.address.as_ref();
            let con_val = pcon.validators().call(sol::raw::uint(i as u64), &mut evm).unwrap();
            assert_eq!(loc_val,
                        con_val,
                        "Local validator: {}\n\t should equal\n contract validator: {}", loc_val.to_hex(), con_val.to_hex());
        }
    }

	#[test]
	fn it_should_allow_state_change_if_all_the_signatures_are_ok() {
        use solaris::sol;

		let (mut evm, contract, validators) = setup();
		let pcon = contract.functions();

		assert_eq!(pcon.state().call(&mut evm).unwrap().to_hex(), "", "Initial State should be empty");

		let new_state = "ffaabb55ffaabb55ffaabb55ffaabb55ffaabb55ffaabb55ffaabb55ffaabb55".from_hex().unwrap();
        let new_state_hash = nonced_hash(&new_state, 1);

		let mut parts: Vec<Secp256k1Parts> = Vec::new();

        let val_len = validators.len();
		for i in 0..val_len {
			parts.push(
				secp256k1_signature_parts(&validators[i].private, &new_state_hash)
			);
		}

		let mut vs: Vec<[u8; 32]> = Vec::new();
		let mut rs: Vec<[u8; 32]> = Vec::new();
		let mut ss: Vec<[u8; 32]> = Vec::new();
		for p in parts {
			vs.push(p.v);
			rs.push(p.r);
			ss.push(p.s);
		}

		pcon.set_state().transact(
			new_state.as_slice(),
			vs,
			rs,
			ss,
			&mut evm)
		.unwrap();

        let cnhash = pcon.nonced_state_hash().call(&mut evm).unwrap();
        let ns_str = new_state_hash.to_hex();
        let ch_str = cnhash.to_hex();
        assert_eq!(ns_str, ch_str,
                   "Submitted hash: {:?} \nshould equal \nContract hash: {:?}",
                   ns_str,
                   ch_str
                  );

        for (i, v) in validators.iter().enumerate() {
            let rec = pcon.recovered_address().call(sol::raw::uint(i as u64), &mut evm).unwrap();
            let rec_str = rec.to_hex();
            let vadr_str = v.address.to_hex();
            assert_eq!(rec_str, vadr_str,
                        "Recovered address: {:?} \nshould equal \nLocal validator address: {:?}",
                        rec_str,
                        vadr_str
                      );
        }
	}
}
