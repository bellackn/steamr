# steamr 🦀

[![crates.io](https://img.shields.io/crates/v/steamr.svg)](https://crates.io/crates/steamr)
[![Documentation](https://docs.rs/steamr/badge.svg)](https://docs.rs/steamr)
[![Apache-2 licensed](https://img.shields.io/crates/l/steamr.svg)](./LICENSE)
[![CI](https://github.com/bellackn/steamr/workflows/CI/badge.svg)](https://github.com/bellackn/steamr/actions?query=workflow%3ACI)

steamr is a simple Rust library to help you to interact with Valve's [Steam API](https://developer.valvesoftware.com/wiki/Steam_Web_API).
It uses the [reqwest](https://github.com/seanmonstar/reqwest) crate under the hood.

## Requirements

You need a valid API key to use this library. Visit <https://steamcommunity.com/dev/apikey> to obtain yours.

## Example

```rust,no_run
use steamr::client::SteamClient;
use steamr::errors::SteamError;
use steamr::games::get_owned_games;

fn main() -> Result<(), SteamError> {
    // Create a new client that will be used to communicate with Steam's API. 
    let api_key = String::from("your-api-key");
    let steam_client = SteamClient::new(api_key);
    
    // Get a list of all games from the user with the provided Steam ID (given that they are publicly visible)
    let steam_id = "some-steam-id";
    let steam_lib = get_owned_games(&steam_client, &steam_id)?;
    
    // Print out the games that were played for more than an hour.
    steam_lib.games.iter()
        .filter(|g| g.playtime_forever > 60)
        .for_each(|g| println!("{}", g.name));
    
    Ok(())
}
```
