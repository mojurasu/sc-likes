use std::{error, fmt};

#[derive(Debug, Clone)]
pub struct SoundCloudError {
    pub message: String,
}

impl fmt::Display for SoundCloudError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for SoundCloudError {}

impl From<reqwest::Error> for SoundCloudError {
    fn from(item: reqwest::Error) -> Self {
        SoundCloudError { message: format!("An error occured during the HTTP request: {}", item) }
    }
}

impl From<serde_json::Error> for SoundCloudError {
    fn from(item: serde_json::Error) -> Self {
        SoundCloudError { message: format!("An error occured during deserializing: {}", item) }
    }
}

impl From<std::io::Error> for SoundCloudError {
    fn from(item: std::io::Error) -> Self {
        SoundCloudError { message: format!("An internal error occured: {}", item) }
    }
}

