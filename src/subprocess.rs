use std::process::Command;

use anyhow::{anyhow, Result};

// Get exit code, print the output as it goes
pub fn getstatus(exec: &str, args: Vec<String>) -> Result<i32> {
    let mut process = Command::new(exec);
    for a in args {
        process.arg(a);
    }

    println!("Running command: {:?}", process);

    let exit_status = process.status()?;
    let code = exit_status
        .code()
        .ok_or(anyhow!("Unable to get exit code"))?;
    Ok(code)
}

// Wrap in a shell invocation
pub fn getstatus_shell(command: String) -> Result<i32> {
    getstatus("/bin/sh".into(), vec!["-c".into(), command])
}
