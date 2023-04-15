use libpobsd::{GameDataBase, Parser, ParserResult};

static DB_URL: &str =
    "https://raw.githubusercontent.com/playonbsd/OpenBSD-Games-Database/main/openbsd-games.db";

fn get_db() -> Result<String, attohttpc::Error> {
    let content: String = attohttpc::get(DB_URL).send()?.text()?;
    Ok(content)
}

pub fn get_game_db() -> Result<GameDataBase, attohttpc::Error> {
    let db = get_db()?;

    let parser = Parser::default();
    let games = match parser.load_from_string(&db) {
        ParserResult::WithError(games, _) => {
            eprintln!("A parsing error occured while parsing the PlayOnBSD database. Data might be imcomplete.");
            games
        }
        ParserResult::WithoutError(games) => games,
    };
    Ok(GameDataBase::new(games))
}
