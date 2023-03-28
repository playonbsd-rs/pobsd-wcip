use pobsd_db::GameDataBase;
use pobsd_parser::{Parser, ParserResult, Store, StoreLink};
use std::process::Command;

static DB_URL: &'static str = "https://raw.githubusercontent.com/Hukadan/OpenBSD-Games-Database/add-store-links/openbsd-games.db";
static STEAM_CTL: &'static str = "/usr/local/bin/steamctl";

fn get_db() -> Result<String, attohttpc::Error> {
    let content: String = attohttpc::get(DB_URL).send()?.text()?;
    Ok(content)
}

fn get_steam_ids() -> Vec<usize> {
    let output = Command::new(STEAM_CTL)
        .arg("apps")
        .arg("list")
        .output()
        .expect("Failed to execute steamctl command");

    let lines =
        std::str::from_utf8(&output.stdout).expect("Failed to convert steamctl output to string");
    let mut ids: Vec<usize> = Vec::new();
    for line in lines.lines() {
        let element: Vec<&str> = line.split(" ").collect();
        // I guess it is ok if it fails for some ids
        if let Ok(id) = element[0].to_string().parse() {
            ids.push(id)
        }
    }
    ids
}

fn main() -> Result<(), attohttpc::Error> {
    let db = get_db()?;
    let parser = Parser::default();
    let games = match parser.load_from_string(&db) {
        ParserResult::WithError(games, _) => games,
        ParserResult::WithoutError(games) => games,
    };
    let db = GameDataBase::new(games);
    let ids = get_steam_ids();
    for id in ids {
        if let Some(game) = db.get_game_by_steam_id(id) {
            let store = match &game.stores {
                Some(stores) => {
                    let mut store = StoreLink::default();
                    for st in stores.inner_ref() {
                        if st.store.eq(&Store::Steam) {
                            store = st.clone();
                        }
                    }

                    store
                }
                _ => panic!(),
            };
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
                " {}\n Hints: {}\n Install: steamctl depot download -a {} -o <PATH>\n engine: {}\n runtime: {}\n url: {}",
                &game.name, hints, id, engine, runtime, store.url
            );
        }
    }
    println!("------------------------------------");
    Ok(())
}
