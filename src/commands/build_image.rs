use anyhow::{bail, Result};
use std::fs;
use std::path::{Path, PathBuf};

use super::super::commands::install_build_requirements::install_build_requirements;
use super::super::config::Config;
use super::super::openwrt_vars::{archive_path, image_builder_url, sdk_dir};
use shleazy::{getstatus, run_shell_or_err};

fn fetch_image_builder(config: &Config) -> Result<()> {
    // TODO: could use native rust, but it would need to decode, support partial downloads, etc...
    let image_builder_url = image_builder_url(&config)?;
    println!("Fetching image builder: {}", image_builder_url);

    run_shell_or_err(&format!(
        "wget --continue --timestamping {}",
        image_builder_url
    ))
}

fn extract_image_builder(config: &Config) -> Result<()> {
    let sdk_dir = sdk_dir(&config)?;
    let archive_path = archive_path(&config)?;

    if !Path::new(&sdk_dir).exists() {
        run_shell_or_err(&format!("tar axf {}", archive_path))?;
    } else {
        println!("sdk already extracted");
    }

    Ok(())
}

pub fn build_image(config: &Config, install_build_deps: bool, skip_files: bool) -> Result<()> {
    if install_build_deps {
        install_build_requirements()?;
    }

    println!("Building image for: {}", &config.name);
    println!("Configuration: {:#?}", config);

    let current_dir: PathBuf = std::env::current_dir()?;
    let mut temp_dir = current_dir.clone();
    temp_dir.push("tmp");
    let mut rootfs_dir_path = current_dir.clone();
    rootfs_dir_path.push(&config.name);
    let rootfs_dir = rootfs_dir_path.as_path().to_string_lossy();

    fs::create_dir_all(&temp_dir)?;

    std::env::set_current_dir(&temp_dir.as_path())?;

    fetch_image_builder(&config)?;
    extract_image_builder(&config)?;

    let sdk_dir = sdk_dir(&config)?;

    std::env::set_current_dir(Path::new(&sdk_dir))?;

    // Construct arguments
    let mut packages_str = config.packages.join(" ");
    for p in &config.package_removals {
        packages_str.push_str(format!(" -{}", p).as_str());
    }

    let disabled_services_str = config.disabled_services.join(" ");

    let mut make_args = vec![
        "image".to_string(),
        format!("PROFILE={}", &config.profile),
        format!("PACKAGES={}", packages_str),
        format!("EXTRA_IMAGE_NAME={}", &config.extra_image_name),
        format!("DISABLED={}", disabled_services_str),
    ];

    if !skip_files {
        make_args.push(format!("FILES={}", rootfs_dir));
    } else {
        println!("NOTE: skipping files");
    }

    let code = getstatus("make", make_args)?;

    // Go back to original directory
    std::env::set_current_dir(current_dir.as_path())?;

    if code != 0 {
        bail!("Error building image");
    }

    Ok(())
}
