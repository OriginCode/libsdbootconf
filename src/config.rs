//! Configuration of the installed systemd-boot environment.

use std::{fs, path::Path, str::FromStr};

use crate::{generate_builder_method, LibSDBootConfError};

/// An abstraction over the configuration file of systemd-boot.
#[derive(Default, Debug)]
pub struct Config {
    /// Pattern to select the default entry in the list of entries.
    pub default: Option<String>,
    /// Timeout in seconds for how long to show the menu.
    pub timeout: Option<i32>,
}

impl FromStr for Config {
    type Err = LibSDBootConfError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut config = Self::default();
        let lines = s.lines();

        for line in lines {
            if line.starts_with('#') {
                continue;
            }

            let mut parts = line.splitn(2, ' ');
            let key = parts.next().ok_or(LibSDBootConfError::ConfigParseError)?;
            let value = parts.next().ok_or(LibSDBootConfError::ConfigParseError)?;

            match key {
                "default" => config.default = Some(value.to_string()),
                "timeout" => config.timeout = Some(value.parse().unwrap_or_default()),
                _ => continue,
            }
        }

        Ok(config)
    }
}

impl ToString for Config {
    fn to_string(&self) -> String {
        let mut buffer = String::new();

        if let Some(default) = &self.default {
            buffer.push_str(&format!("default {}\n", default));
        }

        if let Some(timeout) = &self.timeout {
            buffer.push_str(&format!("timeout {}\n", timeout));
        }

        buffer
    }
}

impl Config {
    /// Create a new Config.
    ///
    /// # Examples
    ///
    /// ```
    /// use libsdbootconf::config::Config;
    ///
    /// let config = Config::new(Some("5.12.0-aosc-main"), Some(5));
    ///
    /// assert_eq!(config.default, Some("5.12.0-aosc-main".to_owned()));
    /// assert_eq!(config.timeout, Some(5));
    /// ```
    pub fn new(default: Option<impl Into<String>>, timeout: Option<i32>) -> Config {
        Config {
            default: default.map(|x| x.into()),
            timeout,
        }
    }

    /// Load an existing config file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libsdbootconf::config::Config;
    ///
    /// let config = Config::load("/path/to/config").unwrap();
    /// ```
    pub fn load(path: impl AsRef<Path>) -> Result<Config, LibSDBootConfError> {
        Config::from_str(&fs::read_to_string(path.as_ref())?)
    }

    /// Save the config to a file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libsdbootconf::config::Config;
    ///
    /// let config = Config::new(Some("5.12.0-aosc-main"), Some(5));
    /// config.write("/path/to/config").unwrap();
    /// ```
    pub fn write(&self, path: impl AsRef<Path>) -> Result<(), LibSDBootConfError> {
        fs::write(path.as_ref(), self.to_string())?;

        Ok(())
    }
}

/// Builder for Config.
#[derive(Default, Debug)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    /// Create an empty ConfigBuilder.
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    generate_builder_method!(option default, impl Into<String>);
    generate_builder_method!(option timeout, i32);

    /// Build the Config.
    pub fn build(self) -> Config {
        self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let entry = ConfigBuilder::new().default("5.12.0-aosc-main").build();

        println!("{:?}", &entry);
    }
}
