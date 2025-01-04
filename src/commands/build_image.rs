
use super::super::config::get_config;

use anyhow::Result;

pub fn build_image(name: String) -> Result<()> {

    let config = get_config(name)?;

    println!("Configuration: {:?}", config);

    Ok(())
}
