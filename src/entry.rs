//! Configuration of a boot entry.
//!
//! Create an `Entry` by using either `Entry::new()` or `EntryBuilder` to build an `Entry` from
//! scratch.
//!
//! Tokens are the possible fields of an `Entry`, visit
//! <https://www.freedesktop.org/wiki/Software/systemd/systemd-boot/> for more information.
//!
//! # Examples
//!
//! ```
//! use libsdbootconf::entry::{Entry, EntryBuilder, Token};
//!
//! let entry = Entry::new(
//!     "5.12.0-aosc-main",
//!     vec![Token::Title("AOSC OS x86_64 (5.12.0-aosc-main)".to_owned())]
//! );
//! let built = EntryBuilder::new("5.12.0-aosc-main")
//!     .title("AOSC OS x86_64 (5.12.0-aosc-main)")
//!     .build();
//!
//! assert_eq!(entry.to_string(), built.to_string());

use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::{generate_builder_method, LibSDBootConfError};

/// Possible fields of an `Entry`.
#[derive(Debug, PartialEq)]
pub enum Token {
    /// Text to show in the menu.
    Title(String),
    /// Version string to append to the title when the title is not unique.
    Version(String),
    /// Machine identifier to append to the title when the title is not unique.
    MachineID(String),
    /// Executable EFI image.
    Efi(PathBuf),
    /// Options to pass to the EFI image / kernel command line
    Options(String),
    /// Linux kernel image (systemd-boot still requires the kernel to have an EFI stub)
    Linux(PathBuf),
    /// Initramfs image (systemd-boot just adds this as option initrd=)
    Initrd(PathBuf),
}

impl FromStr for Token {
    type Err = LibSDBootConfError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, ' ');
        let key = parts.next().ok_or(LibSDBootConfError::EntryParseError)?;
        let value = parts.next().ok_or(LibSDBootConfError::EntryParseError)?;

        Ok(match key {
            "title" => Self::Title(value.to_owned()),
            "version" => Self::Version(value.to_owned()),
            "machine-id" => Self::MachineID(value.to_owned()),
            "efi" => Self::Efi(PathBuf::from(value)),
            "options" => Self::Options(value.to_owned()),
            "linux" => Self::Linux(PathBuf::from(value)),
            "initrd" => Self::Initrd(PathBuf::from(value)),
            _ => return Err(LibSDBootConfError::InvalidToken(key.to_owned())),
        })
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Self::Title(title) => format!("title {}\n", title),
            Self::Version(version) => format!("version {}\n", version),
            Self::MachineID(machine_id) => format!("machine-id {}\n", machine_id),
            Self::Efi(efi) => format!("efi {}\n", efi.display()),
            Self::Options(options) => format!("options {}\n", options),
            Self::Linux(linux) => format!("linux {}\n", linux.display()),
            Self::Initrd(initrd) => format!("initrd {}\n", initrd.display()),
        }
    }
}

/// A boot menu entry.
#[derive(Default, Debug, PartialEq)]
pub struct Entry {
    /// The ID of the `Entry`, used in the filename of the entry and the `default` field in a
    /// `Config`.
    pub id: String,
    /// The fields of the `Entry`.
    pub tokens: Vec<Token>,
}

impl FromStr for Entry {
    type Err = LibSDBootConfError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entry = Entry::default();
        let lines = s.lines();

        for line in lines {
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            entry.tokens.push(line.parse()?);
        }

        Ok(entry)
    }
}

impl ToString for Entry {
    fn to_string(&self) -> String {
        let mut s = String::new();

        for token in &self.tokens {
            s.push_str(&token.to_string())
        }

        s
    }
}

impl Entry {
    /// Create a new `Entry`.
    ///
    /// # Examples
    ///
    /// ```
    /// use libsdbootconf::entry::Entry;
    /// use libsdbootconf::entry::Token;
    ///
    /// let entry = Entry::new(
    ///     "5.12.0-aosc-main",
    ///     vec![Token::Title("5.12.0-aosc-main".to_string())],
    /// );
    ///
    /// assert_eq!(entry.id, "5.12.0-aosc-main");
    /// println!("{:?}", entry.tokens); // [Token::Title("title 5.12.0-aosc-main")]
    /// ```
    pub fn new<S, T>(id: S, tokens: T) -> Entry
    where
        S: Into<String>,
        T: IntoIterator<Item = Token>,
    {
        Entry {
            id: id.into(),
            tokens: tokens.into_iter().collect(),
        }
    }

    /// Load an existing entry file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libsdbootconf::entry::Entry;
    ///
    /// let entry = Entry::load("/path/to/config").unwrap();
    /// ```
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Entry, LibSDBootConfError> {
        let id = path
            .as_ref()
            .file_name()
            .ok_or_else(|| LibSDBootConfError::InvalidEntryFilename(path.as_ref().to_owned()))?
            .to_str()
            .ok_or_else(|| LibSDBootConfError::InvalidEntryFilename(path.as_ref().to_owned()))?
            .strip_suffix(".conf")
            .ok_or_else(|| LibSDBootConfError::InvalidEntryFilename(path.as_ref().to_owned()))?;
        let mut entry = Entry::from_str(&fs::read_to_string(path.as_ref())?)?;

        entry.id = id.to_owned();

        Ok(entry)
    }

    /// Save the entry to a file under the given path.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libsdbootconf::entry::Entry;
    /// use libsdbootconf::entry::Token;
    ///
    /// let entry = Entry::new(
    ///     "5.12.0-aosc-main",
    ///     vec![Token::Title("5.12.0-aosc-main".to_string())],
    /// );
    /// entry.write("/path/to/entry").unwrap();
    /// ```
    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<(), LibSDBootConfError> {
        fs::write(path, self.to_string())?;

        Ok(())
    }
}

/// Builder for `Entry`.
#[derive(Default, Debug)]
pub struct EntryBuilder {
    inner: Entry,
}

impl EntryBuilder {
    /// Build an empty `EntryBuilder` with an inner id.
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self {
            inner: Entry::new(id, Vec::new()),
        }
    }

    generate_builder_method!(
        /// Add a `Title` to the inner.
        token Token::Title INNER(inner) title(S: String)
    );
    generate_builder_method!(
        /// Add a `Version` to the inner.
        token Token::Version INNER(inner) version(S: String)
    );
    generate_builder_method!(
        /// Add a `MachineID` to the inner.
        token Token::MachineID INNER(inner) machine_id(S: String)
    );
    generate_builder_method!(
        /// Add an `Efi` to the inner.
        token Token::Efi INNER(inner) efi(P: PathBuf)
    );
    generate_builder_method!(
        /// Add an `Options` to the inner.
        token Token::Options INNER(inner) options(S: String)
    );
    generate_builder_method!(
        /// Add a `Linux` to the inner.
        token Token::Linux INNER(inner) linux(P: PathBuf)
    );
    generate_builder_method!(
        /// Add an `Initrd` to the inner.
        token Token::Initrd INNER(inner) initrd(P: PathBuf)
    );

    /// Build the `Entry`.
    pub fn build(self) -> Entry {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let entry = EntryBuilder::new("5.12.0-aosc-main")
            .title("5.12.0-aosc-main")
            .build();

        println!("{:?}", &entry);
    }
}
