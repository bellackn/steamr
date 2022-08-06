# steamr 🦀

steamr is a simple Rust library to help you to interact with Valve's [Steam API](https://developer.valvesoftware.com/wiki/Steam_Web_API).
It uses the [reqwest](https://github.com/seanmonstar/reqwest) crate under the hood.

## Example

```rust
fn main() -> Result<(), SteamError> {
    let steam_client = SteamClient::new("an-API-key");
    let steam_lib = get_owned_games(&steam_client, "some-steam-ID")?;
    
    // Print out games that were played for more than an hour.
    steam_lib.games.iter()
        .filter(|g| g.playtime_forever > 60)
        .for_each(|g| println!("{}", g.name));
    
    Ok(())
}
```
