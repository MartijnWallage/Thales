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
use rand::Rng;
use rand::distributions::{Distribution, WeightedIndex};

fn random_tile(rng: &mut impl Rng, dist: &WeightedIndex<u32>) -> Tile {
    match dist.sample(rng) {
        0 => Tile::Grass,
        1 => Tile::Water,
        2 => Tile::Mountain,
        _ => unreachable!(),
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Grass,
    Water,
    Mountain,
    City,
}

#[derive(Clone, Copy, PartialEq)]
enum UnitKind {
    Settler,
}

struct Unit {
    kind: UnitKind,
    x: u16,
    y: u16,
}

struct AppState {
    units: Vec<Unit>,
    selected: Option<usize>,
    width: u16,
    height: u16,
    map: Vec<Vec<Tile>>,
}

impl AppState {
    fn new() -> Self {
        let width = 50;
        let height = 25;
        let mut rng = rand::thread_rng();
        let weights = [50, 10, 15];
        let dist = WeightedIndex::new(weights).unwrap();

        let mut map = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(random_tile(&mut rng, &dist));
            }
            map.push(row);
        }

        AppState {
            units: vec![ Unit { kind: UnitKind::Settler, x: 5, y: 5}],
            selected: Some(0),
            width,
            height,
            map,
        }
    }

    fn move_selected(&mut self, dx: i16, dy: i16) {
        let Some(index) = self.selected else { return };
        let unit = &mut self.units[index];

        let new_x = unit.x as i32 + dx as i32;
        let new_y = unit.y as i32 + dy as i32;
        if new_x >= 0 && new_x < self.width as i32 && new_y >= 0 && new_y < self.height as i32 {
            if self.map[new_y as usize][new_x as usize] != Tile::Water {
               unit.x = new_x as u16;
               unit.y = new_y as u16;
            }
        } 
    }

    fn found_city(&mut self) {
        let Some(index) = self.selected else { return };
        let unit = &mut self.units[index];
        if unit.kind != UnitKind::Settler { return };

        let (x, y) = (unit.x as usize, unit.y as usize);
        if self.map[y][x] == Tile::Grass {
            self.map[y][x] = Tile::City;
            self.units.remove(index);
            self.selected = self.units.len().checked_sub(1);
        }
    }

    fn spawn_settler(&mut self, x: u16, y: u16) {
        let settler = Unit {kind: UnitKind::Settler, x, y};
        self.units.push(settler);
        self.selected = Some(self.units.len() - 1);
    }

    fn neighbors(&self, x: i32, y: i32) -> (u8, u8, u8) {
        let mut water_count: u8 = 0;
        let mut city_count: u8 = 0;
        let mut grass_count: u8 = 0;
       
        for i in y-1..=y+1 {
            for j in x-1..=x+1 {
                if i < 0 || i >= self.height as i32 ||
                    j < 0 || j >= self.width as i32 ||
                        (i == y && j == x) {
                    continue;
                }

                match self.map[i as usize][j as usize] {
                    Tile::City      => city_count += 1,
                    Tile::Grass     => grass_count += 1,
                    Tile::Water     => water_count += 1,
                    Tile::Mountain  => {},
                }
            }
        }
        (city_count, grass_count, water_count)
    }

    fn step_cities(&mut self) {
        let mut new_map = self.map.clone();

        for i in 0..self.height {
            for j in 0..self.width {
                let x = j as usize;
                let y = i as usize;
                let tile = self.map[y][x];

                let (cities, grasses, waters) = self.neighbors(x as i32, y as i32);

                match tile {
                    Tile::Grass if cities > 0 && waters > 0 && grasses > 0 => new_map[y][x] = Tile::City,
                    Tile::City if waters < 0 => new_map[y][x] = Tile::Grass,
                    Tile::City if cities > 4 && grasses > 1 => {
                        new_map[y][x] = Tile::Grass;
                        self.spawn_settler(x as u16, y as u16);
                    },
                    _ => {},
                }
            }
        }

        self.map = new_map;
    }
}

fn render(state: &AppState) -> Paragraph<'static> {
    let mut lines = String::new();

    for i in 0..state.height {
        for j in 0..state.width {
            if let Some(unit) = state.units.iter().find(|u| u.x == j && u.y == i) {
                lines.push(match unit.kind {
                    UnitKind::Settler => '@'
                });
            } else {
                let tile = state.map[i as usize][j as usize];
                let ch = match tile {
                    Tile::Grass     => '.',
                    Tile::Mountain  => '▲',
                    Tile::Water     => '~',
                    Tile::City      => '□',
                };
                lines.push(ch);
            }
        }
        lines.push('\n');
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
                KeyCode::Tab        => {
                    if !state.units.is_empty() {
                        let next = state.selected.map_or(0, |i| (i + 1) % state.units.len() );
                        state.selected = Some(next);
                    }
                }
                KeyCode::Char('b')  => state.found_city(),
                KeyCode::Up         => state.move_selected(0, -1),
                KeyCode::Down       => state.move_selected(0, 1),
                KeyCode::Left       => state.move_selected(-1, 0),
                KeyCode::Right      => state.move_selected(1, 0),
                KeyCode::Char('n')  => state.step_cities(),
                _                   => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
