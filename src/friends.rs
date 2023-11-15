//! Functions to handle any friends-related data

use crate::client::{ApiClient, SteamClient};
use crate::errors::SteamError;
use serde::Deserialize;
use std::fmt::Formatter;

// The "GetFriendList (v0001)" endpoint
const ENDPOINT_GET_FRIENDLIST: &str = "https://api.steampowered.com/ISteamUser/GetFriendList/v0001";

/// Helper struct used during deserializing the API response.
#[derive(Debug, Deserialize)]
struct FriendsResponse {
    /// A list of [`Friend`]s
    #[serde(rename(deserialize = "friendslist"))]
    response: Option<FriendsList>,
}

/// This is the response that comes from the GetFriendList API.
#[derive(Debug, Deserialize)]
pub struct FriendsList {
    /// A list of [`Friend`]s
    pub friends: Vec<Friend>,
}

/// Represents a Steam friend and its metadata
#[derive(Debug, Deserialize, PartialEq)]
pub struct Friend {
    /// The friend's Steam ID
    #[serde(rename(deserialize = "steamid"))]
    pub steam_id: String,
    /// The relationship you have with the Steam user
    pub relationship: SteamRelationship,
    /// Unix timestamp of the time when the relationship was created.
    pub friend_since: i64,
}

impl std::fmt::Display for Friend {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Friend ID: {}, friends since {}",
            self.steam_id, self.friend_since
        )
    }
}

/// Enumeration of possible relationship qualifiers.
#[derive(Debug, Deserialize, PartialEq)]
#[non_exhaustive]
pub enum SteamRelationship {
    /// You are friends with that person
    #[serde(rename(deserialize = "friend"))]
    Friend,
}

/// Gets all friends from the user with the provided Steam ID.
///
/// Example:
///
/// ```ignore
/// // This specific example will not work since the API key is invalid and we're using "?". Also,
/// // chrono is not part of this lib's dependencies.
///
/// # use steamr::client::SteamClient;
/// # use steamr::friends::get_friends;
/// # use steamr::errors::SteamError;
/// fn main() -> Result<(), SteamError> {
///     let steam_client = SteamClient::new("an-api-key".to_string());
///     let steam_friends = get_friends(&steam_client, "some-steam-ID")?;
///
///     // Print friends
///     steam_friends.iter().for_each(|f| println!("{}", f));
///
///     // The standard format of "friends since" is the UNIX timestamp, you might want to get a
///     // more intuitive time format. You could use the `chrono` crate for this:
///     let steam_friends_2 = get_friends(&steam_client, "some-steam-ID")?;
///     steam_friends_2.iter().for_each(|f| {
///         println!(
///             "me and {} are friends since {}",
///             f.steam_id,
///             chrono::NaiveDateTime::from_timestamp(f.friend_since, 0)
///         )
///     });
///
///     Ok(())
/// }
/// ```
pub fn get_friends(client: &SteamClient, steam_id: &str) -> Result<Vec<Friend>, SteamError> {
    let response = client.get_request(
        ENDPOINT_GET_FRIENDLIST,
        vec![("steamid", steam_id), ("relationship", "friend")],
    )?;

    let _res: FriendsResponse =
        serde_json::from_value(response).unwrap_or(FriendsResponse { response: None });

    match _res.response {
        None => Err(SteamError::NoData),
        Some(v) => Ok(v.friends),
    }
}
