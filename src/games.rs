//! Contains all functionalities around games

use crate::client::{ApiClient, SteamClient};
use crate::errors::SteamError;
use serde::Deserialize;
use std::fmt::Formatter;

/// The Steam API "GetOwnedGames (v0001)" endpoint
const ENDPOINT_OWNED_GAMES: &str =
    "https://api.steampowered.com/IPlayerService/GetOwnedGames/v0001";

/// The Steam API "GetNewsForApp (v0002)" endpoint
const ENDPOINT_GAME_NEWS: &str = "https://api.steampowered.com/ISteamNews/GetNewsForApp/v0002";

/// The Steam API "GetUserStatsForGame (v0002) endpoint
const ENDPOINT_USER_STATS_FOR_GAME: &str =
    "https://api.steampowered.com/ISteamUserStats/GetUserStatsForGame/v0002";

/// Helper struct used during deserializing the API response.
#[derive(Debug, Deserialize)]
struct OwnedGamesResponse {
    response: Option<OwnedGames>,
}

/// This is the response that comes from the GetOwnedGames API.
#[derive(Debug, Deserialize)]
pub struct OwnedGames {
    /// Number of games in a user's library
    pub game_count: u64,
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

/// Helper struct used during deserialization of the API response.
#[derive(Debug, Deserialize)]
struct UserStatsResponse {
    #[serde(rename(deserialize = "playerstats"))]
    response: Option<PlayerStats>,
}

/// This struct holds the player statistics.
#[derive(Debug, Deserialize)]
pub struct PlayerStats {
    #[serde(rename(deserialize = "gameName"))]
    /// Name of the game
    pub game_name: String,
    /// List of achievements
    pub achievements: Vec<Achievement>,
    /// List of other stats
    pub stats: Vec<Stat>,
}

/// A single achievement.
#[derive(Debug, Deserialize)]
pub struct Achievement {
    /// Name of the achievement.
    pub name: String,
}

/// A list of stats.
#[derive(Debug, Deserialize)]
pub struct Stat {
    /// Name of the stat.
    pub name: String,
    /// Value of the stat.
    pub value: u16,
}

/// Gets all games that are owned by the user with the given Steam ID.
///
/// Example:
///
/// ```no_run
/// # use steamr::client::SteamClient;
/// # use steamr::games::get_owned_games;
/// # use steamr::errors::SteamError;
/// fn main() -> Result<(), SteamError> {
///     let steam_client = SteamClient::new("an-api-key".to_string());
///     let steam_id = "some-steam-id";
///     let steam_lib = get_owned_games(&steam_client, &steam_id)?;
///
///     // Print out games that were played for more than an hour.
///     steam_lib.games.iter()
///         .filter(|g| g.playtime_forever > 60)
///         .for_each(|g| println!("{}", g.name));
///
///     Ok(())
/// }
/// ```
pub fn get_owned_games(client: &SteamClient, steam_id: &str) -> Result<OwnedGames, SteamError> {
    let response = client.get_request(
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

/// Helper struct used during deserializing the API response.
#[derive(Debug, Deserialize)]
struct GameNewsResponse {
    #[serde(rename(deserialize = "appnews"))]
    response: Option<GameNews>,
}

/// Response from the GetNewsForApp API
#[derive(Debug, Deserialize)]
pub struct GameNews {
    /// List of [`News`] for a given game ID
    #[serde(rename(deserialize = "newsitems"))]
    pub game_news: Vec<News>,
    /// The total number of available news for the given game ID
    pub count: i16,
}

/// A Steam news object
#[derive(Debug, Deserialize)]
pub struct News {
    /// News ID
    #[serde(rename(deserialize = "gid"))]
    pub news_id: String,
    /// Title
    pub title: String,
    /// URL
    pub url: String,
    /// News author
    pub author: String,
    /// News content
    pub contents: String,
    /// Date as UNIX timestamp
    pub date: i64,
    /// Name of the feed
    #[serde(rename(deserialize = "feedname"))]
    pub feed_name: String,
}

/// Returns the news for a game.
///
/// You can control both the number of news tha you want to fetch and
/// the maximum length of the news content (although this does not work strictly, for example when
/// it contains hyperlinks, so this is not a number to rely on!)
///
/// Example:
///
/// ```
/// # use steamr::client::SteamClient;
/// # use steamr::games::get_game_news;
/// # use steamr::errors::SteamError;
/// fn main() -> Result<(), SteamError> {
///     let steam_client = SteamClient::new("an-api-key".to_string());
///     let app_id ="10";  // This is CS:GO
///     let news = get_game_news(&steam_client, app_id, 5, 100)?;
///
///     news.game_news.iter()
///         .for_each(|n| println!("The article '{}' was written by '{}'", n.title, n.author));
///
///     Ok(())
/// }
/// ```
pub fn get_game_news(
    client: &SteamClient,
    app_id: &str,
    news_count: u16,
    max_length: u16,
) -> Result<GameNews, SteamError> {
    let response = client.get_request(
        ENDPOINT_GAME_NEWS,
        vec![
            ("appid", app_id),
            ("count", &news_count.to_string()),
            ("maxlength", &max_length.to_string()),
        ],
    )?;

    let _res: GameNewsResponse =
        serde_json::from_value(response).unwrap_or(GameNewsResponse { response: None });

    match _res.response {
        None => Err(SteamError::NoData),
        Some(v) => Ok(v),
    }
}

/// Returns the stats of a given player and app ID.
///
/// Example:
///
/// ```no_run
/// # use steamr::client::SteamClient;
/// # use steamr::games::get_player_stats;
/// # use steamr::errors::SteamError;
///
/// fn main() -> Result<(), SteamError> {
///     let steam_client = SteamClient::new("an-api-key".to_string());
///     let player_stats = get_player_stats(&steam_client, "some-steam-ID", "some-app-ID")?;
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
    client: &SteamClient,
    steam_id: &str,
    app_id: &str,
) -> Result<PlayerStats, SteamError> {
    let response = client.get_request(
        ENDPOINT_USER_STATS_FOR_GAME,
        vec![("steamid", steam_id), ("appid", app_id)],
    )?;

    let _res: UserStatsResponse =
        serde_json::from_value(response).unwrap_or(UserStatsResponse { response: None });

    match _res.response {
        None => Err(SteamError::NoData),
        Some(v) => Ok(v),
    }
}
