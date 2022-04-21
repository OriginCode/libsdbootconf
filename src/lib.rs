//! This library provides a configuration interface for systemd-boot. It parses systemd-boot loader
//! configuration and systemd-boot entry configuration.
//!
//! **NOTE**: Not all fields in <https://www.freedesktop.org/software/systemd/man/systemd-boot.html>
//! are implemented, this library currently only provides interface for the fields listed on
//! <https://www.freedesktop.org/wiki/Software/systemd/systemd-boot/>.
//!
//! # Create and write a new systemd-boot configuration
//!
//! You can use the `SystemdBootConfig` struct to create a new systemd-boot configuration, or use
//! `SystemdBootConfigBuilder` to build a `SystemdBootConfig` from scratch.
//!
//! ```no_run
//! use libsdbootconf::{ConfigBuilder, EntryBuilder, SystemdBootConfBuilder};
//!
//! let systemd_boot_conf = SystemdBootConfBuilder::new("/efi/loader")
//!     .config(ConfigBuilder::new()
//!         .default("5.12.0-aosc-main")
//!         .timeout(5u32)
//!         .build())
//!     .entry(EntryBuilder::new("5.12.0-aosc-main")
//!         .title("AOSC OS x86_64 (5.12.0-aosc-main)")
//!         .version("5.12.0-aosc-main")
//!         .build())
//!     .build();
//!
//! // Or
//! use libsdbootconf::{Config, Entry, SystemdBootConf, Token};
//!
//! let systemd_boot_conf = SystemdBootConf::new(
//!     "/efi/loader",
//!     Config::new(Some("5.12.0-aosc-main"), Some(5u32)),
//!     vec![Entry::new(
//!         "5.12.0-aosc-main",
//!         vec![
//!             Token::Title("AOSC OS x86_64 (5.12.0-aosc-main)".to_owned()),
//!             Token::Version("5.12.0-aosc-main".to_owned()),
//!         ],
//!     )]
//! );
//!
//! systemd_boot_conf.write_all().unwrap();
//! ```
//!
//! # Create a new systemd-boot menu entry
//!
//! ```no_run
//! use libsdbootconf::entry::{Entry, EntryBuilder, Token};
//! use std::path::PathBuf;
//!
//! let entry = EntryBuilder::new("5.12.0-aosc-main")
//!     .title("AOSC OS x86_64 (5.12.0-aosc-main)")
//!     .linux("/EFI/linux/vmlinux-5.12.0-aosc-main")
//!     .initrd("/EFI/linux/initramfs-5.12.0-aosc-main.img")
//!     .options("root=/dev/sda1 rw")
//!     .build();
//!
//! // Or
//! let entry = Entry::new(
//!     "5.12.0-aosc-main",
//!     vec![
//!         Token::Title("AOSC OS x86_64 (5.12.0-aosc-main)".to_owned()),
//!         Token::Linux(PathBuf::from("/EFI/linux/vmlinux-5.12.0-aosc-main")),
//!         Token::Initrd(PathBuf::from("/EFI/linux/initramfs-5.12.0-aosc-main.img")),
//!         Token::Options("root=/dev/sda1 rw".to_owned()),
//!     ],
//! );
//!
//! entry.write("/efi/loader/entries/5.12.0-aosc-main.conf").unwrap();
//! ```

use std::{
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;

pub mod config;
pub mod entry;
mod macros;

use crate::macros::generate_builder_method;
pub use config::{Config, ConfigBuilder};
pub use entry::{Entry, EntryBuilder, Token};

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
    #[error("invalid token {0}")]
    InvalidToken(String),
}

/// An abstraction over the basic structure of systemd-boot configurations.
#[derive(Default, Debug)]
pub struct SystemdBootConf {
    pub working_dir: PathBuf,
    pub config: Config,
    pub entries: Vec<Entry>,
}

