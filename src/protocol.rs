use quick_error::quick_error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvalidProtocol(protocol: String) {
            display("can't parse '{}' to a valid protocol", protocol)
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "http")]
    Http,

    #[serde(rename = "https")]
    Https,

    #[serde(rename = "rsync")]
    Rsync,
}

impl FromStr for Protocol {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "http" => Ok(Self::Http),
            "https" => Ok(Self::Https),
            "rsync" => Ok(Self::Rsync),
            other => Err(Error::InvalidProtocol(other.into())),
        }
    }
}
