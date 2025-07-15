use anyhow::Result;

use super::super::config::Config;
use shleazy::run_shell_or_err;

pub fn upgrade_packages(config: &Config) -> Result<()> {
    run_shell_or_err(&format!(
        "ssh root@{} 'opkg update && packages=$(opkg list-upgradable | awk \"{{print \\$1}}\") && if [ \"$packages\" != \"\" ] ; then opkg upgrade $packages ; else echo \"No packages to upgrade\" ; fi'",
        &config.name
    ))
}
