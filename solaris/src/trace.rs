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

use ethcore::trace;
use ethcore::trace::trace::{Call, Create};
use parity_bytes::{Bytes, ToPretty};
use ethereum_types::{H160, U256};
use vm;

#[derive(Debug)]
pub struct PrintingTracer {
    vm_enabled: bool,
    depth: usize,
    pc: usize,
    instruction: u8,
    stack: Vec<U256>,
}

impl Default for PrintingTracer {
    fn default() -> Self {
        let vm_enabled = ::std::env::var("SOLARIS_VM_TRACES").is_ok();
        PrintingTracer {
            vm_enabled,
            depth: 0,
            pc: 0,
            instruction: 0,
            stack: Vec::new(),
        }
    }
}

fn u256_as_str(v: &U256) -> String {
    if v.is_zero() {
        "0x0".into()
    } else {
        format!("{:x}", v)
    }
}

fn bytes_as_str(v: &Option<Bytes>) -> String {
    match *v {
        Some(ref b) => b.to_hex(),
        None => "none".into(),
    }
}

impl PrintingTracer {
    fn stack(&self) -> String {
        let items = self.stack.iter().map(u256_as_str).collect::<Vec<_>>();
        format!("[{}]", items.join(","))
    }

    fn depth(&self) -> String {
        let mut s = String::new();
        for _ in 0..self.depth {
            s.push(' ');
        }
        s
    }
}

impl trace::Tracer for PrintingTracer {
    type Output = ();

    fn prepare_trace_call(&self, params: &vm::ActionParams) -> Option<Call> {
        println!(
            "{d}CALL ({from} --{value:?}--> {to}), data: {data}",
            d = self.depth(),
            from = params.sender,
            value = params.value,
            to = params.address,
            data = bytes_as_str(&params.data),
        );
        None
    }

    fn prepare_trace_create(&self, params: &vm::ActionParams) -> Option<Create> {
        println!(
            "{d}CREATE ({from} --{value:?}--> NEW), data: {data}",
            d = self.depth(),
            from = params.sender,
            value = params.value,
            data = bytes_as_str(&params.data),
        );
        None
    }

    fn prepare_trace_output(&self) -> Option<Bytes> {
        None
    }

    fn trace_call(
        &mut self,
        _call: Option<Call>,
        _gas_used: U256,
        output: Option<Bytes>,
        _subs: Vec<Self::Output>,
    ) {
        println!("{}<--Output: {} ", self.depth(), bytes_as_str(&output));
    }

    /// Stores trace create info.
    fn trace_create(
        &mut self,
        _create: Option<Create>,
        _gas_used: U256,
        _code: Option<Bytes>,
        address: H160,
        _subs: Vec<Self::Output>,
    ) {
        println!("{}<--At: {}", self.depth(), address);
    }

    fn trace_failed_call(
        &mut self,
        _call: Option<Call>,
        _subs: Vec<Self::Output>,
        error: trace::TraceError,
    ) {
        println!("{}CALL FAILED: {:?}", self.depth(), error);
    }

    fn trace_failed_create(
        &mut self,
        _create: Option<Create>,
        _subs: Vec<Self::Output>,
        error: trace::TraceError,
    ) {
        println!("{}CREATE FAILED: {:?}", self.depth(), error);
    }

    fn trace_suicide(&mut self, _address: H160, _balance: U256, _refund_address: H160) {}

    fn trace_reward(&mut self, _author: H160, _value: U256, _reward_type: trace::RewardType) {}

    fn subtracer(&self) -> Self
    where
        Self: Sized,
    {
        let mut vm = PrintingTracer::default();
        vm.vm_enabled = self.vm_enabled;
        vm.depth = self.depth + 1;
        vm
    }

    fn drain(self) -> Vec<Self::Output> {
        vec![]
    }
}

impl trace::VMTracer for PrintingTracer {
    type Output = ();

    fn trace_next_instruction(&mut self, pc: usize, instruction: u8, current_gas: U256) -> bool {
        self.pc = pc;
        self.instruction = instruction;
        true
    }

    fn trace_executed(
        &mut self,
        gas_used: U256,
        stack_push: &[U256],
        _mem_diff: Option<(usize, &[u8])>,
        _store_diff: Option<(U256, U256)>,
    ) {
        if !self.vm_enabled {
            return;
        }

        let info = ::ethcore_evm::INSTRUCTIONS[self.instruction as usize];

        let len = self.stack.len();
        self.stack
            .truncate(if len > info.args { len - info.args } else { 0 });
        self.stack.extend_from_slice(stack_push);

        println!(
            "{}[{}] {}({:x}) stack_after: {}, gas_left: {}",
            self.depth(),
            self.pc,
            info.name,
            self.instruction,
            self.stack(),
            gas_used,
        );
    }

    fn prepare_subtrace(&self, _code: &[u8]) -> Self
    where
        Self: Sized,
    {
        let mut vm = PrintingTracer::default();
        vm.vm_enabled = self.vm_enabled;
        vm.depth = self.depth + 1;
        vm
    }

    fn done_subtrace(&mut self, _sub: Self) {}

    fn drain(self) -> Option<Self::Output> {
        None
    }
}
