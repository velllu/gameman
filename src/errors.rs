use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum EmuError {
    CouldNotFindRom,
    CouldNotReadRom,
}

impl Error for EmuError {}
impl Display for EmuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CouldNotFindRom => write!(f, "could not find rom"),
            Self::CouldNotReadRom => write!(f, "could not read rom"),
        }
    }
}
