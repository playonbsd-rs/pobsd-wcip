mod config;
mod database;
mod ls;
mod utils;

#[macro_use]
extern crate lazy_static;

use crate::config::Config;
use crate::database::get_game_db;
use crate::utils::get_steam_games;
use pledge::pledge_promises;
use std::boxed::Box;
use std::error;

mod launcher;
// TUI feature
mod tui;

fn main() -> Result<(), Box<dyn error::Error>> {
    // Set config to default if the config file
    // cannot be read
    let config = match config::get_config() {
        Ok(config) => config,
        Err(_) => {
            eprintln!("Error while reading config file, using default.");
            Config::default()
        }
    };
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
    launcher::run(game_list, &config)?;

    Ok(())
}
