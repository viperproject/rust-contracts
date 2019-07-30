use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=RUST_CONTRACTS_PLUGIN");
    if let Some(plugin_path) = env::var("RUST_CONTRACTS_PLUGIN").ok() {
        println!("cargo:rerun-if-changed={}", plugin_path);
        println!("cargo:rustc-link-search={}", plugin_path);
        println!("cargo:rustc-link-lib=dylib=rust_contracts_plugin");
        println!("cargo:rustc-cfg=use_plugin");
    };
}
