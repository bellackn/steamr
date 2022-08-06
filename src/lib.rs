#![doc = include_str!("../README.md")]

#![deny(missing_docs)]
#![deny(rustdoc::missing_doc_code_examples)]

use crate::errors::SteamError;
use reqwest::blocking::{Client, Response};
use reqwest::StatusCode;

pub mod errors;
pub mod owned_games;

/// This struct holds the blocking reqwest client and is used to interact with the API.
pub struct SteamClient {
    /// A [`reqwest::blocking`] HTTP client
    client: Client,
    /// The dev's Steam API key
    api_key: &'static str,
}

impl SteamClient {
    /// Returns a new SteamClient instance.
    pub fn new(api_key: &'static str) -> Self {
        let client = reqwest::blocking::Client::new();
        SteamClient { client, api_key }
    }

    /// A common function used to send requests to Steam's API and to return JSON data.
    fn send_steam_request(
        &self,
        endpoint: &str,
        query_params: Vec<(&str, &str)>,
    ) -> Result<Response, SteamError> {
        let response = self
            .client
            .get(endpoint)
            .query(&[("key", self.api_key)])
            .query(&query_params)
            .send();

        match response {
            Ok(r) => match r.status() {
                StatusCode::OK => Ok(r),
                StatusCode::UNAUTHORIZED => {
                    Err(SteamError::FailedRequest("Invalid API key".to_string()))
                }
                _ => Err(SteamError::FailedRequest(
                    "Unknown response from Steam".to_string(),
                )),
            },
            Err(_) => Err(SteamError::FailedRequest(
                "Something went wrong with your request".to_string(),
            )),
        }
    }
}
