use anyhow::{bail, Result};

use super::super::subprocess::getstatus_shell;

pub fn create_backup(name: String) -> Result<()> {
    let code = getstatus_shell(format!(
        "ssh root@{} sysupgrade -k --create-backup - | tar -xvzC {}",
        name, name
    ))?;

    if code != 0 {
        bail!("Error creating backup");
    }

    Ok(())
}
