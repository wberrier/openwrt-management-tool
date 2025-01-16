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
    // Optional values
    pub release: Option<String>,
    pub sysupgrade_extension: Option<String>,
    pub packages: Option<Vec<String>>,
    pub package_removals: Option<Vec<String>>,
    pub disabled_services: Option<Vec<String>>,
    // Optional include values
    pub includes: Option<BaseIncludes>,
}

#[derive(Debug)]
/// Final merged config
/// Not sure how to construct this in a simpler way...
pub struct Config {
    pub release: String,
    pub sysupgrade_extension: String,
    pub packages: Vec<String>,
    pub package_removals: Vec<String>,
    pub disabled_services: Vec<String>,

    pub target: String,
    pub sub_target: String,
    pub profile: String,
    pub extra_image_name: String,
}

impl Config {
    fn new() -> Self {
        // TODO: easier way to get some default values?
        Self {
            release: "".to_string(),
            sysupgrade_extension: "".to_string(),
            packages: vec![],
            package_removals: vec![],
            disabled_services: vec![],

            target: "".to_string(),
            sub_target: "".to_string(),
            profile: "".to_string(),
            extra_image_name: "".to_string(),
        }
    }
}

pub fn get_config(name: String) -> Result<Config> {
    let yaml_base_config =
        std::fs::read_to_string(Path::new(CONFIG_DIR).join(BASE_CONFIG_FILE_NAME))?;
    let base_config: BaseConfig = serde_yaml::from_str(&yaml_base_config)?;

    let yaml_image_config = std::fs::read_to_string(Path::new(CONFIG_DIR).join(name + ".yml"))?;
    let image_config: ImageConfig = serde_yaml::from_str(&yaml_image_config)?;

    let mut config = Config::new();

    // TODO: better way to minimize duplication? proc macro?

    // In general, allow image config to override base config, but
    // also allow to include stuff from both base and image config

    config.release = match image_config.release {
        Some(release) => release,
        None => base_config.release,
    };

    config.sysupgrade_extension = match image_config.sysupgrade_extension {
        Some(sysupgrade_extension) => sysupgrade_extension,
        None => base_config.sysupgrade_extension,
    };

    if let Some(packages) = image_config.packages {
        config.packages.extend(packages);
    }

    if let Some(package_removals) = image_config.package_removals {
        config.package_removals.extend(package_removals);
    }

    if let Some(disabled_services) = image_config.disabled_services {
        config.disabled_services.extend(disabled_services);
    }

    if let Some(includes) = image_config.includes {
        if includes.packages {
            config.packages.extend(base_config.packages);
        }
        if includes.extra_packages {
            config.packages.extend(base_config.extra_packages);
        }
        if includes.package_removals {
            config.package_removals.extend(base_config.package_removals);
        }
        if includes.disabled_services {
            config.disabled_services.extend(base_config.disabled_services);
        }
    }

    config.target = image_config.target;
    config.sub_target = image_config.sub_target;
    config.profile = image_config.profile;
    config.extra_image_name = image_config.extra_image_name;

    Ok(config)
}
