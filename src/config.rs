use std::{fs, path::Path, str::FromStr};

use crate::LibSDBootError;

/// An abstraction over the configuration file of systemd-boot.
#[derive(Default, Debug)]
pub struct Config {
    pub default: String,
    pub timeout: i32,
}

impl FromStr for Config {
    type Err = LibSDBootError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut config = Self::default();
        let lines = s.lines();

        for line in lines {
            if line.starts_with('#') {
                continue;
            }

            let mut parts = line.splitn(2, ' ');
            let key = parts.next().ok_or(LibSDBootError::ConfigParseError)?;
            let value = parts.next().ok_or(LibSDBootError::ConfigParseError)?;

            match key {
                "default" => config.default = value.to_string(),
                "timeout" => config.timeout = value.parse().unwrap_or_default(),
                _ => continue,
            }
        }

        Ok(config)
    }
}

impl ToString for Config {
    fn to_string(&self) -> String {
        format!("default {}\ntimeout {}", self.default, self.timeout)
    }
}

impl Config {
    /// Create a new Config.
    ///
    /// # Examples
    ///
    /// ```
    /// use libsdboot::config::Config;
    ///
    /// let config = Config::new("5.12.0-aosc-main", 5);
    ///
    /// assert_eq!(config.default, "5.12.0-aosc-main");
    /// assert_eq!(config.timeout, 5);
    /// ```
    pub fn new(default: impl Into<String>, timeout: i32) -> Config {
        Config {
            default: default.into(),
            timeout,
        }
    }

    /// Load an existing config file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libsdboot::config::Config;
    ///
    /// let config = Config::load("/path/to/config").unwrap();
    /// ```
    pub fn load(path: impl AsRef<Path>) -> Result<Config, LibSDBootError> {
        Config::from_str(&fs::read_to_string(path.as_ref())?)
    }

    /// Save the config to a file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libsdboot::config::Config;
    ///
    /// let config = Config::new("5.12.0-aosc-main", 5);
    /// config.write("/path/to/config").unwrap();
    /// ```
    pub fn write(&self, path: impl AsRef<Path>) -> Result<(), LibSDBootError> {
        fs::write(path.as_ref(), self.to_string())?;

        Ok(())
    }
}
