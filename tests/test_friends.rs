use steamr::client::SteamClient;

#[test]
fn valid_games_response() {
    let test_api_key = std::env::var("IT_API_KEY").expect("IT_API_KEY variable not provided");
    let test_client = SteamClient::from(test_api_key);
    let test_steam_id = std::env::var("IT_STEAM_ID").expect("IT_STEAM_ID variable not provided");

    let test_friends = test_client
        .get_friends(&test_steam_id)
        .unwrap_or_else(|e| panic!("{:?}", e));

    assert!(!test_friends.is_empty());
    assert!(!test_friends[0].steam_id.is_empty());
}
