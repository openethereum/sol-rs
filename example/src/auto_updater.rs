extern crate ethabi;
#[macro_use]
extern crate ethabi_derive;
#[macro_use]
extern crate ethabi_contract;
extern crate ethereum_types as types;
extern crate rustc_hex;
extern crate solaris;

fn main() {
    solaris::main(include_bytes!("../res/Operations.abi"));
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use_contract!(operations, "Operations", "res/Operations.abi");

    fn setup() -> (solaris::evm::Evm, operations::Operations) {
        let contract = operations::Operations::default();
        let code = include_str!("../res/Operations.bin");
        let mut evm = solaris::evm();
    
        let owner = 303.into();
        let _address = evm.with_sender(owner).deploy(&code.from_hex().unwrap());
    
        (evm, contract)
    }

    #[test]
    fn it_should_have_inited(){
        let (mut evm, contract) = setup();
        let ops = contract.functions();

        assert_eq!(ops.clients_required().call(&mut evm).unwrap(), 1.into());
    }
}
