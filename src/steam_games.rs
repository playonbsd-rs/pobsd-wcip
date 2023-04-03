use ::std::error;
use pledge::pledge_promises;
use pobsd_db::GameDataBase;
use pobsd_parser::Game;
use serde::{Deserialize, Serialize};
use std::process::Command;

use crate::config::Config;

static STEAM_CTL: &str = "/usr/local/bin/steamctl";

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

fn get_steamctl_output() -> Result<String, Box<dyn error::Error>> {
    eprintln!("Loading steam data. Please wait.");
    // grap steamctl output
    let output = Command::new(STEAM_CTL).arg("apps").arg("list").output()?;
    pledge_promises![Tty Stdio Rpath Inet Dns]
        .or_else(pledge::Error::ignore_platform)
        .unwrap();
    // convert the output to valid UTF-8
    let output = std::str::from_utf8(&output.stdout)?.to_string();
    Ok(output)
}

fn get_ids_from_output(output: String) -> Vec<usize> {
    let mut ids: Vec<usize> = Vec::new();
    for line in output.lines() {
        // The element of the output are space separated
        // the steam_id being the first element
        let element: Vec<&str> = line.split(' ').collect();
        // I guess it is ok if it fails for some ids
        if let Ok(id) = element[0].to_string().parse() {
            ids.push(id);
        }
    }
    ids
}

fn get_steam_ids_from_steamctl() -> Result<Vec<usize>, Box<dyn error::Error>> {
    let output = get_steamctl_output()?;
    let ids = get_ids_from_output(output);
    Ok(ids)
}

fn get_steam_ids_from_webapi(
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

pub(crate) fn get_steam_games(
    db: GameDataBase,
    config: &Config,
) -> Result<Vec<Game>, Box<dyn error::Error>> {
    let mut game_list: Vec<Game> = vec![];
    let ids = if let Some(steam_id) = &config.steam_id {
        if let Some(steam_key) = &config.steam_key {
            get_steam_ids_from_webapi(steam_id, steam_key)?
        } else {
            get_steam_ids_from_steamctl()?
        }
    } else {
        get_steam_ids_from_steamctl()?
    };
    for id in ids {
        if let Some(game) = db.get_game_by_steam_id(id) {
            game_list.push(game.clone());
        }
    }
    game_list.sort();
    Ok(game_list)
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
