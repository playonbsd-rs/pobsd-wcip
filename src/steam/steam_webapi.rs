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

#[cfg(test)]
mod tests {
    use super::*;

    fn get_ouput() -> String {
        String::from(
            "\
415650 Unknown App 415650
418930 Unknown App 418930
424690 Steam Controller Configs - Streaming Client
443030 Conan Exiles Dedicated Server
443510 Steam Controller Configs - Steam Button Chord
466560 Northgard
476580 Unknown App 476580
551410 6th Annual Saxxy Awards
568880 Sniper Elite 4 Dedicated Server
588460 Unknown App 588460",
        )
    }

    #[test]
    fn test_get_ids_from_output() {
        let output = get_ouput();
        let ids = get_ids_from_output(output);
        let ex_ids = vec![
            415650, 418930, 424690, 443030, 443510, 466560, 476580, 551410, 568880, 588460,
        ];
        assert_eq!(ids, ex_ids);
    }
}
