use anyhow::{bail, Result};

use super::super::os_common::{install_packages, package_system, PackageSystem};

pub fn install_build_requirements() -> Result<()> {
    match package_system() {
        Ok(PackageSystem::DNF) => {
            install_packages("git gawk gettext ncurses-devel zlib-devel openssl-devel libxslt wget which @c-development @development-tools @development-libs zlib-static python3 python3-setuptools perl-FindBin")

        },
        Ok(PackageSystem::APT) => {
            install_packages("build-essential libncurses-dev zlib1g-dev gawk git gettext libssl-dev xsltproc rsync wget unzip python3")
        }
        Err(error) => bail!(error)
    }
}
