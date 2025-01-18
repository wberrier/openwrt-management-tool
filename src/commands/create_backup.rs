use anyhow::Result;

use shleazy::run_shell_or_err;

pub fn create_backup(name: String) -> Result<()> {
    run_shell_or_err(&format!(
        "ssh root@{} sysupgrade -k --create-backup - | tar -xvzC {}",
        name, name
    ))
}
