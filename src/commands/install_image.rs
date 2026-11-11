use anyhow::Result;
use std::path::PathBuf;

use super::super::config::Config;
use super::super::openwrt_vars::{image_file_path, sdk_dir};
use sha256::try_digest;
use shleazy::{getstatus_shell, run_shell_or_err};

pub fn install_image(
    config: &Config,
    filename_arg: &Option<String>,
    hostname_arg: &Option<String>,
) -> Result<()> {
    println!("Installing image for: {}", &config.name);

    let mut image_path: PathBuf = Default::default();

    // Let the passed in value override the config
    if filename_arg.is_some() {
        image_path.push(filename_arg.as_ref().unwrap());
    } else {
        image_path.push(std::env::current_dir()?);
        image_path.push("tmp");
        image_path.push(sdk_dir(config)?);
        image_path.push(image_file_path(config)?);
    }

    let mut hostname = config.name.clone();
    // Let the passed in value override the hostname
    if hostname_arg.is_some() {
        hostname = hostname_arg.as_ref().unwrap().to_string();
    }

    if image_path.exists() {
        let image_path_str = image_path.to_string_lossy();

        // Calculate hash
        let local_hash = try_digest(&image_path)?;

        println!("Image sha256sum: '{}'", &local_hash);

        // Copy file
        println!("Copying image: '{}'", &image_path_str);
        run_shell_or_err(format!(
            "scp -O \"{}\" root@{}:/tmp/image.bin",
            &image_path_str, &hostname
        ))?;

        // Verify copied file and run upgrade
        println!("Verifying image and starting install (NOTE: ignore 'connection failed' error)");
        // NOTE: this sysupgrade command typically fails...
        let _code = getstatus_shell(format!("ssh root@{} \"[[ \\$(sha256sum /tmp/image.bin | awk '{{print \\$1}}') = \"{}\" ]] && sysupgrade -v -n /tmp/image.bin\"", &hostname, &local_hash))?;
    } else {
        eprintln!("Image file does not exist: {:?}", &image_path);
    }

    Ok(())
}
