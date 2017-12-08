use std::sync::Arc;

use bigint;
use ethabi;
use ethcore::client::{self, EvmTestClient, TransactResult};
use ethcore::{self, transaction};
use types::{Address, U256};
use vm;

use trace;

#[derive(Debug)]
pub struct Evm {
    evm: EvmTestClient<'static>,
    sender: Address,
    contract_address: Option<Address>,
    value: U256,
    gas: U256,
    gas_price: U256,
    logs: Vec<ethcore::log_entry::LogEntry>,
}

impl Default for Evm {
    fn default() -> Self {
        Evm::new_current()
    }
}

impl Evm {
    pub fn new_current() -> Self {
        let evm = EvmTestClient::new(&*::FOUNDATION).expect("Valid spec given; qed");
        Evm {
            evm,
            sender: 0.into(),
            contract_address: None,
            gas: 1_000_000.into(),
            gas_price: 0.into(),
            value: 0.into(),
            logs: vec![],
        }
    }

    fn tracers(&self) -> (trace::PrintingTracer, trace::PrintingTracer) {
        Default::default()
    }

    fn env_info(&self) -> client::EnvInfo {
        client::EnvInfo {
            number: 5_000_000u64,
            author: 0.into(),
            timestamp: 1u64,
            difficulty: 1.into(),
            last_hashes: Arc::new([0.into(); 256].to_vec()),
            gas_used: 0.into(),
            gas_limit: 4_700_000.into(),
        }
    }

    pub fn deploy(&mut self, code: &[u8]) -> Result<Address, ()> {
        let env_info = self.env_info();
        let nonce = self.evm.state().nonce(&convert_address(self.sender)).expect(STATE);
        let transaction = transaction::Transaction {
            nonce,
            gas_price: convert_u256(self.gas_price),
            gas: convert_u256(self.gas),
            action: transaction::Action::Create,
            value: convert_u256(self.value),
            data: code.to_vec(),
        }.fake_sign((&*self.sender).into());

        let tracers = self.tracers();
        match self.evm.transact(&env_info, transaction, tracers.0, tracers.1) {
            TransactResult::Ok { contract_address, .. } => {
                self.contract_address = contract_address.map(|x| (&*x).into());
                Ok(Default::default())
            },
            err @ TransactResult::Err { .. } => {
                println!("Unable to deploy contract: {:?}", err);
                Err(())
            },
        }
    }

    pub fn with_gas(&mut self, gas: U256) -> &mut Self {
        self.gas = gas;
        self
    }

    pub fn with_gas_price(&mut self, gas_price: U256) -> &mut Self {
        self.gas_price = gas_price;
        self
    }

    pub fn with_value(&mut self, value: U256) -> &mut Self {
        self.value = value;
        self
    }

    pub fn with_sender(&mut self, address: Address) -> &mut Self {
        self.sender = address;
        self
    }

    /// Ensures that sender has enough funds (value) to call next transaction.
    pub fn ensure_funds(&mut self) -> &mut Self {
        // TODO [ToDr] Just transfer to amount that is actually needed
        let env_info = self.env_info();
        let sender = "7c532DB9E0c06C26fd40Acc56AC55C1eE92D3C3A".parse().unwrap();
        let nonce = self.evm.state().nonce(&sender).expect(STATE);
        let transaction = transaction::Transaction {
            nonce,
            gas_price: 0.into(),
            gas: 21_000.into(),
            action: transaction::Action::Call(convert_address(self.sender)),
            value: convert_u256(self.value),
            data: vec![],
        }.fake_sign(sender);

        match self.evm.transact(&env_info, transaction, ethcore::trace::NoopTracer, ethcore::trace::NoopVMTracer) {
            e @ TransactResult::Err { .. } => panic!("Unable to top-up account: {:?}", e),
            _ => {},
        }

        self
    }

    pub fn logs(&self, _filter: ::ethabi::TopicFilter) -> Vec<()> {
        // TODO [ToDr] Add filter querying
        self.logs.iter().map(|_| ()).collect()
    }

    /// Run the EVM and panic on all errors.
    pub fn run<F>(self, func: F) where
        F: FnOnce(Self) -> ::ethabi::Result<()>,
    {
        func(self).expect("Unexpected error occured.");
    }
}

const STATE: &str = "State failure.";

impl<'a> ethabi::Caller for &'a mut Evm {
    type CallOut = Result<ethabi::Bytes, String>;
    type TransactOut = Result<ethabi::Bytes, String>;

    fn call(self, bytes: ethabi::Bytes) -> Self::CallOut {
        let contract_address = self.contract_address
            .expect("Contract address is not set. Did you forget to deploy the contract?");
        let mut params = vm::ActionParams::default();
        params.sender = convert_address(self.sender);
        params.origin = convert_address(self.sender);
        params.address = convert_address(contract_address);
        params.code_address = convert_address(contract_address);
        params.code = self.evm.state()
            .code(&convert_address(contract_address)).expect(STATE);
        params.data = Some((&*bytes).into());
        params.call_type = vm::CallType::Call;
        params.value = vm::ActionValue::Transfer(convert_u256(self.value));
        params.gas = convert_u256(self.gas);
        params.gas_price = convert_u256(self.gas_price);

        let mut tracers = self.tracers();
        let result = self.evm.call(params, &mut tracers.0, &mut tracers.1);

        match result {
            Ok(result) => {
                Ok((&*result.return_data).into())
            },
            Err(err) => {
                // TODO [ToDr] Nice errors.
                Err(format!("Unexpected error: {:?}", err))
            },
        }
    }

    fn transact(self, bytes: ethabi::Bytes) -> Self::TransactOut {
        let contract_address = self.contract_address
            .expect("Contract address is not set. Did you forget to deploy the contract?");
        let env_info = self.env_info();
        let nonce = self.evm.state().nonce(&convert_address(self.sender)).expect(STATE);
        let transaction = transaction::Transaction {
            nonce,
            gas_price: convert_u256(self.gas_price),
            gas: convert_u256(self.gas),
            action: transaction::Action::Call(convert_address(contract_address)),
            value: convert_u256(self.value),
            data: bytes.to_vec(),
        }.fake_sign(convert_address(self.sender));

        let tracers = self.tracers();
        match self.evm.transact(&env_info, transaction, tracers.0, tracers.1) {
            TransactResult::Ok { output, gas_left, logs, outcome, .. } => {
                self.logs.extend(logs);

                match outcome {
                    ethcore::receipt::TransactionOutcome::Unknown |
                    ethcore::receipt::TransactionOutcome::StateRoot(_) => {
                        // TODO [ToDr] Shitty detection of failed calls?
                        if gas_left > 0.into() {
                            Ok(output)
                        } else {
                            Err(format!("Call went out of gas."))
                        }
                    },
                    ethcore::receipt::TransactionOutcome::StatusCode(status) => {
                        if status == 1 {
                            Ok(output)
                        } else {
                            Err(format!("Call failed with status code: {}", status))
                        }
                    },
                }
            },
            err => {
                Err(format!("EVM Error: {:?}", err))
            },
        }
    }
}

fn convert_u256(x: U256) -> bigint::uint::U256 {
    let mut bytes = [0; 32];
    x.to_big_endian(&mut bytes);
    bytes.into()
}

fn convert_address(x: Address) -> bigint::hash::H160 {
    (&*x).into()
}
