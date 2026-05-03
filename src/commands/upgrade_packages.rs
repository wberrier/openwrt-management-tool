use anyhow::Result;

use super::super::config::Config;
use shleazy::run_shell_or_err;

pub fn upgrade_packages(config: &Config) -> Result<()> {
    run_shell_or_err(format!(
        "ssh root@{} 'apk update && apk upgrade'",
        &config.name
    ))
}
