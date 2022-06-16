use serde::{Serialize, Deserialize};
use ron::{ from_str, ser::{to_string_pretty, PrettyConfig} };
use anyhow::Result;
use log::{error};

use std::path;
use std::fs;
use std::io::prelude::*;

#[cfg(target_family="unix")]
use std::os::unix::fs::OpenOptionsExt;

// Am going to try to keep this bot cross platform until the exact moment it becomes inconvenient to me
// also only the config dir path has been tested (on linux) because that's the directory I use
fn get_config_path() -> path::PathBuf {
    let file = path::Path::new("chloebot.ron");
    // check if there's a config in the same directory as the executable
    if file.exists() {
        return file.to_path_buf();
    } else if let Some(conf_dir) = dirs::config_dir() {
        return conf_dir.join("chloe").join("chloebot.ron");
    } else {
        return file.to_path_buf();
    }
}

/// Config stored in a file, containing the bot token secret, command prefix, and other configurable things.
#[derive(Serialize, Deserialize)]
pub struct Config {
    token: String,
    prefix: char,
    case_insensitive: bool,
}

impl Config {
    /// Returns a template config file. The template key needs to be replaced manually
    pub fn template() -> Self {
        Self {
            token: "TOKEN GOES HERE".into(),
            prefix: ';',
            case_insensitive: true,
        }
    }
    /// Fetches the config file, or creates a template and exits if a config doesn't already exist
    // TODO: fix ambiguous method name
    pub fn get() -> Result<Self> {
        let path = get_config_path();
        if path.is_file() {
            let mut buf = String::new();
            let mut f = fs::File::options().write(false).read(true).open(path)?;

            f.read_to_string(&mut buf)?;
            drop(f);
            return Ok(from_str(&buf)?);
        } else {
            error!("No config file found! Creating template config at {} and closing.", path.display());
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            let mut options = fs::File::options();
            options.create(true).write(true);

            // if on unix, disallow other users or groups from reading or writing to the config file
            // since it contains secrets and all that
            // (I think I got the octal right?? I don't use custom unix file permissions very often) 
            #[cfg(target_family="unix")]
            options.mode(0o600);

            let mut f = options.open(path)?;
            
            f.write_all( to_string_pretty(&Self::template(), PrettyConfig::default())?.as_bytes() )?;
            f.sync_data()?;
            std::process::exit(1);
        }
    }
    // getters

    /// Returns bot token
    pub fn get_token(&self) -> &str {
        self.token.as_str()
    }

    /// Returns prefix used for bot commands. 
    pub fn get_prefix(&self) -> char {
        self.prefix
    }

    /// Returns whether commands are case sensitive.
    pub fn is_case_insensitive(&self) -> bool {
        self.case_insensitive
    }
}