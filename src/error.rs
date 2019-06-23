use serde::{de, ser};

#[derive(Debug)]
pub enum Error {
    WriteToken(String),
    Deserialization(String),
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::WriteToken(ref string) => string,
            Error::Deserialization(ref string) => string,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::WriteToken(ref string) => write!(f, "{}", string),
            Error::Deserialization(ref string) => write!(f, "{}", string),
        }
    }
}

impl ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Error {
        Error::WriteToken(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Error {
        Error::WriteToken(msg.to_string())
    }
}
