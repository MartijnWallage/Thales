use ratatui::{
    backend::CrosstermBackend,
    widgets::Paragraph,
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use std::io;

struct AppState {
    player_x: u16,
    player_y: u16,
}

impl AppState {
    fn new() -> Self {
        AppState { player_x: 5, player_y: 5 }
    }
}

fn render(state: &AppState) -> Paragraph<'static> {
    let width = 20;
    let height = 20;
    let mut lines = String::new();

    for y in 0..height {
        for x in 0..width {
            if x == state.player_x && y == state.player_y {
                lines.push('@');
            } else {
                lines.push('.');
            }
        }
        lines.push('\n')
    }

    Paragraph::new(lines)
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut std_out = io::stdout();
    execute!(std_out, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(std_out);
    let mut terminal = Terminal::new(backend)?;
    
    let mut state = AppState::new();

    loop {
        terminal.draw(|frame| {
            let paragraph = render(&state);
            frame.render_widget(paragraph, frame.area());
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q')  => break,
                KeyCode::Up         => state.player_y = state.player_y.saturating_sub(1),
                KeyCode::Down       => state.player_y += 1,
                KeyCode::Left       => state.player_x = state.player_x.saturating_sub(1),
                KeyCode::Right      => state.player_x += 1,
                _                   => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
