use pobsd_db::GameDataBase;
use pobsd_parser::{Parser, ParserResult, Store, StoreLink};
use std::process::Command;

fn main() {
    let parser = Parser::default();
    let games = match parser
        .load_from_file("openbsd-games.db")
        .expect("No db found")
    {
        ParserResult::WithError(games, _) => games,
        ParserResult::WithoutError(games) => games,
    };
    let db = GameDataBase::new(games);
    println!("Loading Steam data, please wait.");
    let output = Command::new("steamctl")
        .arg("apps")
        .arg("list")
        .output()
        .unwrap();

    let lines = std::str::from_utf8(&output.stdout).expect("No way");
    let mut ids: Vec<usize> = Vec::new();
    for line in lines.lines() {
        let element: Vec<&str> = line.split(" ").collect();
        ids.push(element[0].to_string().parse().expect("What?"));
    }
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
            println!("{}: {}", &game.name, store.url);
        }
    }
}
