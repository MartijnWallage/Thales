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

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Grass,
    Water,
    Mountain,
}

struct AppState {
    player_x: u16,
    player_y: u16,
    width: u16,
    height: u16,
    map: Vec<Vec<Tile>>,
}

impl AppState {
    fn new() -> Self {
        let width = 20;
        let height = 10;
        let mut map = vec![vec![Tile::Grass; width as usize]; height as usize];

        map[3][5] = Tile::Water;
        map[3][6] = Tile::Water;
        map[2][5] = Tile::Water;
        map[2][6] = Tile::Water;
        map[3][5] = Tile::Water;
        map[3][6] = Tile::Water;
        map[7][12] = Tile::Mountain;
        map[7][11] = Tile::Mountain;
        map[6][11] = Tile::Mountain;

        AppState { player_x: 5, player_y: 5, width, height, map }
    }
}

fn render(state: &AppState) -> Paragraph<'static> {
    let mut lines = String::new();

    for y in 0..state.height {
        for x in 0..state.width {
            if x == state.player_x && y == state.player_y {
                lines.push('@');
            } else {
                let tile = state.map[y as usize][x as usize];
                let ch = match tile {
                    Tile::Grass     => '.',
                    Tile::Mountain  => '▲',
                    Tile::Water     => '~',
                };
                lines.push(ch);
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
                KeyCode::Down       => state.player_y = (state.player_y + 1).min(state.height - 1),
                KeyCode::Left       => state.player_x = state.player_x.saturating_sub(1),
                KeyCode::Right      => state.player_x = (state.player_x + 1).min(state.width - 1),
                _                   => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
