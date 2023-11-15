//! The Steam API client

use crate::errors::SteamError;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::Serialize;
use serde_json::Value;

/// Common functions for a Steam API client.
pub trait ApiClient {
    /// Send a GET request and return JSON
    fn get_request<T: Serialize>(
        &self,
        endpoint: &str,
        query: Vec<(&str, T)>,
    ) -> Result<Value, SteamError>;
}

/// This struct holds the blocking reqwest client and is used to interact with the API.
pub struct SteamClient {
    /// A [`reqwest::blocking`] HTTP client
    client: Client,
    /// The dev's Steam API key
    api_key: String,
}

impl ApiClient for SteamClient {
    fn get_request<T: Serialize>(
        &self,
        endpoint: &str,
        query: Vec<(&str, T)>,
    ) -> Result<Value, SteamError> {
        let response = self
            .client
            .get(endpoint)
            .query(&[("key", self.api_key.clone())])
            .query(&query)
            .send();

        match response {
            Ok(r) => match r.status() {
                StatusCode::OK => Ok(r.json().unwrap()),  // we trust steam that we'll actually get json w/ a 200 response, so unwrap() is good enough
                StatusCode::UNAUTHORIZED => {
                    Err(SteamError::FailedRequest("Unauthorized. Either you have used an invalid API key, or the data you wanted to access is private".to_string()))
                }
                _ => Err(SteamError::FailedRequest(
                    "Steam could not process your request. Double-check your provided Steam IDs.".to_string(),
                )),
            },
            Err(_) => Err(SteamError::FailedRequest(
                "Something went wrong with your request".to_string(),
            )),
        }
    }
}

impl SteamClient {
    /// Returns a new SteamClient instance.
    pub fn new(api_key: String) -> Self {
        let client = reqwest::blocking::Client::new();
        SteamClient { client, api_key }
    }
}
