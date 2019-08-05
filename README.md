Rust Contracts
==============

[![Build Status](https://travis-ci.org/viperproject/rust-contracts.svg?branch=master)](https://travis-ci.org/viperproject/rust-contracts)

This crate offers the syntax to specify the *contract* of Rust functions (e.g. functional specification).

By default all contracts are ignored.

Verification tools can plug-in their implementation of contracts, in order to check the contracts statically or at runtime, using the technique of their choice.


Example of contract
-------------------

Contracts can be used to specify the requirements and the guarantees of a function:

```rust
#[requires(x > 5)]
#[ensures(result >= 0)]
fn foo(x: i32) -> i32 {
    if x <= 5 {
        panic!();
    }
    (x - 1) / 2
}
```


Compile ignoring contracts
--------------------------

Be default, contracts are ignored.

```bash
cargo run --example true
```

Note that the example does not print messages to standard output.


Custom implementation of contracts
----------------------------------

The behaviour of contracts can be configured.
As an example, we provide in the `example_verification_tool` and `example_contracts_impl` folders an implementation that prints the contracts at runtime.

```bash
# Build the implementation of contracts and the compilation tool
cargo build --manifest-path example_contracts_impl/Cargo.toml
cargo build --manifest-path example_verification_tool/Cargo.toml

# Compile and run an example with the modified contracts behaviour
export RUST_CONTRACTS_LIB=example_contracts_impl/target/debug/libexample_contracts_impl.so
cargo clean
example_verification_tool/target/debug/cargo-tool run --example true
```

You can see from the output that the contract is printed while running the example.


License
-------

Licensed under either of

- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT) at your option.