impl SystemdBootConf {
    /// Create a new `SystemdBootConf` with a working directory, a configuration, and a list of
    /// entries.
    ///
    /// # Examples
    ///
    /// ```
    /// use libsdbootconf::SystemdBootConf;
    ///
    /// let systemd_boot_conf = SystemdBootConf::init("/efi/loader");
    ///
    /// assert_eq!(systemd_boot_conf.working_dir, std::path::PathBuf::from("/efi/loader"));
    /// ```
    pub fn new<P, C, E>(working_dir: P, config: C, entries: E) -> Self
    where
        P: Into<PathBuf>,
        C: Into<Config>,
        E: Into<Vec<Entry>>, 
    {
        Self {
            working_dir: working_dir.into(),
            config: config.into(),
            entries: entries.into(),
        }
    }

    /// Initialize a new `SystemdBootConf` with a working directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use libsdbootconf::SystemdBootConf;
    ///
    /// let systemd_boot_conf = SystemdBootConf::init("/efi/loader");
    ///
    /// assert_eq!(systemd_boot_conf.working_dir, std::path::PathBuf::from("/efi/loader"));
    /// ```
    pub fn init<P: Into<PathBuf>>(working_dir: P) -> Self {
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
    pub fn load<P: AsRef<Path>>(working_dir: P) -> Result<Self, LibSDBootConfError> {
        let mut systemd_boot_conf = Self::init(working_dir.as_ref());

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
    /// let mut systemd_boot_conf = SystemdBootConf::init("/efi/loader");
    ///
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

    /// Write systemd-boot configuration file to the system.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libsdbootconf::SystemdBootConf;
    ///
    /// let systemd_boot_conf = SystemdBootConf::init("/efi/loader");
    ///
    /// systemd_boot_conf.write_config().unwrap();
    /// ```
    pub fn write_config(&self) -> Result<(), LibSDBootConfError> {
        self.config.write(self.working_dir.join("loader.conf"))?;
        
        Ok(())
    }

    /// Write all entries to the system.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libsdbootconf::SystemdBootConf;
    ///
    /// let systemd_boot_conf = SystemdBootConf::init("/efi/loader");
    ///
    /// systemd_boot_conf.write_entries().unwrap();
    /// ```
    pub fn write_entries(&self) -> Result<(), LibSDBootConfError> {
        for entry in self.entries.iter() {
            entry.write(
                self.working_dir
                    .join("entries")
                    .join(format!("{}.conf", entry.id)),
            )?;
        }

        Ok(())
    }
 
    /// Write all configurations and entries to the system.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libsdbootconf::SystemdBootConf;
    ///
    /// let systemd_boot_conf = SystemdBootConf::init("/efi/loader");
    ///
    /// systemd_boot_conf.write_all().unwrap();
    /// ```
    pub fn write_all(&self) -> Result<(), LibSDBootConfError> {
        self.write_config()?;
        self.write_entries()?;

        Ok(())
    }
}

/// Builder for `SystemdBootConf`.
#[derive(Default, Debug)]
pub struct SystemdBootConfBuilder {
    systemd_boot_conf: SystemdBootConf,
}

impl SystemdBootConfBuilder {
    /// Create an empty `SystemdBootConfBuilder` with a working directory.
    pub fn new<P: Into<PathBuf>>(working_dir: P) -> Self {
        Self {
            systemd_boot_conf: SystemdBootConf::init(working_dir),
        }
    }

    generate_builder_method!(
        /// Add a systemd-boot loader `Config`.
        plain INNER(systemd_boot_conf) config(Config)
    );
    generate_builder_method!(
        /// Add a list of `Entry`.
        into INNER(systemd_boot_conf) entries(E: Vec<Entry>)
    );

    /// Add an `Entry`
    pub fn entry(mut self, entry: Entry) -> Self {
        self.systemd_boot_conf.entries.push(entry);

        self
    }

    /// Build the `SystemdBootConf`.
    pub fn build(self) -> SystemdBootConf {
        self.systemd_boot_conf
    }
}
