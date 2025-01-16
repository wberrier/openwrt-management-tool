use anyhow::Result;
use structopt::StructOpt;

use openwrt_management_tool::commands::build_image::build_image;
use openwrt_management_tool::commands::create_backup::create_backup;
use openwrt_management_tool::commands::install_build_requirements::install_build_requirements;
use openwrt_management_tool::commands::upgrade_packages::upgrade_packages;

#[derive(Debug, StructOpt)]
#[structopt(about, author)]
enum OMTCommands {
    #[structopt(about = "install build requirements")]
    InstallBuildRequirements {},
    #[structopt(about = "build firmware image")]
    BuildImage { name: String },
    #[structopt(about = "install firmware image")]
    InstallImage {},
    #[structopt(about = "create backup configuration")]
    CreateBackup { name: String },
    #[structopt(about = "restore backup configuration")]
    RestoreBackup {},
    #[structopt(about = "install packages")]
    InstallPackages {},
    #[structopt(about = "upgrade packages")]
    UpgradePackages { name: String },
}

fn main() -> Result<()> {
    let subcommand_options = OMTCommands::from_args();

    match subcommand_options {
        OMTCommands::InstallBuildRequirements {} => install_build_requirements(),
        OMTCommands::BuildImage { name } => build_image(name),
        OMTCommands::CreateBackup { name } => create_backup(name),
        OMTCommands::UpgradePackages { name } => upgrade_packages(name),
        _ => Ok(()),
    }
}
