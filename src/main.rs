mod steam_games;
#[cfg(feature = "tui")]
mod tui;

use crate::steam_games::get_steam_games;
#[cfg(feature = "tui")]
use crate::tui::browse;
use pledge::pledge_promises;
use pobsd_db::GameDataBase;
use pobsd_parser::{Game, Parser, ParserResult, Store, StoreLink};
use std::boxed::Box;
use std::error;

static DB_URL: &str =
    "https://raw.githubusercontent.com/playonbsd/OpenBSD-Games-Database/main/openbsd-games.db";

fn get_db() -> Result<String, attohttpc::Error> {
    let content: String = attohttpc::get(DB_URL).send()?.text()?;
    Ok(content)
}

fn get_steam_store(game: &Game) -> &StoreLink {
    match &game.stores {
        Some(stores) => {
            // the get_game_by_steam_id uses store entries
            // so there is for sure one steam store link
            let store: &StoreLink = stores
                .inner_ref()
                .iter()
                .filter(|a| a.store.eq(&Store::Steam))
                .collect::<Vec<&StoreLink>>()
                .first()
                .cloned()
                .unwrap();
            store
        }
        // the get_game_by_steam_id uses stores entry
        // therefore every game returned by the method
        // should have at least of store link
        _ => unreachable!(),
    }
}
// Displaying the games
fn display_game_list(game_list: Vec<Game>) {
    for game in game_list {
        let game_display = game_to_string(&game);
        println!(
            "-----------------------------------------\n{}",
            game_display
        );
    }
    println!("------------------------------------");
}

fn game_to_string(game: &Game) -> String {
    let store = get_steam_store(game);
    let id = store.id.unwrap();
    let mut to_display: Vec<String> = Vec::new();
    to_display.push(game.name.to_string());
    to_display.push(
        format!(
            "Install: steamctl depot download -a {} -o <PATH> -os linux64 (if available, windows otherwise)", 
            id
        )
    );
    match &game.hints {
        Some(hints) => to_display.push(format!("hint: {}", hints)),
        None => to_display.push(String::from("hint: None")),
    };
    match &game.engine {
        Some(engine) => to_display.push(format!("engine: {}", engine)),
        None => to_display.push(String::from("engine: N/A")),
    };
    match &game.runtime {
        Some(runtime) => to_display.push(format!("runtime: {}", runtime)),
        None => to_display.push(String::from("runtime: N/A")),
    };
    to_display.push(format!("url: {}", store.url));
    to_display.join("\n")
}

fn main() -> Result<(), Box<dyn error::Error>> {
    pledge_promises![Tty Stdio Rpath Inet Dns Exec Proc]
        .or_else(pledge::Error::ignore_platform)
        .unwrap();
    /*
    pledge_promises![Tty Stdio Rpath Inet Dns]
        .or_else(pledge::Error::ignore_platform)
        .unwrap();
    */
    let db = get_db()?;
    /*
    pledge_promises![Tty Stdio]
        .or_else(pledge::Error::ignore_platform)
        .unwrap();
    */
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

    #[cfg(feature = "tui")]
    browse(game_list)?;
    #[cfg(not(feature = "tui"))]
    display_game_list(game_list);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pobsd_parser::StoreLinks;
    fn get_test_game() -> Game {
        let mut game = Game::default();
        game.name = String::from("Super Game");
        game.hints = Some(String::from("Some hints"));
        game.engine = Some(String::from("Engine1"));
        game.runtime = Some(String::from("Runtime1"));
        let sl =
            StoreLink::from("https://store.steampowered.com/app/1965800/Dice_Tribes_Ambitions/");
        let mut sls = StoreLinks::default();
        sls.push(sl);
        game.stores = Some(sls);
        game
    }
    #[test]
    fn test_game_to_string() {
        let game = get_test_game();
        let disp = game_to_string(&game);
        let exp = "\
        Super Game\n\
        Install: steamctl depot download -a 1965800 -o <PATH> -os linux64 (if available, windows otherwise)\n\
        hint: Some hints\n\
        engine: Engine1\n\
        runtime: Runtime1\n\
        url: https://store.steampowered.com/app/1965800/Dice_Tribes_Ambitions/";
        assert_eq!(disp, exp);
    }
    #[test]
    fn test_get_steam_store() {
        let game = get_test_game();
        let sl = get_steam_store(&game);
        assert_eq!(
            sl,
            &StoreLink::from("https://store.steampowered.com/app/1965800/Dice_Tribes_Ambitions/")
        )
    }
}
