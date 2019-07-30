Rust Contracts
==============

This crate offers a way declare the *contract* (e.g. functional specification) of Rust functions.

:warning: WORK IN PROGRESS :warning:

Test
----

To test this crate without plugins:
```bash
cargo build --all --examples
cargo test --all --examples
```

To test this crate using the plugin `example_plugin`:
```bash
cargo build --all --examples
cargo build --manifest-path example_plugin/Cargo.toml
LD_LIBRARY_PATH=example_plugin/target/debug/ RUST_CONTRACTS_PLUGIN=example_plugin/target/debug/ cargo test --all --examples -- --nocapture
```


License
-------

Licensed under either of

- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT) at your option.
