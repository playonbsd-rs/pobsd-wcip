use crate::config::Config;
use crate::ls::display_game_list;
use crate::tui::browse;
use clap::Command;
use libpobsd::Game;

fn cli() -> Command {
    Command::new("wcip")
        .about("A small utility that list the games you own on Steam and can be played on OpenBSD")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("ls").about("List owned games running on OpenBSD"))
        .subcommand(Command::new("tui").about("Browse owned games running on OpenBSD"))
}

pub fn run(game_list: Vec<Game>, config: &Config) -> Result<(), attohttpc::Error> {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("ls", _)) => display_game_list(game_list, config),
        Some(("tui", _)) => browse(game_list)?,
        _ => println!("Unsupported command"),
    }
    Ok(())
}
