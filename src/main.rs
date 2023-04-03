mod config;
mod database;
mod ls;
mod os_support;
mod steam_games;

#[macro_use]
extern crate lazy_static;

use crate::database::get_game_db;
use crate::steam_games::get_steam_games;
use pledge::pledge_promises;
use std::boxed::Box;
use std::error;

// Without TUI feature
#[cfg(not(feature = "tui"))]
use crate::ls::display_game_list;

// TUI feature
#[cfg(feature = "tui")]
mod tui;
#[cfg(feature = "tui")]
mod tui_launcher;

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = config::get_config();
    pledge_promises![Tty Stdio Rpath Inet Dns Exec Proc]
        .or_else(pledge::Error::ignore_platform)
        .unwrap();

    let db = get_game_db()?;
    // Get list of games owned and running on OpenBSD
    // When called, drop Exec and Proc pledge promises
    let game_list = get_steam_games(db, &config)?;

    pledge_promises![Tty Stdio]
        .or_else(pledge::Error::ignore_platform)
        .unwrap();

    // TUI feature
    #[cfg(feature = "tui")]
    tui_launcher::run(game_list)?;

    // Without TUI feature
    #[cfg(not(feature = "tui"))]
    display_game_list(game_list);

    Ok(())
}
