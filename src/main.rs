mod ls;
mod steam_games;
#[cfg(feature = "tui")]
mod tui;

use crate::ls::display_game_list;
use crate::steam_games::get_steam_games;
#[cfg(feature = "tui")]
use crate::tui::browse;
use pledge::pledge_promises;
use pobsd_db::GameDataBase;
use pobsd_parser::{Parser, ParserResult};
use std::boxed::Box;
use std::error;

static DB_URL: &str =
    "https://raw.githubusercontent.com/playonbsd/OpenBSD-Games-Database/main/openbsd-games.db";

fn get_db() -> Result<String, attohttpc::Error> {
    let content: String = attohttpc::get(DB_URL).send()?.text()?;
    Ok(content)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    pledge_promises![Tty Stdio Rpath Inet Dns Exec Proc]
        .or_else(pledge::Error::ignore_platform)
        .unwrap();

    let db = get_db()?;
    let parser = Parser::default();
    let games = match parser.load_from_string(&db) {
        ParserResult::WithError(games, _) => {
            eprintln!("A parsing error occured while parsing the PlayOnBSD database. Data might be imcomplete.");
            games
        }
        ParserResult::WithoutError(games) => games,
    };
    let db = GameDataBase::new(games);
    let game_list = get_steam_games(db)?;

    // Make a list of owned games running on OpenBSD
    pledge_promises![Tty Stdio]
        .or_else(pledge::Error::ignore_platform)
        .unwrap();

    #[cfg(feature = "tui")]
    browse(game_list)?;
    #[cfg(not(feature = "tui"))]
    display_game_list(game_list);
    Ok(())
}
