//! This library provides a general purpose configuration interface for systemd-boot.
//! It parses systemd-boot configuration and systemd-boot entry configuration.

use thiserror::Error;

pub mod config;
pub mod entry;

pub use config::Config;
pub use entry::Entry;

#[derive(Error, Debug)]
pub enum LibSDBootError {
    #[error("configuration at {0} not found")]
    ConfigNotFound(String),
    #[error("invalid configuration")]
    ConfigParseError,
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

pub struct SystemdBoot {
    working_dir: String,
    config: Config,
    entries: Vec<Entry>,
}