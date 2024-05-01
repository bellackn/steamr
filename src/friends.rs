//! Functionality dealing with an account's friends

use crate::client::SteamClient;
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
#[derive(Debug, Default, Deserialize)]
pub struct FriendsList {
    /// A list of [`Friend`]s
    pub friends: Vec<Friend>,
}

impl From<FriendsResponse> for FriendsList {
    fn from(value: FriendsResponse) -> Self {
        let v = value.response.unwrap_or_default();
        Self { friends: v.friends }
    }
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

impl SteamClient {
    /// Gets all friends from the user with the provided Steam ID.
    ///
    /// Example:
    ///
    /// ```no_run
    /// # use steamr::client::SteamClient;
    /// # use steamr::errors::SteamError;
    ///
    /// fn main() -> Result<(), SteamError> {
    ///     let steam_client = SteamClient::from("an-api-key".to_string());
    ///     let steam_friends = steam_client.get_friends("some-steam-ID")?;
    ///
    ///     // Print friends
    ///     steam_friends.iter().for_each(|f| println!("{}", f));
    ///     
    ///     Ok(())
    ///  }
    /// ```
    ///
    ///  The standard format of "friends since" is the UNIX timestamp, you might want to get a
    ///  more intuitive time format. You could use the `chrono` crate for this:
    /// ```ignore
    ///  let steam_friends = steam_client.get_friends("some-steam-ID")?;
    ///  steam_friends.iter().for_each(|f| {
    ///      println!(
    ///          "me and {} are friends since {}",
    ///          f.steam_id,
    ///          chrono::NaiveDateTime::from_timestamp(f.friend_since, 0)
    ///      )
    ///  });
    /// ```
    pub fn get_friends(&self, steam_id: &str) -> Result<Vec<Friend>, SteamError> {
        let response = self.get_request(
            ENDPOINT_GET_FRIENDLIST,
            vec![("steamid", steam_id), ("relationship", "friend")],
        )?;

        let friends_list = self
            .parse_response::<FriendsResponse, FriendsList>(response)
            .unwrap();
        Ok(friends_list.friends)
    }
}
