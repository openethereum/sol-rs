# Solaris

Solidity Smart Contracts unit test harness written in Rust using the native Parity Client EVM.

## Benefits

+ Statically typed tests in Rust:
  - changes in test code are checked against Rust's typesystem
+ Using native Parity EVM: 
  - latest stuff from Parity
  - contracts will run exactly the same as on a Parity full node
+ High performance:
  - no unnecessary overhead (only the bits you need)
  - directly run EVM bytecode on the Parity EVM interpreter
+ Directly import Solidity contracts
  - Solaris directly converts the ABI + bytecode of compiled Solidity contracts into native Rust code

## Goals

+ Emulate contract behavior once it has been successfully deployed to a Parity full node
+ Provide developers a lightweight, simple API for directly testing smart contracts
+ Modular design as a pluggable backend
  - At time of writing, Solaris depends heavily on the monolithic `parity-ethcore` crate
  - Solaris will eventually be deployed as a stand-alone crate, with minimal dependencies 
+ Singularity in purpose as a unit testing framework
+ Interoperability with other test frameworks (common input/output formats?)
  - JSON
  - YAML

## Out-of-scope

+ Solaris is not a full integration test framework, separating itself from frameworks like Truffle and Hive
+ Consensus rules
  - Solaris is only concerned with how a contract behaves once successfully deployed on a blockchain
  - Consensus rules determine which blockchain history is valid, not proper contract execution
+ Differences in EVM implementations
  - Subtle differences in various EVM implementations *may* lead to different behavior (which would lead to consensus bugs)
  - Validating Parity's EVM interpreter implementation is a separate and important project
  - Other tools, like Hive and KEVM are better suited for benchmarking/testing EVM implementations 
+ Contracts depending on hardcoded blockchain information
+ Contracts that require hard-coded, on-chain information will need to either:
  - mock the on-chain information
  - ignore features requiring on-chain information
