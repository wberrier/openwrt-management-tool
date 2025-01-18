use anyhow::Result;

use shleazy::run_shell_or_err;

pub fn restore_backup(name: String) -> Result<()> {
    // fix dropbear directory permissions (since dir permissions don't persist in git)
    run_shell_or_err(&format!("chmod 700 {}/etc/dropbear", &name))?;

    run_shell_or_err(&format!(
        "tar -cvzC {} --group=0 --owner=0 . | ssh root@{} sysupgrade --restore-backup -",
        &name, &name
    ))?;

    run_shell_or_err(&format!("ssh root@{} reboot", &name))
}
