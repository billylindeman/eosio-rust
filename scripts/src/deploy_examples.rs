use crate::shared::cleos;
use std::io;
use std::process::ExitStatus;

fn deploy_example_contract(account: &str, bin: &str) -> io::Result<ExitStatus> {
    cleos()
        .arg("set")
        .arg("abi")
        .arg(account)
        .arg(format!("mnt/dev/examples/{}/{}.abi.json", bin, bin))
        .status()?;
    cleos()
        .arg("set")
        .arg("code")
        .arg(account)
        .arg(format!("mnt/dev/release/{}_gc.wasm", bin))
        .status()
}

pub fn run_deploy_examples() -> io::Result<()> {
    for (package, bin, account) in &[
        ("addressbook", "addressbook", "addressbook"),
        ("hello", "hello", "hello"),
        ("hello_bare", "hello_bare", "hellobare"),
        ("tictactoe", "tictactoe", "tictactoe"),
    ] {
        crate::build_contract(package);
        deploy_example_contract(account, bin)?;
    }
    Ok(())
}
