use anyhow::Result;
use serde::Deserialize;
use std::path::Path;

const CONFIG_DIR: &str = "conf";
const BASE_CONFIG_FILE_NAME: &str = "base.yml";

#[derive(Deserialize, Debug)]
pub struct BaseConfig {
    pub release: String,
    pub sysupgrade_extension: String,
    pub packages: Vec<String>,
    pub extra_packages: Vec<String>,
    pub package_removals: Vec<String>,
    pub disabled_services: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct BaseIncludes {
    pub packages: bool,
    pub extra_packages: bool,
    pub package_removals: bool,
    pub disabled_services: bool,
}

#[derive(Deserialize, Debug)]
pub struct ImageConfig {
    pub target: String,
    pub sub_target: String,
    pub profile: String,
    pub extra_image_name: String,
    pub packages: Option<Vec<String>>,
    pub package_removals: Option<Vec<String>>,
    // Optional values
    pub disabled_services: Option<Vec<String>>,
    pub release: Option<String>,
    pub sysupgrade_extension: Option<String>,
    // Optional include values
    pub includes: Option<BaseIncludes>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub defaults: BaseConfig,
    pub image: ImageConfig,
}

pub fn get_config(name: String) -> Result<Config> {
    let yaml_base_config =
        std::fs::read_to_string(Path::new(CONFIG_DIR).join(BASE_CONFIG_FILE_NAME))?;
    let defaults = serde_yaml::from_str(&yaml_base_config)?;

    let yaml_image_config = std::fs::read_to_string(Path::new(CONFIG_DIR).join(name + ".yml"))?;
    let image = serde_yaml::from_str(&yaml_image_config)?;

    Ok(Config { defaults, image })
}
