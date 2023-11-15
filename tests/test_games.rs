use steamr::client::SteamClient;
use steamr::games::{get_game_news, get_owned_games, get_player_stats};

#[test]
fn games_response_is_valid() {
    let test_api_key = std::env::var("IT_API_KEY").expect("IT_API_KEY variable not provided");
    let test_client = SteamClient::new(test_api_key);
    let test_steam_id = std::env::var("IT_STEAM_ID").expect("IT_STEAM_ID variable not provided");

    let test_steam_lib =
        get_owned_games(&test_client, &test_steam_id).unwrap_or_else(|e| panic!("{:?}", e));

    assert!(!test_steam_lib.games.is_empty());
    assert!(!test_steam_lib.games[0].name.is_empty());
}

#[test]
fn game_news_response_is_valid() {
    let test_client = SteamClient::new("no-valid-key-needed-for-this-test".to_string());
    let test_app_id = "10"; // This is CS:GO

    let test_news =
        get_game_news(&test_client, &test_app_id, 5, 300).unwrap_or_else(|e| panic!("{:?}", e));

    assert!(test_news.count >= test_news.game_news.len().try_into().unwrap());
    assert_eq!(test_news.game_news.len(), 5);
}

#[test]
fn user_stats_response_is_valid() {
    let test_api_key = std::env::var("IT_API_KEY").expect("IT_API_KEY variable not provided");
    let test_client = SteamClient::new(test_api_key);
    let test_steam_id = std::env::var("IT_STEAM_ID").expect("IT_STEAM_ID variable not provided");
    let test_app_id = "1086940"; // This is Baldur's Gate 3

    let test_player_stats = get_player_stats(&test_client, &test_steam_id, test_app_id)
        .unwrap_or_else(|e| panic!("{:?}", e));

    assert_eq!(test_player_stats.game_name, "Baldur's Gate 3");
}
