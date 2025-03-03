use anyhow::Result;
use std::path::PathBuf;

use super::super::config::get_config;
use super::super::openwrt_vars::{image_file_path, sdk_dir};
use sha256::try_digest;
use shleazy::{getstatus_shell, run_shell_or_err};

pub fn install_image(name: String) -> Result<()> {
    let config = get_config(&name)?;

    println!("Installing image for: {}", &name);

    let mut image_path: PathBuf = std::env::current_dir()?;
    image_path.push("tmp");
    image_path.push(sdk_dir(&config)?);
    image_path.push(image_file_path(&config)?);

    if image_path.exists() {
        let image_path_str = image_path.to_string_lossy();

        // Calculate hash
        let local_hash = try_digest(&image_path)?;

        println!("Image sha256sum: '{}'", &local_hash);

        // Copy file
        println!("Copying image: '{}'", &image_path_str);
        run_shell_or_err(format!(
            "scp -O \"{}\" root@{}:/tmp/image.bin",
            &image_path_str, &name
        ))?;

        // Verify copied file and run upgrade
        println!("Verifying image and starting install (NOTE: ignore 'connection failed' error)");
        // NOTE: this sysupgrade command typically fails...
        let _code = getstatus_shell(format!("ssh root@{} \"[[ \\$(sha256sum /tmp/image.bin | awk '{{print \\$1}}') = \"{}\" ]] && sysupgrade -v -n /tmp/image.bin\"", &name, &local_hash))?;
    } else {
        eprintln!("Image file does not exist: {:?}", &image_path);
    }

    Ok(())
}
