use anyhow::Result;
use serde::Deserialize;

const BASE_CONFIG_FILE_NAME: &str = "conf/base.yml";

#[derive(Deserialize)]
pub struct BaseOptions {
    pub release: String,
    pub sysupgrade_extension: String,
    pub packages: Vec<String>,
    pub extra_packages: Vec<String>,
    pub package_removals: Vec<String>,
    pub disabled_services: Vec<String>,
}

#[derive(Deserialize)]
pub struct ImageOptions {
    pub target: String,
    pub sub_target: String,
    pub profile: String,
    pub extra_image_name: String,
    pub packages: Vec<String>,
    pub package_removals: Vec<String>,
    // Optional values
    pub disabled_services: Vec<String>,
    pub release: Option<String>,
    pub sysupgrade_extension: Option<String>,
    // Optional include values
    pub include_packages: bool,
    pub include_extra_packages: bool,
    pub include_package_removals: bool,
    pub include_disabled_services: bool,
}

#[derive(Deserialize)]
pub struct Options {
    pub defaults: BaseOptions,
}

pub fn get_options() -> Result<Options> {
    let yaml_config = std::fs::read_to_string(BASE_CONFIG_FILE_NAME)?;

    let options = serde_yaml::from_str(&yaml_config)?;

    Ok(options)
}
