use std::process::Command;

fn main(){
    if let Err(code) = process(std::env::args().skip(1)) {
        std::process::exit(code);
    }
}

fn process<I>(args: I) -> Result<(), i32>
where
    I: Iterator<Item = String>,
{
    let mut rustc_path = std::env::current_exe()
        .expect("current executable path invalid")
        .with_file_name("rustc-tool");
    if cfg!(windows) {
        rustc_path.set_extension("exe");
    }

    let exit_status = Command::new("cargo".to_string())
        .args(args)
        .env("RUSTC_WRAPPER", rustc_path)
        .status()
        .expect("could not run cargo");

    if exit_status.success() {
        Ok(())
    } else {
        Err(exit_status.code().unwrap_or(-1))
    }
}
