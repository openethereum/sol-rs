extern crate ethabi;
#[macro_use]
extern crate ethabi_derive;
#[macro_use]
extern crate ethabi_contract;

pub fn main(_json_bytes: &[u8]) {
    println!("This might be a contract CLI in the future.");
}

pub fn evm() -> Evm {
    Evm::default()
}

#[derive(Default, Debug)]
pub struct Evm {
    sender: ethabi::Address,
}

impl Evm {
    pub fn set_sender(&mut self, address: ethabi::Address) {
        self.sender = address;
    }
}

impl<'a> ethabi::Caller for &'a mut Evm {
    type CallOut = Result<ethabi::Bytes, String>;
    type TransactOut = Result<ethabi::Bytes, String>;

    fn call(self, bytes: ethabi::Bytes) -> Self::CallOut {
        unimplemented!()
    }

    fn transact(self, bytes: ethabi::Bytes) -> Self::TransactOut {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
