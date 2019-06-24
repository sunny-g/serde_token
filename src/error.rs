#[doc(hidden)]
#[derive(Debug)]
pub enum Error {
    TokenizerError(String),
    TokenSinkError,
    TokenSinkNotReadyError,
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::TokenizerError(ref string) => string,
            Error::TokenSinkError => "",
            Error::TokenSinkNotReadyError => "",
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::TokenizerError(ref string) => write!(f, "{}", string),
            Error::TokenSinkError => write!(f, ""),
            Error::TokenSinkNotReadyError => write!(f, ""),
        }
    }
}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::TokenizerError(msg.to_string())
    }
}
