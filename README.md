# Solaris

Solidity Smart Contracts unit test harness written in Rust using the native Parity Client EVM.

## Benefits

+ Statically typed tests in Rust:
  - Because test harnesses are written in Rust, changes in harness code are checked against Rust's typesystem
  - Contracts are compiled using solcjs or solc, the standard for deployed smart contracts
+ Using native Parity EVM: 
  - latest stuff from Parity
  - contracts will run exactly the same as on a Parity full node
+ High performance:
  - no unnecessary overhead (only the bits you need)
  - directly run EVM bytecode on the Parity EVM interpreter
+ Directly import Solidity contracts
  - Solaris directly converts the ABI + bytecode of compiled Solidity contracts into native Rust code
+ IDE-agnostic
  - Solaris does not care what development path is followed
  - As long as the contract can produce a valid binary and ABI, Solaris can test it

## Goals

+ Emulate contract behavior once it has been successfully deployed to a Parity full node
+ Provide developers with a lightweight, simple API for directly testing smart contracts
+ Mock every part external to smart contract execution (blockhash, blocktimes, senders, etc)
  - Potentially even mock features provided by other external contracts
+ Modular design as a pluggable backend
  - At time of writing, Solaris depends heavily on the monolithic `parity-ethcore` crate
  - Solaris will eventually be deployed as a stand-alone crate, with minimal dependencies 
+ Singularity in purpose as a unit testing framework
+ Interoperability with other test frameworks (common input/output formats?)
  - JSON
  - YAML

## Out-of-scope

+ Solaris is _NOT_ a development or all-in-one testing framework
  - Full-service frameworks like Truffle take one from design-to-deployment, Solaris is not that
  - Testing frameworks like Hive incorporate numerous testing strategies (unit, integration, etc), Solaris is not that
  - Solaris aims to be a focused, single-purpose utility for unit testing smart contracts
+ Consensus rules
  - Consensus rules determine which blockchain history is valid, not proper contract execution
  - Solaris is only concerned with how a contract behaves once successfully deployed on a blockchain
+ Differences in EVM implementations
  - Subtle differences in various EVM implementations _*may*_ lead to different behavior (which would lead to consensus bugs)
  - Validating Parity's EVM interpreter implementation is a separate and important project
  - Other tools, like Hive and KEVM are better suited for benchmarking/testing EVM implementations 
