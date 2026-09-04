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
        map[4][7] = Tile::Water;
        map[5][8] = Tile::Water;
        map[6][9] = Tile::Water;
        map[7][9] = Tile::Water;
        map[8][9] = Tile::Water;
        map[8][10] = Tile::Water;
        map[9][11] = Tile::Water;
        map[7][12] = Tile::Mountain;
        map[7][10] = Tile::Mountain;
        map[6][11] = Tile::Mountain;
        map[8][11] = Tile::Mountain;

        AppState { player_x: 5, player_y: 5, width, height, map }
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
        let x = self.player_x as usize;
        let y = self.player_y as usize;
        if self.map[y][x] == Tile::Grass {
            self.map[y][x] = Tile::City;
        }
    }

    fn urban_growth(&self, x: i32, y: i32) -> u8 {
        let mut is_water: bool = false;
        let mut is_city: bool = false;
        let mut is_grass: bool = false;

        for i in y-1..=y+1 {
            for j in x-1..=x+1 {
                if i < 0 || i >= self.height as i32 ||
                    j < 0 || j >= self.width as i32 ||
                        (i == y && j == x) {
                    continue;
                }

                match self.map[i as usize][j as usize] {
                    Tile::City      => is_city = true,
                    Tile::Grass     => is_grass = true,
                    Tile::Water     => is_water = true,
                    Tile::Mountain  => {},
                }
            }
        }
        
        is_city as u8 + is_grass as u8 + is_water as u8
    }

    fn step_cities(&mut self) {
        let mut new_map = self.map.clone();

        for i in 0..self.height as usize {
            for j in 0..self.width as usize {
                let tile = self.map[i][j];
                let score = self.urban_growth(j as i32, i as i32);

                new_map[i][j] = match tile {
                    Tile::Grass if score > 2    => Tile::City,
                    Tile::City if score < 2     => Tile::Grass,
                    other                       => other,
                }
            }
        }

        self.map = new_map;
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
                KeyCode::Char('n')  => state.step_cities(),
                _                   => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
