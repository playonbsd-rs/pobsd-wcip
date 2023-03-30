use pledge::pledge_promises;
use pobsd_db::GameDataBase;
use pobsd_parser::{Game, Parser, ParserResult, Store, StoreLink};
use std::boxed::Box;
use std::error;
use std::process::Command;

static DB_URL: &str =
    "https://raw.githubusercontent.com/playonbsd/OpenBSD-Games-Database/main/openbsd-games.db";
static STEAM_CTL: &str = "/usr/local/bin/steamctl";

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

fn get_ids_from_output(output: String) -> Vec<usize> {
    let mut ids: Vec<usize> = Vec::new();
    for line in output.lines() {
        // The element of the output are space separated
        // the steam_id being the first element
        let element: Vec<&str> = line.split(' ').collect();
        // I guess it is ok if it fails for some ids
        if let Ok(id) = element[0].to_string().parse() {
            ids.push(id);
        }
    }
    ids
}

fn get_steam_ids() -> Result<Vec<usize>, Box<dyn error::Error>> {
    let output = get_steamctl_output()?;
    let ids = get_ids_from_output(output);
    Ok(ids)
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

fn game_to_sting(game: &Game) -> String {
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
    pledge_promises![Stdio Rpath Inet Dns Exec Proc]
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
        let game_display = game_to_sting(game);
        println!(
            "-----------------------------------------\n{}",
            game_display
        );
    }
    println!("------------------------------------");
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
    fn get_ouput() -> String {
        String::from(
            "\
415650 Unknown App 415650
418930 Unknown App 418930
424690 Steam Controller Configs - Streaming Client
443030 Conan Exiles Dedicated Server
443510 Steam Controller Configs - Steam Button Chord
466560 Northgard
476580 Unknown App 476580
551410 6th Annual Saxxy Awards
568880 Sniper Elite 4 Dedicated Server
588460 Unknown App 588460",
        )
    }

    #[test]
    fn test_game_to_sting() {
        let game = get_test_game();
        let disp = game_to_sting(&game);
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
    #[test]
    fn test_get_ids_from_output() {
        let output = get_ouput();
        let ids = get_ids_from_output(output);
        let ex_ids = vec![
            415650, 418930, 424690, 443030, 443510, 466560, 476580, 551410, 568880, 588460,
        ];
        assert_eq!(ids, ex_ids);
    }
}
