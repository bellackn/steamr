//! Custom error types

use reqwest::Error;
use thiserror::Error;

/// Represents an error that was returned by a Steam API endpoint.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SteamError {
    /// A reqwest failed for some reason
    #[error("Error response from steam: {0}")]
    FailedRequest(String),
    /// The requested data is either private, or not present at all. Usually, this comes from a
    /// deserialization error in serde.
    #[error("The data you requested is either private or empty")]
    NoData,
}

#[doc(hidden)]
impl From<reqwest::Error> for SteamError {
    fn from(err: Error) -> Self {
        // If the reqwest goes wrong, we should forward it to the user
        let reqwest_error = err.to_string();
        Self::FailedRequest(reqwest_error)
    }
}
