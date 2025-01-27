use anyhow::Result;
use clap::Parser;

use openwrt_management_tool::commands::build_image::build_image;
use openwrt_management_tool::commands::create_backup::create_backup;
use openwrt_management_tool::commands::install_build_requirements::install_build_requirements;
use openwrt_management_tool::commands::install_image::install_image;
use openwrt_management_tool::commands::restore_backup::restore_backup;
use openwrt_management_tool::commands::upgrade_packages::upgrade_packages;

#[derive(Debug, Parser)]
#[clap(about, author)]
enum OMTCommands {
    #[clap(about = "install build requirements")]
    InstallBuildRequirements {},
    #[clap(about = "build firmware image")]
    BuildImage { name: String },
    #[clap(about = "install firmware image")]
    InstallImage { name: String },
    #[clap(about = "create backup configuration")]
    CreateBackup { name: String },
    #[clap(about = "restore backup configuration")]
    RestoreBackup { name: String },
    #[clap(about = "upgrade packages")]
    UpgradePackages { name: String },
}

fn main() -> Result<()> {
    let subcommand_options = OMTCommands::parse();

    match subcommand_options {
        OMTCommands::InstallBuildRequirements {} => install_build_requirements(),
        OMTCommands::BuildImage { name } => build_image(name),
        OMTCommands::InstallImage { name } => install_image(name),
        OMTCommands::CreateBackup { name } => create_backup(name),
        OMTCommands::RestoreBackup { name } => restore_backup(name),
        OMTCommands::UpgradePackages { name } => upgrade_packages(name),
    }
}
