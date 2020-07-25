use std::error;
use std::process;

#[cfg(debug_assertions)]
const CLI: &str = "./target/debug/silverfish";
#[cfg(not(debug_assertions))]
const CLI: &str = "./target/release/silverfish";

#[test]
fn cli_help_test() -> Result<(), Box<dyn error::Error>> {
    // sanity check that code compiles and runs
    let mut command = process::Command::new(CLI);
    command.args(&["--help"]);
    println!("{:?}", command);
    let status = command.output()?.status;
    assert!(status.success());
    Ok(())
}

#[test]
fn code_benches_test() -> Result<(), Box<dyn error::Error>> {
    // run oode_benches
    let mut command = process::Command::new("code_benches/run.py");
    if cfg!(not(debug_assertions)) {
        command.args(&["--release"]);
    }
    println!("{:?}", command);
    let status = command.status()?;
    assert!(status.success());
    Ok(())
}

