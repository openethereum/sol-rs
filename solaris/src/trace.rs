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

use ethcore_trace;
use parity_bytes::{Bytes, ToPretty};
use ethereum_types::{Address, H160, U256};
use vm;
use vm::{Error as VmError, ActionParams};

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

impl ethcore_trace::Tracer for PrintingTracer {
    type Output = ();

    fn prepare_trace_call(&mut self, params: &ActionParams, depth: usize, _is_builtin: bool) {
        println!(
            "{d}CALL ({from} --{value:?}--> {to}), data: {data}",
            d = depth,
            from = params.sender,
            value = params.value,
            to = params.address,
            data = bytes_as_str(&params.data),
        );
    }

    fn prepare_trace_create(&mut self, params: &vm::ActionParams) {
        println!(
            "{d}CREATE ({from} --{value:?}--> NEW), data: {data}",
            d = self.depth(),
            from = params.sender,
            value = params.value,
            data = bytes_as_str(&params.data),
        );
    }

    fn done_trace_call(&mut self, _gas_used: U256, output: &[u8]) {
        println!("DONE TRACE CALL Output: {:?}", output);
    }

	fn done_trace_create(&mut self, _gas_used: U256, _code: &[u8], address: Address) {
        println!("DONE TRACE CREATE At: {}", address);
    }

	fn done_trace_failed(&mut self, error: &VmError) {
        println!("DONE TRACE FAILED: {}", error);
    }

    fn trace_suicide(&mut self, _address: H160, _balance: U256, _refund_address: H160) {}

    fn trace_reward(&mut self, _author: H160, _value: U256, _reward_type: ethcore_trace::RewardType) {}

    fn drain(self) -> Vec<Self::Output> {
        vec![]
    }
}

impl ethcore_trace::VMTracer for PrintingTracer {
    type Output = ();

    fn trace_next_instruction(&mut self, pc: usize, instruction: u8, _current_gas: U256) -> bool {
        self.pc = pc;
        self.instruction = instruction;
        true
    }

    fn trace_executed(
        &mut self,
        gas_used: U256,
        stack_push: &[U256],
        _mem: &[u8],
    ) {
        if !self.vm_enabled {
            return;
        }

        if let Some(instruction) = ::ethcore_evm::Instruction::from_u8(self.instruction) {

            let info = instruction.info();

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
    }

    fn prepare_subtrace(&mut self, _code: &[u8]) {
        let mut vm = PrintingTracer::default();
        vm.vm_enabled = self.vm_enabled;
        vm.depth = self.depth + 1;
    }

    fn done_subtrace(&mut self) {}

    fn drain(self) -> Option<Self::Output> {
        None
    }
}
