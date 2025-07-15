use anyhow::Result;

use super::super::config::Config;
use shleazy::run_shell_or_err;

pub fn set_wifi(config: &Config, enabled: bool) -> Result<()> {
    println!("Set wifi: {}: {}", &config.name, enabled);

    let disabled_str = if enabled { "0" } else { "1" };

    run_shell_or_err(format!("ssh root@{} \"for config in \\$(uci show wireless | grep -E 'wifi-(device|iface)' | cut -d '=' -f1) ; do uci set \\${{config}}.disabled={} ; done ; uci commit wireless; wifi reload\"", &config.name, disabled_str))
}
