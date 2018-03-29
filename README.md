# Solaris

[![Build Status][travis-image]][travis-url]

[travis-image]: https://travis-ci.org/paritytech/sol-rs.svg?branch=master
[travis-url]: https://travis-ci.org/paritytech/sol-rs

Solidity Smart Contracts test harness written in Rust using the native Parity Client EVM.

## Benefits

+ Statically typed tests in Rust: if the contract changes the test code does not compile
+ Using native Parity EVM: latest stuff from Parity - will run exactly the same as on chain
+ High performance: no servers, directly to EVM, etc...
