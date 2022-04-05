//! This library provides a general purpose configuration interface for systemd-boot.
//! It parses systemd-boot configuration and systemd-boot entry configuration.
//!
//! Visit [systemd-boot](https://www.freedesktop.org/wiki/Software/systemd/systemd-boot/) for
//! more information.
//!
//! # Examples
//!
//! ```no_run
//! use libsdbootconf::{config::ConfigBuilder, entry::EntryBuilder, SystemdBootConfBuilder};
//!
//! let systemd_boot_conf = SystemdBootConfBuilder::new("/efi/loader")
//!     .config(ConfigBuilder::new()
//!         .default("5.12.0-aosc-main")
//!         .timeout(5)
//!         .build())
//!     .entries(vec![EntryBuilder::new("5.12.0-aosc-main")
//!         .title("5.12.0-aosc-main")
//!         .build()])
//!     .build();
//!
//! systemd_boot_conf.write_all().unwrap();
//! ```

use std::{
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;

pub mod config;
pub mod entry;
mod macros;

pub use config::Config;
pub use entry::Entry;

#[derive(Error, Debug)]
pub enum LibSDBootConfError {
    #[error("invalid configuration")]
    ConfigParseError,
    #[error("invalid entry")]
    EntryParseError,
    #[error("invalid entry filename {0}")]
    InvalidEntryFilename(PathBuf),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

/// An abstraction over the basic structure of systemd-boot configurations.
#[derive(Default, Debug)]
pub struct SystemdBootConf {
    pub working_dir: PathBuf,
    pub config: Config,
    pub entries: Vec<Entry>,
}

impl SystemdBootConf {
    /// Create a new SystemdBootConf with a working directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use libsdbootconf::SystemdBootConf;
    ///
    /// let systemd_boot_conf = SystemdBootConf::new("/efi/loader");
    ///
    /// assert_eq!(systemd_boot_conf.working_dir, std::path::PathBuf::from("/efi/loader"));
    /// ```
    pub fn new(working_dir: impl Into<PathBuf>) -> Self {
        Self {
            working_dir: working_dir.into(),
            ..Default::default()
        }
    }

    /// Read from an existing systemd-boot installation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libsdbootconf::SystemdBootConf;
    ///
    /// let systemd_boot_conf = SystemdBootConf::load("/efi/loader").unwrap();
    /// ```
    pub fn load(working_dir: impl AsRef<Path>) -> Result<Self, LibSDBootConfError> {
        let mut systemd_boot_conf = Self::new(working_dir.as_ref());

        systemd_boot_conf.load_current()?;

        Ok(systemd_boot_conf)
    }

    /// Read from the current systemd-boot working directory.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libsdbootconf::SystemdBootConf;
    ///
    /// let mut systemd_boot_conf = SystemdBootConf::new("/efi/loader");
    /// systemd_boot_conf.load_current().unwrap();
    /// ```
    pub fn load_current(&mut self) -> Result<(), LibSDBootConfError> {
        let config = Config::load(self.working_dir.join("loader.conf"))?;
        let mut entries = Vec::new();

        for file in fs::read_dir(self.working_dir.join("entries"))? {
            let path = file?.path();
            if path.is_file() {
                let entry = Entry::load(&path)?;
                entries.push(entry);
            }
        }

        self.config = config;
        self.entries = entries;

        Ok(())
    }

    /// Write all configurations and entries to system.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libsdbootconf::SystemdBootConf;
    ///
    /// let systemd_boot_conf = SystemdBootConf::new("/efi/loader");
    ///
    /// systemd_boot_conf.write_all().unwrap();
    /// ```
    pub fn write_all(&self) -> Result<(), LibSDBootConfError> {
        self.config.write(self.working_dir.join("loader.conf"))?;

        for entry in self.entries.iter() {
            entry.write(
                self.working_dir
                    .join("entries")
                    .join(format!("{}.conf", entry.id)),
            )?;
        }

        Ok(())
    }
}

/// Builder for SystemdBootConf.
#[derive(Default, Debug)]
pub struct SystemdBootConfBuilder {
    systemd_boot_conf: SystemdBootConf,
}

impl SystemdBootConfBuilder {
    /// Create an empty SystemdBootConfBuilder with a working directory.
    pub fn new(working_dir: impl Into<PathBuf>) -> Self {
        Self {
            systemd_boot_conf: SystemdBootConf::new(working_dir),
        }
    }

    generate_builder_method!(plain systemd_boot_conf, config, Config);
    generate_builder_method!(plain systemd_boot_conf, entries, Vec<Entry>);

    /// Build the SystemdBootConf.
    pub fn build(self) -> SystemdBootConf {
        self.systemd_boot_conf
    }
}
