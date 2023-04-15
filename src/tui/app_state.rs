use crossterm::event::{Event, Event::Key, KeyCode};
use libpobsd::{Game, GameDataBase};
use tui::widgets::ListState;

pub(crate) enum AppStatus {
    Continue,
    Close,
}

#[derive(Debug, PartialEq)]
pub(crate) enum SearchMode {
    Name,
    Tag,
    Genre,
}

#[derive(Debug, PartialEq)]
pub(crate) enum InputMode {
    Normal,
    Search(SearchMode),
}

pub(crate) struct AppState {
    pub(crate) mode: InputMode,
    pub(crate) game_db: GameDataBase,
    pub(crate) list_state: ListState,
    pub(crate) search_text: String,
}

impl AppState {
    pub(crate) fn new() -> AppState {
        AppState {
            mode: InputMode::Normal,
            game_db: GameDataBase::default(),
            list_state: ListState::default(),
            search_text: String::new(),
        }
    }
    pub(crate) fn event_handler(&mut self, event: Event) -> AppStatus {
        if let Key(key) = event {
            match &self.mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return AppStatus::Close,
                    KeyCode::Char('s') => {
                        self.change_mode(InputMode::Search(SearchMode::Name));
                        self.list_state.select(None);
                    }
                    KeyCode::Up | KeyCode::Char('k') => self.move_up(),
                    KeyCode::Down | KeyCode::Char('j') => self.move_down(),
                    _ => {}
                },
                InputMode::Search(search_mode) => match key.code {
                    KeyCode::Esc => {
                        self.change_mode(InputMode::Normal);
                        self.search_text.clear();
                        self.list_state.select(None);
                    }
                    KeyCode::Char(c) => {
                        self.search_text.push(c);
                    }
                    KeyCode::Backspace => {
                        self.search_text.pop();
                    }
                    KeyCode::Tab => match search_mode {
                        SearchMode::Name => self.change_mode(InputMode::Search(SearchMode::Tag)),
                        SearchMode::Tag => self.change_mode(InputMode::Search(SearchMode::Genre)),
                        SearchMode::Genre => self.change_mode(InputMode::Search(SearchMode::Name)),
                    },
                    KeyCode::Up => self.move_up(),
                    KeyCode::Down => self.move_down(),
                    _ => {}
                },
            }
        }
        AppStatus::Continue
    }
    pub(crate) fn change_mode(&mut self, mode: InputMode) {
        self.mode = mode;
    }
    pub(crate) fn search_list(&self) -> Vec<&Game> {
        match &self.mode {
            InputMode::Search(search_mode) => match search_mode {
                SearchMode::Name => self
                    .game_db
                    .search_game_by_name(&self.search_text)
                    .into_inner(),
                SearchMode::Tag => self
                    .game_db
                    .search_game_by_tag(&self.search_text)
                    .into_inner(),
                SearchMode::Genre => self
                    .game_db
                    .search_game_by_genre(&self.search_text)
                    .into_inner(),
            },
            _ => unreachable!(),
        }
    }
    pub(crate) fn move_up(&mut self) {
        let len_list = if self.search_text.is_empty() {
            self.game_db.get_all_games().count
        } else {
            self.search_list().len()
        };
        let selected = match self.list_state.selected() {
            Some(v) => {
                if len_list == 0 {
                    None
                } else if v == 0 {
                    Some(v)
                } else {
                    Some(v - 1)
                }
            }
            None => {
                if len_list == 0 {
                    None
                } else {
                    Some(0)
                }
            }
        };
        self.list_state.select(selected);
    }
    pub(crate) fn move_down(&mut self) {
        let len_list = if self.search_text.is_empty() {
            self.game_db.get_all_games().count
        } else {
            self.search_list().len()
        };
        let selected = match self.list_state.selected() {
            Some(v) => {
                if len_list == 0 {
                    None
                } else if v >= len_list - 1 {
                    Some(len_list - 1)
                } else {
                    Some(v + 1)
                }
            }
            None => {
                if len_list == 0 {
                    None
                } else {
                    Some(0)
                }
            }
        };
        self.list_state.select(selected);
    }
}
#[cfg(test)]
mod test_app_states {
    use super::*;
    use libpobsd::Game;
    fn get_games() -> Vec<Game> {
        let mut games: Vec<Game> = Vec::new();
        let params = vec![
            (1, "AAa", "GenreA", "TagA"),
            (2, "Bbb", "GenreB", "TagB"),
            (3, "Ccc", "GenreC", "TagC"),
        ];
        for (uid, name, genres, tags) in params {
            let mut game = Game::default();
            game.uid = uid;
            game.name = name.to_string();
            game.genres = Some(vec![genres.split(',').map(|a| a.trim()).collect()]);
            game.tags = Some(vec![tags.split(',').map(|a| a.trim()).collect()]);
            games.push(game)
        }
        games
    }
    #[test]
    fn test_app_state_change_mode_method() {
        let mut app_state = AppState::new();
        assert_eq!(app_state.mode, InputMode::Normal);
        app_state.change_mode(InputMode::Search(SearchMode::Name));
        assert_eq!(app_state.mode, InputMode::Search(SearchMode::Name));
    }
    #[test]
    fn test_app_state_search_method_in_name_mode() {
        let games = get_games();
        let mut app_state = AppState::new();
        app_state.change_mode(InputMode::Search(SearchMode::Name));
        app_state.game_db = GameDataBase::new(games.clone());
        app_state.search_text = "Aaa".to_string();
        assert_eq!(app_state.search_list()[0], &games[0]);
        assert_eq!(app_state.search_list().len(), 1);
        app_state.search_text = "does not exist".to_string();
        assert_eq!(app_state.search_list().len(), 0);
    }
    #[test]
    fn test_app_state_search_method_in_tag_mode() {
        let games = get_games();
        let mut app_state = AppState::new();
        app_state.change_mode(InputMode::Search(SearchMode::Tag));
        app_state.game_db = GameDataBase::new(games.clone());
        app_state.search_text = "TagC".to_string();
        assert_eq!(app_state.search_list()[0], &games[2]);
        assert_eq!(app_state.search_list().len(), 1);
        app_state.search_text = "does not exist".to_string();
        assert_eq!(app_state.search_list().len(), 0);
    }
    #[test]
    fn test_app_state_search_method_in_genre_mode() {
        let games = get_games();
        let mut app_state = AppState::new();
        app_state.change_mode(InputMode::Search(SearchMode::Genre));
        app_state.game_db = GameDataBase::new(games.clone());
        app_state.search_text = "GenreB".to_string();
        assert_eq!(app_state.search_list()[0], &games[1]);
        assert_eq!(app_state.search_list().len(), 1);
        app_state.search_text = "does not exist".to_string();
        assert_eq!(app_state.search_list().len(), 0);
    }
}
