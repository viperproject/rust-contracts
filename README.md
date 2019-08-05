Rust Contracts
==============

[![Build Status](https://travis-ci.org/viperproject/rust-contracts.svg?branch=master)](https://travis-ci.org/viperproject/rust-contracts)

This crate offers a way declare the *contract* (e.g. functional specification) of Rust functions.


Test
----

```bash
# By default, contracts are just ignored
cargo test --all-features
cargo run --example true | (! grep -q 'requires: true')

# Clean up
cargo clean
cargo clean --manifest-path example_plugin/Cargo.toml
cargo clean --manifest-path example_tool/Cargo.toml

# Build a tool that checks the contracts (in this example, it will only print the contract)
cargo build --manifest-path example_plugin/Cargo.toml
cargo build --manifest-path example_tool/Cargo.toml

# Run the example with the modified contracts behaviour
export RUST_CONTRACTS_LIB=example_plugin/target/debug/libexample_plugin.so
example_tool/target/debug/cargo-tool test --examples
example_tool/target/debug/cargo-tool run --example true | grep -q 'requires: true'
```


License
-------

Licensed under either of

- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT) at your option.
