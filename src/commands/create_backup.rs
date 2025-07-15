use anyhow::Result;

use super::super::config::Config;
use shleazy::run_shell_or_err;

pub fn create_backup(config: &Config) -> Result<()> {
    run_shell_or_err(&format!(
        "ssh root@{} sysupgrade -k --create-backup - | tar -xvzC {}",
        &config.name, &config.name
    ))
}
