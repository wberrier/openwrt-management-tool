use anyhow::{bail, Result};
use clap::{Parser, Subcommand, ValueEnum};

use openwrt_management_tool::commands::build_image::build_image;
use openwrt_management_tool::commands::create_backup::create_backup;
use openwrt_management_tool::commands::install_image::install_image;
use openwrt_management_tool::commands::restore_backup::restore_backup;
use openwrt_management_tool::commands::upgrade_packages::upgrade_packages;
use openwrt_management_tool::commands::wifi::set_wifi;

#[derive(Debug, Parser)]
#[clap(about, author)]
struct Args {
    #[clap(short, long, required = true, value_delimiter = ',')]
    names: Vec<String>,
    #[command(subcommand)]
    command: OMTCommand,
}

#[derive(Debug, Subcommand, Clone)]
enum OMTCommand {
    /// build firmware image
    BuildImage {
        #[clap(short = 'i')]
        install_build_deps: bool,
    },
    /// install firmware image
    InstallImage {},
    /// create backup configuration
    CreateBackup {},
    /// restore backup configuration
    RestoreBackup {},
    /// upgrade packages
    UpgradePackages {},
    /// set wifi
    SetWifi {
        #[arg(require_equals = true, num_args = 0..=1, value_enum, value_name = "on|off")]
        value: OnOff,
    },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum OnOff {
    On,
    Off,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Flag to ensure deps are only installed once
    let mut first_build_image = true;

    for name in args.names {
        match match args.command {
            OMTCommand::BuildImage { install_build_deps } => {
                let result = build_image(name, first_build_image && install_build_deps);
                first_build_image = false;
                result
            }
            OMTCommand::InstallImage {} => install_image(name),
            OMTCommand::CreateBackup {} => create_backup(name),
            OMTCommand::RestoreBackup {} => restore_backup(name),
            OMTCommand::UpgradePackages {} => upgrade_packages(name),
            OMTCommand::SetWifi { value } => set_wifi(name, value == OnOff::On),
        } {
            Ok(()) => {}
            Err(error) => {
                bail!("Error running command: {:?}, {:?}", args.command, error);
            }
        }
    }

    Ok(())
}
