use std::{collections::HashMap, net::IpAddr};

use mac_address::MacAddress;
use serde_derive::Deserialize;
use thiserror::Error;


#[derive(Debug, Deserialize, Clone)]
pub struct SleepCommand {
    pub cmd: String,
    pub args: Vec<String>
}

#[derive(Debug, Deserialize, Clone)]
pub struct WOLConfig {
    pub mac_address: MacAddress,
    pub port: Option<u16>,
    pub broadcast_address: Option<IpAddr>,
    pub sleep_command: Option<SleepCommand>
}

#[derive(Error, Debug)]
#[error(transparent)]
pub enum RunError {
    #[error("General IO error")]
    IOErr(#[from] std::io::Error),

    #[error("Could not parse config")]
    TomlParseErr(#[from] toml::de::Error),

    #[error("Could not expand config path")]
    CouldNotExpand(#[from] shellexpand::LookupError<std::env::VarError>),

    #[error("No sleep command found in config")]
    NoSleepCommand,

    #[error("Error during sleep command")]
    SleepCommandFailed,

    #[error("Could not find [{section_name:?}]]")]
    UnknownSection { section_name: String },
}

impl RunError {
    fn unknown_section(name: &str) -> RunError {
        RunError::UnknownSection {
            section_name: String::from(name),
        }
    }
}


/// Load a toml file from `path` and extract the section `section_name` as a [WOLConfig] instance
pub fn read_config(path: &str, section_name: &str) -> Result<WOLConfig, RunError> {
    let real_path = shellexpand::full(path)?;
    let content = std::fs::read_to_string(&*real_path)?;
    let sections: HashMap<String, WOLConfig> = toml::from_str(&content)?;
    sections
        .get(section_name)
        .ok_or_else(|| RunError::unknown_section(section_name))
        .map(|c| c.clone())
}

