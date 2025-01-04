use anyhow::Result;
use structopt::StructOpt;

use openwrt_management_tool::commands::build_image::build_image;

#[derive(Debug, StructOpt)]
#[structopt(about, author)]
enum OMTCommands {
    #[structopt(about = "build firmware image")]
    BuildImage { name: String },
    #[structopt(about = "install firmware image")]
    InstallImage {},
    #[structopt(about = "create backup configuration")]
    CreateBackup {},
    #[structopt(about = "restore backup configuration")]
    RestoreBackup {},
    #[structopt(about = "install packages")]
    InstallPackages {},
    #[structopt(about = "upgrade packages")]
    UpgradePackages {},
}

fn main() -> Result<()> {
    let subcommand_options = OMTCommands::from_args();

    match subcommand_options {
        OMTCommands::BuildImage { name } => build_image(name),
        _ => Ok(()),
    }
}
