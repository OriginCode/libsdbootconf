//! This library provides a general purpose configuration interface for systemd-boot.
//! It parses systemd-boot configuration and systemd-boot entry configuration.

use std::{
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;

pub mod config;
pub mod entry;

pub use config::Config;
pub use entry::Entry;

#[derive(Error, Debug)]
pub enum LibSDBootError {
    #[error("invalid configuration")]
    ConfigParseError,
    #[error("invalid entry")]
    EntryParseError,
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

///
#[derive(Debug)]
pub struct SystemdBoot {
    pub working_dir: PathBuf,
    pub config: Config,
    pub entries: Vec<Entry>,
}

impl SystemdBoot {
    pub fn new(working_dir: impl AsRef<Path>) -> Result<Self, LibSDBootError> {
        let config = Config::load(working_dir.as_ref().join("loader.conf"))?;
        let mut entries = Vec::new();

        for file in fs::read_dir(working_dir.as_ref().join("entries"))? {
            let path = file?.path();
            if path.is_file() {
                let entry = Entry::load(&path)?;
                entries.push(entry);
            }
        }

        Ok(Self {
            working_dir: working_dir.as_ref().to_owned(),
            config,
            entries,
        })
    }
}
