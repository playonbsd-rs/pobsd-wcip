use pledge::pledge_promises;
use pobsd_db::GameDataBase;
use pobsd_parser::{Game, Parser, ParserResult, Store, StoreLink};
use std::boxed::Box;
use std::error;
use std::process::Command;

static DB_URL: &'static str =
    "https://raw.githubusercontent.com/playonbsd/OpenBSD-Games-Database/main/openbsd-games.db";
static STEAM_CTL: &'static str = "/usr/local/bin/steamctl";

fn get_db() -> Result<String, attohttpc::Error> {
    let content: String = attohttpc::get(DB_URL).send()?.text()?;
    Ok(content)
}

fn get_steamctl_output() -> Result<String, Box<dyn error::Error>> {
    eprintln!("Loading steam data. Please wait.");
    // grap steamctl output
    let output = Command::new(STEAM_CTL).arg("apps").arg("list").output()?;
    // convert the output to valid UTF-8
    let output = std::str::from_utf8(&output.stdout)?.to_string();
    Ok(output)
}

fn get_steam_ids() -> Result<Vec<usize>, Box<dyn error::Error>> {
    let mut ids: Vec<usize> = Vec::new();
    let output = get_steamctl_output()?;
    for line in output.lines() {
        // The element of the output are space separated
        // the steam_id being the first element
        let element: Vec<&str> = line.split(" ").collect();
        // I guess it is ok if it fails for some ids
        if let Ok(id) = element[0].to_string().parse() {
            ids.push(id);
        }
    }
    Ok(ids)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    pledge_promises![Stdio Rpath Inet Dns Exec Proc Unveil]
        .or_else(pledge::Error::ignore_platform)
        .unwrap();
    let ids = get_steam_ids()?;
    pledge_promises![Stdio Rpath Inet Dns]
        .or_else(pledge::Error::ignore_platform)
        .unwrap();
    let db = get_db()?;
    pledge_promises![Stdio]
        .or_else(pledge::Error::ignore_platform)
        .unwrap();
    let parser = Parser::default();
    let games = match parser.load_from_string(&db) {
        ParserResult::WithError(games, _) => {
            eprintln!("A parsing error occured while parsing the PlayOnBSD database. Data might be imcomplete.");
            games
        }
        ParserResult::WithoutError(games) => games,
    };
    let db = GameDataBase::new(games);
    let mut game_list: Vec<&Game> = vec![];
    // Make a list of owned games running on OpenBSD
    for id in ids {
        if let Some(game) = db.get_game_by_steam_id(id) {
            game_list.push(game);
        }
    }
    game_list.sort();
    // Displaying the games
    for game in game_list {
        let store = match &game.stores {
            Some(stores) => {
                let store: Vec<&StoreLink> = stores
                    .inner_ref()
                    .iter()
                    .filter(|a| a.store.eq(&Store::Steam))
                    .collect();
                // the get_game_by_steam_id uses store entries
                // so there is for sure one steam store link
                store
                    .get(0)
                    .expect("Expected one Steam store link, found none")
                    .clone()
            }
            // the get_game_by_steam_id uses stores entry
            // therefore every game returned by the method
            // should have at least of store link
            _ => unreachable!(),
        };
        // This is game was chosen according to its id
        // and therefore has one
        let id = store.id.unwrap();
        let hints = match &game.hints {
            Some(hints) => hints.into(),
            None => String::from("None"),
        };
        let engine = match &game.engine {
            Some(engine) => engine.into(),
            None => String::from("N/A"),
        };
        let runtime = match &game.runtime {
            Some(runtime) => runtime.into(),
            None => String::from("N/A"),
        };
        println!("------------------------------------");
        println!(
                " {}\n Hints: {}\n Install: steamctl depot download -a {} -o <PATH> -os linux64 (if available, windows otherwise)\n engine: {}\n runtime: {}\n url: {}",
                &game.name, hints, id, engine, runtime, store.url
            );
    }
    println!("------------------------------------");
    Ok(())
}
