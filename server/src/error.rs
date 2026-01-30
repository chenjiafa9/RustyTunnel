use std::fmt;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ConfigError(String),
    CryptoError(String),
    DeviceError(String),
    NetworkError(String),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "IO Error: {}", e),
            Error::ConfigError(e) => write!(f, "Config Error: {}", e),
            Error::CryptoError(e) => write!(f, "Crypto Error: {}", e),
            Error::DeviceError(e) => write!(f, "Device Error: {}", e),
            Error::NetworkError(e) => write!(f, "Network Error: {}", e),
            Error::Other(e) => write!(f, "Error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::ConfigError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
