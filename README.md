# steamr 🦀

[![crates.io](https://img.shields.io/crates/v/steamr.svg)](https://crates.io/crates/steamr)
[![Documentation](https://docs.rs/steamr/badge.svg)](https://docs.rs/steamr)
[![Apache-2 licensed](https://img.shields.io/crates/l/steamr.svg)](./LICENSE)
[![CI](https://github.com/bellackn/steamr/workflows/CI/badge.svg)](https://github.com/bellackn/steamr/actions?query=workflow%3ACI)

steamr is a simple Rust library to help you to interact with Valve's [Steam API](https://developer.valvesoftware.com/wiki/Steam_Web_API).
It uses the [reqwest](https://github.com/seanmonstar/reqwest) crate under the hood.

> This is a project to gather more hands-on Rust experience. Not all endpoints
> are implemented and you might find awkward code in here. However, things that
> _are_ implemented should be quite stable.

## Implemented Endpoints

- [GetFriendList](https://developer.valvesoftware.com/wiki/Steam_Web_API#GetFriendList_.28v0001.29)
- [GetOwnedGames](https://developer.valvesoftware.com/wiki/Steam_Web_API#GetOwnedGames_.28v0001.29)
- [GetNewsForApp](https://developer.valvesoftware.com/wiki/Steam_Web_API#GetNewsForApp_.28v0002.29)
- [GetUserStatsForGame](https://developer.valvesoftware.com/wiki/Steam_Web_API#GetUserStatsForGame_.28v0002.29)

Retrieving data from endpoints works via calling the respective [`SteamClient`](https://docs.rs/steamr/latest/steamr/client/struct.SteamClient.html) method. Check the examples in this README to see how it works.

## Requirements

You need a valid API key to make full use of this library. Visit <https://steamcommunity.com/dev/apikey> to obtain yours.

## Example

```rust,no_run
use steamr::client::SteamClient;
use steamr::errors::SteamError;

fn main() -> Result<(), SteamError> {
    // Create a new client that will be used to communicate with Steam's API.
    let api_key = String::from("your-api-key");
    let steam_client = SteamClient::from(api_key);

    // Get a list of all games from the user with the provided Steam ID (given that they are publicly visible)
    let steam_id = String::from("some-steam-id");
    let steam_lib = steam_client.get_library(&steam_id)?;

    // Print out the games that were played for more than an hour.
    steam_lib.games.iter()
        .filter(|g| g.playtime_forever > 60)
        .for_each(|g| println!("{}", g.name));

    Ok(())
}
```

There are some endpoints that don't require an API key. If you only need those,
you can use `SteamClient` like so:

```rust,no_run
use steamr::client::SteamClient;
use steamr::errors::SteamError;

fn main() -> Result<(), SteamError> {
    // Create a new SteamClient without an API key
    let steam_client = SteamClient::new();

    // Get news for a game
    let app_id = "10";
    let news = steam_client.get_game_news(app_id, 5, 100)?;
    news.game_news.iter()
        .for_each(|n| println!("The article '{}' was written by '{}'", n.title, n.author));

    Ok(())
}
```
