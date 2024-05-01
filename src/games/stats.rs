//! Implementation for the GetUserStatsForGame endpoint

use crate::client::SteamClient;
use crate::errors::SteamError;
use serde::Deserialize;

/// The Steam API "GetUserStatsForGame (v0002) endpoint
const ENDPOINT_USER_STATS_FOR_GAME: &str =
    "https://api.steampowered.com/ISteamUserStats/GetUserStatsForGame/v0002";

/// Helper struct used during deserialization of the API response.
#[derive(Debug, Deserialize)]
struct PlayerStatsResponse {
    #[serde(rename(deserialize = "playerstats"))]
    response: Option<PlayerStats>,
}

/// This struct holds the player statistics.
#[derive(Debug, Default, Deserialize)]
pub struct PlayerStats {
    #[serde(rename(deserialize = "gameName"))]
    /// Name of the game
    pub game_name: String,
    /// List of achievements
    pub achievements: Vec<Achievement>,
    /// List of other stats
    pub stats: Vec<Stat>,
}

impl From<PlayerStatsResponse> for PlayerStats {
    fn from(value: PlayerStatsResponse) -> Self {
        let v = value.response.unwrap_or_default();
        Self {
            game_name: v.game_name,
            achievements: v.achievements,
            stats: v.stats,
        }
    }
}

/// A single achievement.
#[derive(Debug, Deserialize)]
pub struct Achievement {
    /// Name of the achievement.
    pub name: String,
}

/// A stat.
#[derive(Debug, Deserialize)]
pub struct Stat {
    /// Name of the stat.
    pub name: String,
    /// Value of the stat.
    pub value: u16,
}

impl SteamClient {
    /// Returns the stats of a given player and app ID.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use steamr::client::SteamClient;
    /// # use steamr::errors::SteamError;
    ///
    /// fn main() -> Result<(), SteamError> {
    ///     let steam_client = SteamClient::from("an-api-key".to_string());
    ///     let player_stats = steam_client.get_player_stats("some-steam-ID", "some-app-ID")?;
    ///
    ///     println!("Showing stats for the game '{}'", &player_stats.game_name);
    ///
    ///     player_stats.achievements.iter().for_each(|a| println!("Achievement: {}", a.name));
    ///     player_stats.stats.iter().for_each(|s| println!("Stat: {}, Value: {}", s.name, s.value));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get_player_stats(
        &self,
        steam_id: &str,
        app_id: &str,
    ) -> Result<PlayerStats, SteamError> {
        let response = self.get_request(
            ENDPOINT_USER_STATS_FOR_GAME,
            vec![("steamid", steam_id), ("appid", app_id)],
        )?;

        let stats = self
            .parse_response::<PlayerStatsResponse, PlayerStats>(response)
            .unwrap();
        Ok(stats)
    }
}
