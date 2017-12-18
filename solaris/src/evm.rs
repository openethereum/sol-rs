use std::sync::Arc;

use ethabi;
use ethcore;
use ethcore::client::{EvmTestClient, TransactResult};
use ethcore_transaction::{Transaction, Action, SignedTransaction};
use ethereum_types::{U256, Address};
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
            gas: 4_000_000.into(),
            gas_price: 0.into(),
            value: 0.into(),
            logs: vec![],
        }
    }

    fn tracers(&self) -> (trace::PrintingTracer, trace::PrintingTracer) {
        Default::default()
    }

    fn env_info(&self) -> vm::EnvInfo {
        vm::EnvInfo {
            number: 5_000_000u64,
            author: 0.into(),
            timestamp: 1u64,
            difficulty: 1.into(),
            last_hashes: Arc::new([0.into(); 256].to_vec()),
            gas_used: 0.into(),
            gas_limit: 4_700_000.into(),
        }
    }

    pub fn deploy(&mut self, code: &[u8]) -> Result<Address, String> {
        let env_info = self.env_info();
        let nonce = self.evm.state().nonce(&self.sender).expect(STATE);
        let transaction = Transaction {
            nonce,
            gas_price: self.gas_price,
            gas: self.gas,
            action: Action::Create,
            value: self.value,
            data: code.to_vec(),
        }.fake_sign((&*self.sender).into());

        self.evm_transact(&env_info, transaction, true, |s, _output, contract_address| {
            s.contract_address = contract_address;
            s.contract_address.ok_or_else(|| "Contract address missing.".into())
        })
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
        let transaction = Transaction {
            nonce,
            gas_price: 0.into(),
            // supplying a bit more than 21k if people use builtin addresses as destinations.
            // builtins have different pricing schemes.
            gas: 22_000.into(),
            action: Action::Call(self.sender),
            value: self.value,
            data: vec![],
        }.fake_sign(sender);

        self.evm_transact(&env_info, transaction, false, |_, _, _| Ok(()))
            .expect("Unable to top up account.");
        self
    }

    pub fn logs(&self) -> Vec<ethcore::log_entry::LogEntry> {
        self.logs.clone()
    }

    /// Run the EVM and panic on all errors.
    pub fn run<F>(self, func: F) where
        F: FnOnce(Self) -> ::ethabi::Result<()>,
    {
        func(self).expect("Unexpected error occured.");
    }

    fn evm_transact<O, F>(
        &mut self,
        env_info: &vm::EnvInfo,
        transaction: SignedTransaction,
        with_tracing: bool,
        result: F,
    ) -> Result<O, String> where
        F: FnOnce(&mut Self, Vec<u8>, Option<Address>) -> Result<O, String>,
    {
        let evm_result = if with_tracing {
            let tracers = self.tracers();
            match self.evm.transact(&env_info, transaction, tracers.0, tracers.1) {
                TransactResult::Ok { output, gas_left, logs, outcome, contract_address, .. } => {
                    self.logs.extend(logs);
                    Ok((output, gas_left, outcome, contract_address))
                },
                e => Err(format!("EVM Error: {:?}", e)),
            }
        } else {
            match self.evm.transact(&env_info, transaction, ethcore::trace::NoopTracer, ethcore::trace::NoopVMTracer) {
                TransactResult::Ok { output, gas_left, outcome, contract_address, .. } => {
                    Ok((output, gas_left, outcome, contract_address))
                },
                e => Err(format!("EVM Error: {:?}", e)),
            }
        }?;

        let (output, gas_left, outcome, contract_address) = evm_result;
        let contract_address = contract_address.map(|x| (&*x).into());
        match outcome {
            ethcore::receipt::TransactionOutcome::Unknown |
            ethcore::receipt::TransactionOutcome::StateRoot(_) => {
                // TODO [ToDr] Shitty detection of failed calls?
                if gas_left > 0.into() {
                    result(self, output, contract_address)
                } else {
                    Err(format!("Call went out of gas."))
                }
            },
            ethcore::receipt::TransactionOutcome::StatusCode(status) => {
                if status == 1 {
                    result(self, output, contract_address)
                } else {
                    Err(format!("Call failed with status code: {}", status))
                }
            },
        }
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
        params.sender = self.sender;
        params.origin = self.sender;
        params.address = contract_address;
        params.code_address = contract_address;
        params.code = self.evm.state()
            .code(&contract_address).expect(STATE);
        params.data = Some((&*bytes).into());
        params.call_type = vm::CallType::Call;
        params.value = vm::ActionValue::Transfer(self.value);
        params.gas = self.gas;
        params.gas_price = self.gas_price;

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
        let nonce = self.evm.state().nonce(&self.sender).expect(STATE);
        let transaction = Transaction {
            nonce,
            gas_price: self.gas_price,
            gas: self.gas,
            action: Action::Call(contract_address),
            value: self.value,
            data: bytes.to_vec(),
        }.fake_sign(self.sender);

        self.evm_transact(&env_info, transaction, true, |_, output, _| Ok(output))
    }
}

/// converts an `ethcore::log_entry::LogEntry` to an `ethabi::RawLog`
/// since the events in a contract derived with `ethabi` can only
/// be parsed from `ethabi::RawLog` (via `event.parse_log(raw_log`).
fn log_entry_to_raw_log(log_entry: &ethcore::log_entry::LogEntry) -> ethabi::RawLog {
	let topics: Vec<ethabi::Hash> = log_entry.topics.iter().map(|x| x.0).collect();
	ethabi::RawLog::from((topics, log_entry.data.clone()))
}

/// we should probably move this inside the `Topic` type in the `ethabi` crate.
fn is_in_topic<T: PartialEq>(topic: &ethabi::Topic<T>, maybe_value: Option<&T>) -> bool {
    match (topic, maybe_value) {
        (&ethabi::Topic::Any, None) => true,
        (&ethabi::Topic::OneOf(ref one_of), Some(value)) => one_of.contains(value),
        (&ethabi::Topic::This(ref this), Some(value)) => this == value,
        _ => false
    }
}

/// we should probably move this inside the `TopicFilter` type in the `ethabi` crate
fn is_log_in_filter(filter: &ethabi::TopicFilter, raw_log: &ethabi::RawLog) -> bool {
    is_in_topic(&filter.topic0, raw_log.topics.get(0)) &&
    is_in_topic(&filter.topic1, raw_log.topics.get(1)) &&
    is_in_topic(&filter.topic2, raw_log.topics.get(2)) &&
    is_in_topic(&filter.topic3, raw_log.topics.get(3))
}
