mod app_state;
mod game_details;

use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use libpobsd::{Game, GameDataBase};
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, BorderType, Borders, List, ListItem, Paragraph};
use tui::{Frame, Terminal};

pub(crate) use app_state::{AppState, AppStatus};
pub(crate) use app_state::{InputMode, SearchMode};
pub(crate) use game_details::display_game;

const APP_KEYS_BINDING: &str = r#"
Key bindings
s:    Search mode
TAB:  On search mode, change search mode (name/tag/genre)
ESC:  On search mode, back to list mode
UP:   Previous on the list
DOWN: Next on the list  
k:    On list mode, previous on the list
j:    On list mode, next on the list
q:    On list mode, exit
"#;

pub fn browse(games: Vec<Game>) -> Result<(), std::io::Error> {
    let mut app_state = AppState::new();
    let game_db = GameDataBase::new(games);
    app_state.game_db = game_db;
    enable_raw_mode()?;
    //execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    execute!(std::io::stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal, &mut app_state);

    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    disable_raw_mode()?;

    if let Err(e) = result {
        println!("{}", e);
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    state: &mut AppState,
) -> Result<(), std::io::Error> {
    loop {
        terminal.draw(|f| ui(f, state))?;
        let event = event::read()?;
        if let AppStatus::Close = state.event_handler(event) {
            return Ok(());
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, state: &mut AppState) {
    let parent_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let list_section_block = Block::default()
        .title("Games")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(list_section_block, parent_chunk[0]);
    list_section(f, state, parent_chunk[0]);

    let detail_section_block = Block::default()
        .title("Details")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(detail_section_block, parent_chunk[1]);
    detail_section(f, state, parent_chunk[1]);
}

fn detail_section<B: Backend>(f: &mut Frame<B>, state: &mut AppState, area: Rect) {
    let game = match state.list_state.selected() {
        Some(id) => {
            if state.search_text.is_empty() {
                state.game_db.get_all_games().into_inner().get(id).cloned()
            } else {
                state.search_list().get(id).cloned()
            }
        }
        None => None,
    };

    let new_selection_chunk = Layout::default()
        .horizontal_margin(2)
        .vertical_margin(1)
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(2),
                Constraint::Min(10),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(10),
            ]
            .as_ref(),
        )
        .split(area);
    let title = match &game {
        Some(game) => Paragraph::new(game.name.to_string().to_uppercase()).style(
            Style::default()
                .fg(Color::LightRed)
                .add_modifier(Modifier::BOLD),
        ),
        None => Paragraph::new("Select a game"),
    };
    f.render_widget(title, new_selection_chunk[0]);

    let (desc, genres, tags) = match &game {
        Some(game) => display_game(game),
        None => (Paragraph::new(""), Paragraph::new(""), Paragraph::new("")),
    };
    f.render_widget(desc, new_selection_chunk[1]);
    f.render_widget(genres, new_selection_chunk[2]);
    f.render_widget(tags, new_selection_chunk[3]);

    let key_bindings = Paragraph::new(APP_KEYS_BINDING);
    f.render_widget(key_bindings, new_selection_chunk[4]);
}

fn list_section<B: Backend>(f: &mut Frame<B>, state: &mut AppState, area: Rect) {
    let list_to_show = if state.search_text.is_empty() {
        state.game_db.get_all_games().into_inner()
    } else {
        state.search_list()
    };

    let items: Vec<ListItem> = list_to_show
        .into_iter()
        .map(|item| ListItem::new(Span::from(item.name.clone())))
        .clone()
        .collect();

    let list_chunk = Layout::default()
        .horizontal_margin(2)
        .vertical_margin(1)
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(area);

    let search_title = match &state.mode {
        InputMode::Search(search_mode) => match search_mode {
            SearchMode::Name => " Search by name ",
            SearchMode::Tag => " Search by tag ",
            SearchMode::Genre => " Search by genre ",
        },
        _ => " Search ",
    };
    let search_input = Paragraph::new(state.search_text.to_owned())
        .block(
            Block::default()
                .title(search_title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(match state.mode {
            InputMode::Search(_) => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        });
    f.render_widget(search_input, list_chunk[0]);

    let list = List::new(items)
        .block(Block::default())
        //.highlight_symbol("->")
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::LightMagenta),
        );
    f.render_stateful_widget(list, list_chunk[1], &mut state.list_state)
}
