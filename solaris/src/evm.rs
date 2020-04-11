// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

use std::sync::Arc;

use error;
use ethabi;
use ethabi::Function;
use ethcore::test_helpers::{EvmTestClient, TransactSuccess};
use common_types::transaction::{Action, SignedTransaction, Transaction};
use ethereum_types::{Address, H160, H256, U256};
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
    logs: Vec<common_types::log_entry::LogEntry>,
}

impl Default for Evm {
    fn default() -> Self {
        Evm::new_current()
    }
}

pub struct TransactionOutput {
    state_root: H256,
    gas_left: U256,
    output: Vec<u8>,
    contract_address: Option<H160>,
    logs: Vec<common_types::log_entry::LogEntry>,
    outcome: common_types::receipt::TransactionOutcome,
}

impl<T, V> From<TransactSuccess<T, V>> for TransactionOutput {
    fn from(t: TransactSuccess<T, V>) -> Self {
        TransactionOutput {
            state_root: t.state_root,
            gas_left: t.gas_left,
            output: t.output,
            contract_address: t.contract_address,
            logs: t.logs,
            outcome: t.outcome,
        }
    }
}

impl Evm {
    pub fn new_current() -> Self {
        let evm = EvmTestClient::new(&*::FOUNDATION).expect("Valid spec given; qed");
        Evm {
            evm,
            sender: Address::from_low_u64_be(0),
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
            number: 7_280_000u64,
            author: Address::from_low_u64_be(0),
            timestamp: 1u64,
            difficulty: 1.into(),
            last_hashes: Arc::new([H256::from_low_u64_be(0); 256].to_vec()),
            gas_used: 0.into(),
            gas_limit: 4_700_000.into(),
        }
    }

    pub fn deploy(&mut self, code: &[u8]) -> error::Result<Address> {
        let env_info = self.env_info();
        let nonce = self.evm.state().nonce(&self.sender).expect(STATE);
        let transaction = Transaction {
            nonce,
            gas_price: self.gas_price,
            gas: self.gas,
            action: Action::Create,
            value: self.value,
            data: code.to_vec(),
        }.fake_sign(self.sender);

        let transaction_output = self.raw_transact(&env_info, transaction)?;

        let contract_address = transaction_output
            .contract_address
            .expect("transaction output must have contract_address after deploy");

        self.contract_address = Some(contract_address);

        Ok(contract_address)
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

        self.raw_transact(&env_info, transaction)
            .expect("Unable to top up account.");
        self
    }

    /// returns a vector of all logs that were collected for a specific `event`.
    /// the logs are conveniently converted to the events log struct `T::Log`.
    pub fn logs_for_event(&self, event: ethabi::Event) -> Vec<ethabi::Log> {
        self.logs
            .iter()
            .filter_map(|log| event.parse_log(ethcore_log_to_ethabi_log(log)).ok())
            .collect()
    }

    /// returns a vector of all raw logs collected until now
    pub fn raw_logs(&self) -> Vec<ethabi::RawLog> {
        self.logs.iter().map(ethcore_log_to_ethabi_log).collect()
    }

    /// Run the EVM and panic on all errors.
    pub fn run<F>(self, func: F)
    where
        F: FnOnce(Self) -> ::ethabi::Result<()>,
    {
        func(self).expect("Unexpected error occured.");
    }

    pub fn call(&mut self, encoded_input: ethabi::Bytes) -> error::Result<vm::ReturnData> {
        let contract_address = self.contract_address
            .expect("Contract address is not set. Did you forget to deploy the contract?");
        let mut params = vm::ActionParams::default();
        params.sender = self.sender;
        params.origin = self.sender;
        params.address = contract_address;
        params.code_address = contract_address;
        params.code = self.evm.state().code(&contract_address).expect(STATE);
        params.data = Some(encoded_input);
        params.action_type = vm::ActionType::Call;
        params.value = vm::ActionValue::Transfer(self.value);
        params.gas = self.gas;
        params.gas_price = self.gas_price;

        let env_info = self.env_info();
        let mut tracers = self.tracers();
        let result = self.evm.call_envinfo(params, &mut tracers.0, &mut tracers.1, env_info)?;

        Ok(result.return_data)
        // let output = f.decode_output(&result.return_data).expect(
        //     "output must be decodable with `Function` that has encoded input. q.e.d.",
        // );
        // Ok(output)
    }

    fn raw_transact(
        &mut self,
        env_info: &vm::EnvInfo,
        transaction: SignedTransaction,
    ) -> error::Result<TransactionOutput> {
        let mut tracers = self.tracers();
        let transact_success = self.evm
            .transact(env_info, transaction, tracers.0, tracers.1)
            .map_err(|_| "TransactErr occurred".to_string())?;
        self.logs.extend(transact_success.logs.clone());
        Ok(transact_success.into())
    }

    pub fn transact(&mut self, encoded_input: ethabi::Bytes) -> error::Result<TransactionOutput> {
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
            data: encoded_input,
        }.fake_sign(self.sender);

        self.raw_transact(&env_info, transaction)
    }
}

const STATE: &str = "State failure.";

/// converts an `common_types::log_entry::LogEntry` to an `ethabi::RawLog`
/// since the events in a contract derived with `ethabi` can only
/// be parsed from `ethabi::RawLog` (via `event.parse_log(raw_log)`)
fn ethcore_log_to_ethabi_log(input: &common_types::log_entry::LogEntry) -> ethabi::RawLog {
    ethabi::RawLog::from((input.topics.clone(), input.data.clone()))
}