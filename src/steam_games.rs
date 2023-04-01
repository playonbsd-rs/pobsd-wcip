use ::std::error;
use pobsd_db::GameDataBase;
use pobsd_parser::Game;
use std::process::Command;

static STEAM_CTL: &str = "/usr/local/bin/steamctl";

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

pub(crate) fn get_steam_games(db: GameDataBase) -> Result<Vec<Game>, Box<dyn error::Error>> {
    let mut game_list: Vec<Game> = vec![];
    let ids = get_steam_ids()?;
    for id in ids {
        if let Some(game) = db.get_game_by_steam_id(id) {
            game_list.push(game.clone());
        }
    }
    game_list.sort();
    Ok(game_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pobsd_parser::{StoreLink, StoreLinks};
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
    fn test_get_ids_from_output() {
        let output = get_ouput();
        let ids = get_ids_from_output(output);
        let ex_ids = vec![
            415650, 418930, 424690, 443030, 443510, 466560, 476580, 551410, 568880, 588460,
        ];
        assert_eq!(ids, ex_ids);
    }
}
