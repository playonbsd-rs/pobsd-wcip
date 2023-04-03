mod os_lookup_table;
mod steam_webapi;
mod steamctl;

use self::steam_webapi::get_steam_ids_from_webapi;
use self::steamctl::get_steam_ids_from_steamctl;
use crate::config::Config;

use ::std::error;
use pobsd_db::GameDataBase;
use pobsd_parser::Game;

use self::os_lookup_table::GAMETOOS;

pub fn get_preferable_os(game_id: u32) -> String {
    if let Some(oss) = GAMETOOS.get(&game_id) {
        if oss.contains(&"linux") {
            return String::from("linux");
        } else if oss.contains(&"win") {
            return String::from("win");
        }
    }
    String::from("any")
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
