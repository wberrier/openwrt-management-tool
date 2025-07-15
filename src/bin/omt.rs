use anyhow::{bail, Result};
use clap::{Parser, Subcommand, ValueEnum};

use openwrt_management_tool as omt;

use omt::commands::build_image::build_image;
use omt::commands::create_backup::create_backup;
use omt::commands::install_image::install_image;
use omt::commands::restore_backup::restore_backup;
use omt::commands::upgrade_packages::upgrade_packages;
use omt::commands::wifi::set_wifi;
use omt::config::get_config;

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
        /// install host dependencies
        #[clap(short = 'i')]
        install_build_deps: bool,
        /// skip rootfs file includes
        #[clap(short = 's')]
        skip_files: bool,
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
        let config = get_config(&name)?;

        match match args.command {
            OMTCommand::BuildImage {
                install_build_deps,
                skip_files,
            } => {
                let result =
                    build_image(&config, first_build_image && install_build_deps, skip_files);
                first_build_image = false;
                result
            }
            OMTCommand::InstallImage {} => install_image(&config),
            OMTCommand::CreateBackup {} => create_backup(&config),
            OMTCommand::RestoreBackup {} => restore_backup(&config),
            OMTCommand::UpgradePackages {} => upgrade_packages(&config),
            OMTCommand::SetWifi { value } => set_wifi(&config, value == OnOff::On),
        } {
            Ok(()) => {}
            Err(error) => {
                bail!("Error running command: {:?}, {:?}", args.command, error);
            }
        }
    }

    Ok(())
}
