//! The Steam API client

use crate::errors::SteamError;
use log::warn;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

/// This struct holds the blocking reqwest client and is used to interact with the API.
pub struct SteamClient {
    /// A [`reqwest::blocking`] HTTP client
    client: Client,
    /// The dev's Steam API key
    api_key: String,
}

impl Default for SteamClient {
    fn default() -> Self {
        let client = reqwest::blocking::Client::new();
        Self {
            client,
            api_key: String::new(),
        }
    }
}

impl SteamClient {
    /// Returns a new SteamClient instance carrying a developer API token
    pub fn from(api_key: String) -> Self {
        let client = reqwest::blocking::Client::new();
        Self { client, api_key }
    }

    /// Return a SteamClient without a Steam API token
    pub fn new() -> Self {
        let client = reqwest::blocking::Client::new();
        Self {
            client,
            api_key: String::new(),
        }
    }

    pub(crate) fn get_request<T: Serialize>(
        &self,
        endpoint: &str,
        query: Vec<(&str, T)>,
    ) -> Result<Value, SteamError> {
        if self.api_key.is_empty() {
            warn!("Not using a valid API key. Is this on purpose?")
        }
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
                    "Steam could not process your request. Double-check your provided parameters (Steam ID, app ID, ...).".to_string(),
                )),
            },
            Err(_) => Err(SteamError::FailedRequest(
                "Something went wrong with your request".to_string(),
            )),
        }
    }

    pub(crate) fn parse_response<R: DeserializeOwned, S: From<R> + DeserializeOwned>(
        &self,
        response: Value,
    ) -> Result<S, SteamError> {
        let res = serde_json::from_value::<R>(response);
        if let Ok(v) = res {
            Ok(v.into())
        } else {
            Err(SteamError::NoData)
        }
    }
}
