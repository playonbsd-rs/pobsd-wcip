use tui::style::{Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Paragraph};

use libpobsd::Game;

pub fn display_game(game: &Game) -> (Paragraph, Paragraph, Paragraph) {
    let mut res: Vec<Spans> = Vec::new();
    let mut tags: Vec<Spans> = Vec::new();
    let mut genres: Vec<Spans> = Vec::new();

    if let Some(engine) = &game.engine {
        res.push(format_single("Engine".to_string(), engine.to_string()));
    }
    if let Some(setup) = &game.setup {
        res.push(format_single("Setup".to_string(), setup.to_string()));
    }
    if let Some(runtime) = &game.runtime {
        res.push(format_single("Runtime".to_string(), runtime.to_string()));
    }
    if let Some(stores) = &game.stores {
        let vecspans = format_mutiple((
            "Stores".to_string(),
            stores
                .clone()
                .into_inner()
                .into_iter()
                .map(|a| a.url.clone())
                .collect(),
        ));
        for spans in vecspans {
            res.push(spans);
        }
    }
    if let Some(hints) = &game.hints {
        res.push(format_single("Hints".to_string(), hints.to_string()));
    }
    if let Some(item) = &game.genres {
        genres.push(format_tags(item.clone()));
    }
    if let Some(item) = &game.tags {
        tags.push(format_tags(item.clone()));
    }
    if let Some(year) = &game.year {
        res.push(format_single("Year".to_string(), year.to_string()));
    }
    if let Some(devs) = &game.devs {
        res.push(format_single("Developer".to_string(), devs.join(", ")));
    }
    if let Some(publis) = &game.publis {
        res.push(format_single("Publisher".to_string(), publis.join(", ")));
    }
    if let Some(version) = &game.version {
        res.push(format_single("Version".to_string(), version.to_string()));
    }
    if let Some(status) = &game.status {
        res.push(format_single("Status".to_string(), status.to_string()));
    }
    if let Some(added) = &game.added {
        res.push(format_single("Added".to_string(), added.to_string()));
    }
    if let Some(updated) = &game.updated {
        res.push(format_single(
            "Last updated".to_string(),
            updated.to_string(),
        ));
    };
    let res = Paragraph::new(res);
    let genres = Paragraph::new(genres).block(
        Block::default()
            .title("GENRES")
            .borders(Borders::ALL)
            .border_type(BorderType::Thick),
    );
    let tags = Paragraph::new(tags).block(
        Block::default()
            .title("TAGS")
            .borders(Borders::ALL)
            .border_type(BorderType::Thick),
    );
    (res, genres, tags)
}

fn format_single<'a>(name: String, data: String) -> Spans<'a> {
    Spans::from(vec![
        Span::styled(name, Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": "),
        Span::raw(data),
    ])
}

fn format_mutiple<'a>(data: (String, Vec<String>)) -> Vec<Spans<'a>> {
    let mut text = vec![Spans::from(vec![
        Span::styled(data.0, Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(": "),
    ])];
    for store in data.1 {
        text.push(Spans::from(vec![
            Span::raw("- "),
            Span::styled(
                store.to_string(),
                Style::default().add_modifier(Modifier::UNDERLINED),
            ),
        ]))
    }
    text
}

fn format_tags<'a>(data: Vec<String>) -> Spans<'a> {
    let mut tags: Vec<Span> = vec![];
    for store in data {
        tags.push(Span::styled(
            store.to_string(),
            Style::default().add_modifier(Modifier::UNDERLINED),
        ));
        tags.push(Span::raw(" "));
    }
    Spans::from(tags)
}
