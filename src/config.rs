//! Configuration of the installed systemd-boot environment.
//!
//! Create a Config by using either `Config::new()`, or `ConfigBuilder` to build a `Config` from
//! scratch.
//!
//! # Examples
//!
//! ```
//! use libsdbootconf::config::{Config, ConfigBuilder};
//!
//! let config = Config::new(Some("5.12.0-aosc-main"), Some(5u32));
//! let built = ConfigBuilder::new()
//!     .default("5.12.0-aosc-main")
//!     .timeout(5u32)
//!     .build();
//!
//! assert_eq!(config.to_string(), built.to_string());
//! ```

use std::{fs, ops::Not, path::Path, str::FromStr};

use crate::{generate_builder_method, Entry, LibSDBootConfError};

/// A systemd-boot loader configuration.
#[derive(Default, Debug, PartialEq)]
pub struct Config {
    /// Pattern to select the default entry in the list of entries.
    pub default: Option<String>,
    /// Timeout in seconds for how long to show the menu.
    pub timeout: Option<u32>,
}

impl FromStr for Config {
    type Err = LibSDBootConfError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut config = Self::default();
        let lines = s.lines();

        for line in lines {
            if line.starts_with('#') || line.is_empty() {
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
    /// Create a new `Config`.
    ///
    /// # Examples
    ///
    /// ```
    /// use libsdbootconf::config::Config;
    ///
    /// let config = Config::new(Some("5.12.0-aosc-main"), Some(5u32));
    ///
    /// assert_eq!(config.default, Some("5.12.0-aosc-main".to_owned()));
    /// assert_eq!(config.timeout, Some(5u32));
    /// ```
    pub fn new<S, U>(default: Option<S>, timeout: Option<U>) -> Config
    where
        S: Into<String>,
        U: Into<u32>,
    {
        Config {
            default: default.map(|s| s.into()),
            timeout: timeout.map(|u| u.into()),
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
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Config, LibSDBootConfError> {
        Config::from_str(&fs::read_to_string(path.as_ref())?)
    }

    /// Save the config to a file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libsdbootconf::config::Config;
    ///
    /// let config = Config::new(Some("5.12.0-aosc-main"), Some(5u32));
    /// config.write("/path/to/config").unwrap();
    /// ```
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<(), LibSDBootConfError> {
        fs::write(path.as_ref(), self.to_string())?;

        Ok(())
    }

    /// Set an Entry as the default boot entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use libsdbootconf::{Entry, Config};
    ///
    /// let mut config = Config::default();
    /// let entry = Entry::new("5.12.0-aosc-main", Vec::new());
    ///
    /// config.set_default(&entry);
    ///
    /// assert_eq!(config.default, Some("5.12.0-aosc-main.conf".to_owned()));
    /// ```
    pub fn set_default(&mut self, default: &Entry) {
        self.default = Some(
            default.id.clone()
                + default
                    .id
                    .ends_with(".conf")
                    .not()
                    .then_some(".conf")
                    .unwrap_or_default(),
        );
    }

    /// Try to load the default entry as an Entry object.
    ///
    /// Returns `None` if the config does not contain a `default` field.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libsdbootconf::{Config, Entry};
    ///
    /// let config = Config::new(Some("5.12.0-aosc-main.conf"), Some(5u32));
    ///
    /// assert_ne!(None, config.default_entry("/efi/loader/entries/").unwrap())
    /// ```
    pub fn default_entry<P: AsRef<Path>>(
        &self,
        directory: P,
    ) -> Result<Option<Entry>, LibSDBootConfError> {
        self.default
            .as_ref()
            .map(|default| Entry::load(directory.as_ref().join(default)))
            .transpose()
    }
}

/// Builder for `Config`.
#[derive(Default, Debug)]
pub struct ConfigBuilder {
    inner: Config,
}

impl ConfigBuilder {
    /// Create an empty `ConfigBuilder`.
    pub fn new() -> Self {
        Self {
            inner: Config::default(),
        }
    }

    generate_builder_method!(
        /// Set the default entry with a `String`.
        option INNER(inner) default(S: String)
    );
    generate_builder_method!(
        /// Set the timeout.
        option INNER(inner) timeout(U: u32)
    );

    /// Set the default entry with an `Entry`.
    pub fn default_entry(mut self, entry: &Entry) -> Self {
        self.inner.set_default(entry);

        self
    }

    /// Build the `Config`.
    pub fn build(self) -> Config {
        self.inner
    }
}
