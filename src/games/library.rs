//! This module deals with a user's games library.

use serde::Deserialize;
use std::fmt::Formatter;

use crate::{
    client::{ApiClient, SteamClient},
    errors::SteamError,
};

/// The Steam API "GetOwnedGames (v0001)" endpoint
const ENDPOINT_OWNED_GAMES: &str =
    "https://api.steampowered.com/IPlayerService/GetOwnedGames/v0001";

/// Helper struct used during deserializing the API response.
#[derive(Debug, Deserialize)]
struct OwnedGamesResponse {
    response: Option<Library>,
}

/// This is the response that comes from the GetOwnedGames API.
#[derive(Debug, Deserialize)]
pub struct Library {
    /// Number of games in a user's library
    pub game_count: u32,
    /// List of [`Game`]s in a user's library
    pub games: Vec<Game>,
}

/// Represents a game and its metadata.
#[derive(Debug, Deserialize)]
pub struct Game {
    #[serde(rename(deserialize = "appid"))]
    /// Game ID
    pub app_id: u64,
    /// Name of the game
    pub name: String,
    /// Total playtime in minutes
    pub playtime_forever: u64,
    /// Playtime in minutes on Windows
    pub playtime_windows_forever: u64,
    /// Playtime in minutes on Mac
    pub playtime_mac_forever: u64,
    /// Playtime in minutes on Linux
    pub playtime_linux_forever: u64,
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.app_id == other.app_id
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Game: {}, total time played: {}",
            self.name, self.playtime_forever
        )
    }
}

impl SteamClient {
    /// Gets all games that are owned by the user with the given Steam ID.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use steamr::client::SteamClient;
    /// # use steamr::errors::SteamError;
    /// fn main() -> Result<(), SteamError> {
    ///     let steam_client = SteamClient::from("an-api-key".to_string());
    ///     let steam_id = "some-steam-id";
    ///     let steam_lib = steam_client.get_library(&steam_id)?;
    ///
    ///     // Print out games that were played for more than an hour.
    ///     steam_lib.games.iter()
    ///         .filter(|g| g.playtime_forever > 60)
    ///         .for_each(|g| println!("{}", g.name));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get_library(&self, steam_id: &str) -> Result<Library, SteamError> {
        let response = self.get_request(
            ENDPOINT_OWNED_GAMES,
            vec![
                ("steamid", steam_id),
                ("include_appInfo", "true"),
                ("include_played_free_games", "true"),
            ],
        )?;

        let _res: OwnedGamesResponse =
            serde_json::from_value(response).unwrap_or(OwnedGamesResponse { response: None });

        match _res.response {
            None => Err(SteamError::NoData),
            Some(v) => Ok(v),
        }
    }
}
