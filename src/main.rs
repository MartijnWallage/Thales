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
    City,
}

struct City {
    x: u16,
    y: u16,
    name: String,
}

struct AppState {
    player_x: u16,
    player_y: u16,
    width: u16,
    height: u16,
    map: Vec<Vec<Tile>>,
    cities: Vec<City>,
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

        AppState { player_x: 5, player_y: 5, width, height, map, cities: Vec::new() }
    }

    fn player_move(&mut self, dx: i16, dy: i16) {
        let new_x = self.player_x as i32 + dx as i32;
        let new_y = self.player_y as i32 + dy as i32;
        if new_x >= 0 && new_x < self.width as i32 && new_y >= 0 && new_y < self.height as i32 {
            if self.map[new_y as usize][new_x as usize] != Tile::Water {
               self.player_x = new_x as u16;
               self.player_y = new_y as u16;
            }
        } 
    }

    fn found_city(&mut self) {
        let tile = self.map[self.player_y as usize][self.player_x as usize];
        if tile != Tile::Grass {
            return
        }

        self.map[self.player_y as usize][self.player_x as usize] = Tile::City;
        self.cities.push(City {
                x: self.player_x,
                y: self.player_y,
                name: String::from("Rome"),
        });
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
                    Tile::City      => '□',
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
                KeyCode::Char('b')  => state.found_city(),
                KeyCode::Up         => state.player_move(0, -1),
                KeyCode::Down       => state.player_move(0, 1),
                KeyCode::Left       => state.player_move(-1, 0),
                KeyCode::Right      => state.player_move(1, 0),
                _                   => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
