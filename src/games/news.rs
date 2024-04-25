//! Apps on Steam and news about them.

use serde::Deserialize;

use crate::{
    client::{ApiClient, SteamClient},
    errors::SteamError,
};

/// The Steam API "GetNewsForApp (v0002)" endpoint
const ENDPOINT_GAME_NEWS: &str = "https://api.steampowered.com/ISteamNews/GetNewsForApp/v0002";

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

impl SteamClient {
    /// Returns the news for a game.
    ///
    /// You can control both the number of news that you want to fetch and
    /// the maximum length of the news content (although this does not work strictly, for example when
    /// it contains hyperlinks, so this is not a number to rely on!)
    ///
    /// This endpoint doesn't necessarily require an API key.
    ///
    /// Example:
    ///
    /// ```
    /// # use steamr::client::SteamClient;
    /// # use steamr::errors::SteamError;
    /// fn main() -> Result<(), SteamError> {
    ///     let steam_client = SteamClient::from("an-api-key".to_string());
    ///     let app_id ="10";  // This is CS:GO
    ///     let news = steam_client.get_game_news(app_id, 5, 100)?;
    ///
    ///     news.game_news.iter()
    ///         .for_each(|n| println!("The article '{}' was written by '{}'", n.title, n.author));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn get_game_news(
        &self,
        app_id: &str,
        news_count: u16,
        max_length: u16,
    ) -> Result<GameNews, SteamError> {
        let response = self.get_request(
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
}
