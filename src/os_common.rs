use anyhow::{anyhow, bail, Result};
use os_info::Type::*;

pub enum PackageSystem {
    DNF,
    APT,
}

pub fn package_system() -> Result<PackageSystem> {
    match os_info::get().os_type() {
        Fedora | AlmaLinux | OracleLinux | Redhat | RedHatEnterprise | CentOS => {
            Ok(PackageSystem::DNF)
        }
        Debian | Ubuntu => Ok(PackageSystem::APT),
        _ => Err(anyhow!("Unsupported linux distribution")),
    }
}

pub fn install_packages(packages: &str) -> Result<()> {
    let install_command = match package_system() {
        Ok(PackageSystem::DNF) => "sudo dnf install -y --skip-broken --allowerasing",
        Ok(PackageSystem::APT) => {
            "sudo apt-get update ; sudo DEBIAN_FRONTEND=noninteractive apt-get install -yq"
        }
        Err(error) => bail!(error),
    };

    let (code, output, _error) =
        run_script::run_script!(format!("{} {}", install_command, packages))?;
    if code == 0 {
        return Ok(());
    } else {
        bail!("Error installing packages: {}", output);
    }
}
