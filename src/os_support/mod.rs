mod lookup;

use self::lookup::GAMETOOS;

pub fn get_preferable_os(game_id: u32) -> String {
    if let Some(oss) = GAMETOOS.get(&game_id) {
        if oss.contains(&"linux") {
            return String::from("linux");
        } else if oss.contains(&"win") {
            return String::from("win");
        }
    }
    String::from("any")
}
