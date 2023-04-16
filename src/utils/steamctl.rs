use ::std::error;
use std::path::PathBuf;
use std::process::Command;

static STEAM_CTL: &str = "/usr/local/bin/steamctl";

#[allow(dead_code)]
pub(crate) fn download_steam_game(
    password: &Option<String>,
    path: &PathBuf,
    steam_id: &str,
) -> Result<String, Box<dyn error::Error>> {
    // grab steamctl output
    let output = match password {
        Some(password) => Command::new(STEAM_CTL)
            .arg("depot")
            .arg("dowload")
            .arg(format!("-a {}", steam_id))
            .arg(format!("-o {}", path.to_string_lossy()))
            .arg(format!("-p {}", password))
            .output()?,
        // Should prompt for steam password in that case
        // or better hope the bug in steamctl is quickly
        // fixed.
        // To deal with this case later.
        None => Command::new(STEAM_CTL).arg("apps").arg("list").output()?,
    };
    // convert the output to valid UTF-8
    let output = std::str::from_utf8(&output.stdout)?.to_string();
    Ok(output)
}

fn get_steamctl_output(password: &Option<String>) -> Result<String, Box<dyn error::Error>> {
    eprintln!("Loading steam data. Please wait.");
    // grab steamctl output
    let output = match password {
        // At least for now, password is not available for the apps list subcommand
        // So the command is the same in both cases.
        Some(_password) => Command::new(STEAM_CTL).arg("apps").arg("list").output()?,
        None => Command::new(STEAM_CTL).arg("apps").arg("list").output()?,
    };
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

pub fn get_steam_ids_from_steamctl(
    password: &Option<String>,
) -> Result<Vec<usize>, Box<dyn error::Error>> {
    let output = get_steamctl_output(password)?;
    let ids = get_ids_from_output(output);
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;

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
