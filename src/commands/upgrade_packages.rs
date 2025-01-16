use anyhow::{bail, Result};

use super::super::subprocess::getstatus_shell;

pub fn upgrade_packages(name: String) -> Result<()> {
    let code = getstatus_shell(format!(
        "ssh root@{} 'opkg update && packages=$(opkg list-upgradable | awk \"{{print \\$1}}\") && if [ \"$packages\" != \"\" ] ; then opkg upgrade $packages ; else echo \"No packages to upgrade\" ; fi'",
        name
    ))?;

    if code != 0 {
        bail!("Error upgrading packages");
    }

    Ok(())
}
