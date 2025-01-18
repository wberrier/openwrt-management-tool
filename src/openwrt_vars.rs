use std::path::Path;

use version_compare::Version;

use anyhow::{anyhow, bail, Result};

use super::config::Config;

pub fn sdk_extension(config: &Config) -> Result<String> {
    let release_str = config.release.clone();
    let release = Version::from(release_str.as_str()).ok_or(anyhow!("invalid version"))?;

    // Versions older than this used .tar.xz
    let zst_version = Version::from("24").ok_or(anyhow!("invalid version"))?;

    let mut extension = ".tar.zst".to_string();

    // Snapshot is .zst ...
    if release_str != "snapshot" && release < zst_version {
        extension = ".tar.xz".to_string();
    }

    Ok(extension)
}

pub fn image_builder_url(config: &Config) -> Result<String> {
    if config.target.is_empty() || config.sub_target.is_empty() {
        bail!("Missing target or sub_target");
    }

    let sdk_extension = sdk_extension(&config)?;

    if config.release == "snapshot" {
        return Ok(format!("https://downloads.openwrt.org/snapshots/targets/{}/{}/openwrt-imagebuilder-{}-{}.Linux-x86_64{}", &config.target, &config.sub_target, &config.target, &config.sub_target, sdk_extension));
    }

    Ok(format!("https://downloads.openwrt.org/releases/{}/targets/{}/{}/openwrt-imagebuilder-{}-{}-{}.Linux-x86_64{}", &config.release, &config.target, &config.sub_target, &config.release, &config.target, &config.sub_target, sdk_extension))
}

// TODO: return Path instead of String?
pub fn sdk_dir(config: &Config) -> Result<String> {
    let archive_path = archive_path(&config)?;

    let sdk_extension = sdk_extension(&config)?;

    let sdk_dir = archive_path
        .strip_suffix(sdk_extension.as_str())
        .ok_or(anyhow!("Unable to determine sdk dir"))?;

    Ok(sdk_dir.to_string())
}

pub fn archive_path(config: &Config) -> Result<String> {
    let url = &image_builder_url(&config)?;
    let path = Path::new(url);
    Ok(path
        .file_name()
        .ok_or(anyhow!("Unknown archive path"))?
        .to_string_lossy()
        .to_string())
}

pub fn image_file_path(config: &Config) -> Result<String> {
    let mut image_release = format!("-{}", config.release);
    if config.release == "snapshot" {
        image_release = "".to_string();
    }

    Ok(format!(
        "bin/targets/{}/{}/openwrt{}-{}-{}-{}-{}-squashfs-sysupgrade{}",
        &config.target,
        &config.sub_target,
        image_release,
        &config.extra_image_name,
        &config.target,
        &config.sub_target,
        &config.profile,
        &config.sysupgrade_extension,
    ))
}
