use anyhow::{bail, Result};

use super::super::subprocess::getstatus_shell;

pub fn restore_backup(name: String) -> Result<()> {
    // fix dropbear directory permissions (since dir permissions don't persist in git)
    getstatus_shell(format!("chmod 700 {}/etc/dropbear", &name))?;

    let code = getstatus_shell(format!(
        "tar -cvzC {} --group=0 --owner=0 . | ssh root@{} sysupgrade --restore-backup -",
        &name, &name
    ))?;

    if code != 0 {
        bail!("Error restoring backup");
    }

    getstatus_shell(format!("ssh root@{} reboot", &name))?;

    Ok(())
}
