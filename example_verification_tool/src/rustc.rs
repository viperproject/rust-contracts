use std::process::Command;
use std::path::Path;
use std::env;

fn main(){
    if let Err(code) = process(std::env::args().skip(1).collect()) {
        std::process::exit(code);
    }
}

fn process(mut args: Vec<String>) -> Result<(), i32> {
    // Remove leading "rustc" when called with RUSTC_WRAPPER
    if args.len() > 0 && Path::new(&args[0]).file_stem() == Some("rustc".as_ref()) {
        args.remove(0);
    }

    let rust_contracts_lib = env::var("RUST_CONTRACTS_LIB")
        .expect("The RUST_CONTRACTS_LIB environment variable is missing");

    // Swap `rust_contracts` with the library specified in `RUST_CONTRACTS_LIB`
    let mut preceded_by_extern = false;
    for arg in args.iter_mut() {
        if preceded_by_extern && arg.starts_with("rust_contracts=") {
            *arg = format!("rust_contracts={}", rust_contracts_lib);
        } else if arg == "--extern" {
            preceded_by_extern = true;
            continue
        }
    }

    let exit_status = Command::new("rustc".to_string())
        .args(args)
        .status()
        .expect("could not run rustc");

    if exit_status.success() {
        Ok(())
    } else {
        Err(exit_status.code().unwrap_or(-1))
    }
}
