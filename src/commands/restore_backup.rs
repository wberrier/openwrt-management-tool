use anyhow::Result;

use super::super::config::Config;
use shleazy::run_shell_or_err;

pub fn restore_backup(config: &Config) -> Result<()> {
    // fix dropbear directory permissions (since dir permissions don't persist in git)
    run_shell_or_err(format!("chmod 700 {}/etc/dropbear", &config.name))?;

    run_shell_or_err(format!(
        "tar -cvzC {} --group=0 --owner=0 . | ssh root@{} sysupgrade --restore-backup -",
        &config.name, &config.name
    ))?;

    run_shell_or_err(format!("ssh root@{} reboot", &config.name))
}
