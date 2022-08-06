//! Contains all functionality regarding the GetOwnedGames endpoint

use crate::errors::SteamError;
use crate::SteamClient;
use serde::Deserialize;
use std::fmt::Formatter;

/// The Steam API "GetOwnedGames (v0001)" endpoint
const ENDPOINT_OWNED_GAMES: &str =
    "https://api.steampowered.com/IPlayerService/GetOwnedGames/v0001";

/// Helper struct used during deserializing the API response.
#[derive(Debug, Deserialize)]
struct OwnedGamesResponse {
    response: OwnedGames,
}

/// This is the response that comes from the GetOwnedGames API.
#[derive(Debug, Deserialize)]
pub struct OwnedGames {
    /// Number of games in a user's library
    pub game_count: u16,
    /// List of [`Game`]s in a user's library
    pub games: Vec<Game>,
}

/// Represents a game and its metadata.
#[derive(Debug, Deserialize)]
pub struct Game {
    #[serde(alias = "appid")]
    /// Game ID
    pub app_id: u32,
    /// Name of the game
    pub name: String,
    /// Total playtime in minutes
    pub playtime_forever: u32,
    /// Playtime in minutes on Windows
    pub playtime_windows_forever: u32,
    /// Playtime in minutes on Mac
    pub playtime_mac_forever: u32,
    /// Playtime in minutes on Linux
    pub playtime_linux_forever: u32,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            app_id: 0,
            name: "default".to_string(),
            playtime_forever: 0,
            playtime_windows_forever: 0,
            playtime_mac_forever: 0,
            playtime_linux_forever: 0,
        }
    }
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

/// Gets all games that are owned by the user with the given Steam ID.
pub fn get_owned_games(client: &SteamClient, steam_id: &str) -> Result<OwnedGames, SteamError> {
    let response = client
        .send_steam_request(
            ENDPOINT_OWNED_GAMES,
            vec![
                ("steamid", steam_id),
                ("include_appInfo", "true"),
                ("include_played_free_games", "true"),
            ],
        )?
        .text()?;

    let _ogr: OwnedGamesResponse = serde_json::from_str(&response)?;
    Ok(_ogr.response)
}
