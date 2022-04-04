use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::LibSDBootError;

#[derive(Default, Debug)]
pub struct Entry {
    pub id: String,
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub enum Token {
    Title(String),
    Version(String),
    MachineID(String),
    Efi(PathBuf),
    Options(String),
    Linux(PathBuf),
    Initrd(PathBuf),
}

impl FromStr for Entry {
    type Err = LibSDBootError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entry = Entry::default();
        let lines = s.lines();

        for line in lines {
            if line.starts_with('#') {
                continue;
            }

            let mut parts = line.splitn(2, ' ');
            let key = parts.next().ok_or(LibSDBootError::ConfigParseError)?;
            let value = parts.next().ok_or(LibSDBootError::ConfigParseError)?;

            entry.tokens.push(match key {
                "title" => Token::Title(value.to_owned()),
                "version" => Token::Version(value.to_owned()),
                "machine-id" => Token::MachineID(value.to_owned()),
                "efi" => Token::Efi(PathBuf::from(value)),
                "options" => Token::Options(value.to_owned()),
                "linux" => Token::Linux(PathBuf::from(value)),
                "initrd" => Token::Initrd(PathBuf::from(value)),
                _ => continue,
            })
        }

        Ok(entry)
    }
}

impl ToString for Entry {
    fn to_string(&self) -> String {
        let mut s = String::new();

        for token in &self.tokens {
            s.push_str(&match token {
                Token::Title(title) => format!("title {}\n", title),
                Token::Version(version) => format!("version {}\n", version),
                Token::MachineID(machine_id) => format!("machine-id {}\n", machine_id),
                Token::Efi(efi) => format!("efi {}\n", efi.display()),
                Token::Options(options) => format!("options {}\n", options),
                Token::Linux(linux) => format!("linux {}\n", linux.display()),
                Token::Initrd(initrd) => format!("initrd {}\n", initrd.display()),
            })
        }

        s
    }
}

impl Entry {
    /// Create a new Entry.
    ///
    /// # Examples
    ///
    /// ```
    /// use libsdboot::entry::Entry;
    /// use libsdboot::entry::Token;
    ///
    /// let entry = Entry::new(
    ///     "5.12.0-aosc-main",
    ///     vec![Token::Title("5.12.0-aosc-main".to_string())],
    /// );
    ///
    /// assert_eq!(config.default, "5.12.0-aosc-main");
    /// assert_eq!(config.timeout, 5);
    /// ```
    pub fn new(id: impl Into<String>, tokens: impl Into<Vec<Token>>) -> Entry {
        Entry {
            id: id.into(),
            tokens: tokens.into(),
        }
    }

    /// Load an existing entry file.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libsdboot::entry::Entry;
    ///
    /// let entry = Entry::load("/path/to/config").unwrap();
    /// ```
    pub fn load(path: impl AsRef<Path>) -> Result<Entry, LibSDBootError> {
        let id = path.as_ref().file_name().unwrap().to_str().unwrap();
        let mut entry = Entry::from_str(&fs::read_to_string(path.as_ref())?)?;

        entry.id = id.to_owned();

        Ok(entry)
    }

    /// Save the entry to a file under the given path.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libsdboot::entry::Entry;
    /// use libsdboot::entry::Token;
    ///
    /// let entry = Entry::new(
    ///     "5.12.0-aosc-main",
    ///     vec![Token::Title("5.12.0-aosc-main".to_string())],
    /// );
    /// entry.write("/path/to/config").unwrap();
    /// ```
    pub fn write(&self, path: impl AsRef<Path>) -> Result<(), LibSDBootError> {
        let dest_path = path.as_ref().join(self.id.as_str());

        fs::write(dest_path, self.to_string())?;

        Ok(())
    }
}
