use anyhow::Result;

use shleazy::run_shell_or_err;

pub fn upgrade_packages(name: String) -> Result<()> {
    run_shell_or_err(&format!(
        "ssh root@{} 'opkg update && packages=$(opkg list-upgradable | awk \"{{print \\$1}}\") && if [ \"$packages\" != \"\" ] ; then opkg upgrade $packages ; else echo \"No packages to upgrade\" ; fi'",
        name
    ))
}
