//! Custom error types

use reqwest::Error;
use thiserror::Error;

/// Represents an error that was returned by a Steam API endpoint.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SteamError {
    #[error("Error response from steam: {0}")]
    FailedRequest(String),
    #[error("Error with (de-)serializing JSON response")]
    Serde,
}

impl From<serde_json::Error> for SteamError {
    fn from(_: serde_json::Error) -> Self {
        Self::Serde
    }
}

impl From<reqwest::Error> for SteamError {
    fn from(err: Error) -> Self {
        // If the reqwest goes wrong, we should forward it to the user
        let reqwest_error = err.to_string();
        Self::FailedRequest(reqwest_error)
    }
}
