use steamr::games::{get_game_news, get_owned_games};
use steamr::SteamClient;

#[test]
fn games_response_is_valid() {
    let test_api_key = std::env::var("IT_API_KEY").expect("IT_API_KEY variable not provided");
    let test_client = SteamClient::new(test_api_key);
    let test_steam_id = std::env::var("IT_STEAM_ID").expect("IT_STEAM_ID variable not provided");

    let test_steam_lib =
        get_owned_games(&test_client, &test_steam_id).unwrap_or_else(|e| panic!("{:?}", e));

    assert!(test_steam_lib.games.len() > 0);
    assert!(test_steam_lib.games[0].name.len() > 0);
}

#[test]
fn game_news_response_is_valid() {
    let test_client = SteamClient::new("no-valid-key-needed-for-this-test".to_string());
    let test_game_id = String::from("10");

    let test_news =
        get_game_news(&test_client, test_game_id, 5, 300).unwrap_or_else(|e| panic!("{:?}", e));

    assert!(test_news.count >= test_news.game_news.len().try_into().unwrap());
    assert_eq!(test_news.game_news.len(), 5);
}
