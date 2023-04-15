mod os_lookup_table;
mod steam_webapi;
mod steamctl;

use self::steam_webapi::get_steam_ids_from_webapi;
use self::steamctl::get_steam_ids_from_steamctl;
use crate::config::Config;

use ::std::error;
use libpobsd::{Game, GameDataBase};

use self::os_lookup_table::GAMETOOS;

pub fn get_preferable_os(game_id: u32) -> String {
    match GAMETOOS.get(&game_id) {
        Some(oss) => {
            if oss.contains(&"linux") {
                String::from("linux")
            } else if oss.contains(&"win") {
                String::from("win")
            } else {
                String::from("any")
            }
        }
        None => String::from("any"),
    }
}

pub(crate) fn get_steam_games(
    db: GameDataBase,
    config: &Config,
) -> Result<Vec<Game>, Box<dyn error::Error>> {
    let mut game_list: Vec<Game> = vec![];
    let ids = match &config.steam_id {
        // Use the webapi if both steam_id and steam_key
        // are set
        Some(steam_id) => match &config.steam_key {
            Some(steam_key) => get_steam_ids_from_webapi(steam_id, steam_key)?,
            None => get_steam_ids_from_steamctl()?,
        },
        None => get_steam_ids_from_steamctl()?,
    };
    for id in ids {
        if let Some(game) = db.get_game_by_steam_id(id) {
            game_list.push(game.clone());
        }
    }
    game_list.sort();
    Ok(game_list)
}
