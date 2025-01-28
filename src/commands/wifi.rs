use anyhow::Result;

use shleazy::run_shell_or_err;

pub fn set_wifi(name: String, enabled: bool) -> Result<()> {
    println!("Set wifi: {}: {}", &name, enabled);

    let disabled_str = if enabled { "0" } else { "1" };

    run_shell_or_err(format!("ssh root@{} \"for config in \\$(uci show wireless | grep -E 'wifi-(device|iface)' | cut -d '=' -f1) ; do uci set \\${{config}}.disabled={} ; done ; uci commit wireless; wifi reload\"", name, disabled_str))
}
