use ::std::error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct OwnedGame {
    appid: usize,
    playtime_forever: usize,
    playtime_windows_forever: usize,
    playtime_mac_forever: usize,
    playtime_linux_forever: usize,
    rtime_last_played: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct OwnedGames {
    game_count: usize,
    games: Vec<OwnedGame>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct OwnedGamesResponse {
    response: OwnedGames,
}

fn get_steam_api_url(steam_id: &str, steam_key: &str) -> String {
    format!("https://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/?key={}&steamid={}&format=json", steam_key, steam_id)
}

fn get_owned_games(steam_id: &str, steam_key: &str) -> Result<OwnedGames, Box<dyn error::Error>> {
    eprintln!("Loading steam data. Please wait.");
    let url = get_steam_api_url(steam_id, steam_key);
    let content: String = attohttpc::get(url).send()?.text()?;
    let res: OwnedGamesResponse = serde_json::from_str(&content)?;
    let res = res.response;
    Ok(res)
}

pub(super) fn get_steam_ids_from_webapi(
    steam_id: &str,
    steam_key: &str,
) -> Result<Vec<usize>, Box<dyn error::Error>> {
    let owned_games = get_owned_games(steam_id, steam_key)?;
    let mut ids: Vec<usize> = vec![];
    for game in owned_games.games {
        ids.push(game.appid);
    }
    Ok(ids)
}
