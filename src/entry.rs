use std::path::PathBuf;

pub struct Entry {
    pub tokens: Vec<Token>
}

pub enum Token {
    Title(String),
    Version(String),
    MachineID(String),
    Efi(PathBuf),
    Options(String),
    Linux(PathBuf),
    Initrd(PathBuf),
}