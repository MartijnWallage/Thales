use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};

use rand::Rng;
use rand::distributions::{Distribution, WeightedIndex};

use ratatui::{
    backend::CrosstermBackend,
    widgets::Paragraph,
    Terminal,
};


// -----------------------------------------------
// Tile
// -----------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Grass,
    Water,
    Mountain,
    City,
}


// -------------------------------------------------------------------------
// Unit
// -------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum UnitKind {
    Settler,
}

struct Unit {
    kind: UnitKind,
    x: u16,
    y: u16,
}


//--------------------------------------------------------------------------
// Map
// -------------------------------------------------------------------------

struct Map {
    width: u16,
    height: u16,
    tiles: Vec<Tile>,
}

impl Map {
    fn new(width: u16, height: u16) -> Self {
        let mut rng = rand::thread_rng();

        let weights = [50, 10, 15];
        let dist = WeightedIndex::new(weights).unwrap();

        let mut tiles = Vec::new();

        for _ in 0..(width * height) {
            tiles.push(Self::random_tile(&mut rng, &dist));
            }

        Self {
            width,
            height,
            tiles
        }
    }

    fn random_tile(
        rng: &mut impl Rng,
        dist: &WeightedIndex<u32>
        ) -> Tile {
            match dist.sample(rng) {
                0 => Tile::Grass,
                1 => Tile::Water,
                2 => Tile::Mountain,
                _ => unreachable!(),
            }
    }

    fn index(&self, x: u16, y: u16) -> usize {
        y as usize * self.width as usize + x as usize
    }

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0
            && x < self.width as i32
            && y >= 0
            && y < self.height as i32
    }

    fn get(&self, x: u16, y: u16) -> Tile {
        let index = self.index(x, y);
        self.tiles[index]
    }

    fn set(&mut self, x: u16, y: u16, tile: Tile) {
        let index = self.index(x, y);
        self.tiles[index] = tile;
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

                let index = self.index(j as u16, i as u16);

                match self.tiles[index] {
                    Tile::City      => city_count += 1,
                    Tile::Grass     => grass_count += 1,
                    Tile::Water     => water_count += 1,
                    Tile::Mountain  => {},
                }
            }
        }

        (city_count, grass_count, water_count)
    }

    fn step(&mut self) -> Vec<(u16, u16)> {
        let mut new_tiles = self.tiles.clone();
        let mut settlers_to_spawn = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.index(x, y);
                let tile = self.tiles[index];

                let (cities, grasses, waters) =
                    self.neighbors(x as i32, y as i32);

                match tile {
                    Tile::Grass
                        if cities > 0
                            && waters > 0
                            && grasses > 0 =>
                    {
                        new_tiles[index] = Tile::City;
                    }
                    Tile::City
                        if grasses > 3 && cities > waters =>
                    {
                        new_tiles[index] = Tile::Grass;
                        settlers_to_spawn.push((x, y));
                    },

                    _ => {},
                }
            }
        }

        self.tiles = new_tiles;

        settlers_to_spawn
    }
}


// -------------------------------------------------------------------------------
// Game
// -------------------------------------------------------------------------------

struct Game {
    map: Map,
    units: Vec<Unit>,
    selected: Option<usize>,
}

impl Game {
    fn new() -> Self {
        let map = Map::new(50, 25);
        
        Self {
            map: map,
            units: vec![
                Unit {
                    kind: UnitKind::Settler,
                    x: 5,
                    y: 5
                }
            ],
            selected: Some(0),
        }
    }

    fn move_selected(&mut self, dx: i16, dy: i16) {
        let Some(index) = self.selected else {
            return 
        };

        let unit = &mut self.units[index];

        let new_x = unit.x as i32 + dx as i32;
        let new_y = unit.y as i32 + dy as i32;

        if !self.map.in_bounds(new_x, new_y) {
            return
        };

        let new_x = new_x as u16;
        let new_y = new_y as u16;

        if self.map.get(new_x, new_y) == Tile::Water {
            return
        };

        unit.x = new_x;
        unit.y = new_y;
    }

    fn found_city(&mut self) {
        let Some(index) = self.selected else {
            return
        };

        if self.units[index].kind != UnitKind::Settler {
            return
        };

        let x = self.units[index].x;
        let y = self.units[index].y;

        if self.map.get(x, y) != Tile::Grass {
            return
        };
        
        self.map.set(x, y, Tile::City);

        self.units.remove(index);

        self.selected = self.units
            .len()
            .checked_sub(1);
    }

    fn spawn_settler(&mut self, x: u16, y: u16) {
        self.units.push(Unit {
            kind: UnitKind::Settler,
            x,
            y
        });

        self.selected = Some(self.units.len() - 1);
    }

    fn step(&mut self) {
        let spawn_locations = self.map.step();

        for (x, y) in spawn_locations {
            self.spawn_settler(x, y);
        }
    }
}


// ----------------------------------------------------------------------------------
// Render
// ----------------------------------------------------------------------------------

fn render(game: &Game) -> Paragraph<'static> {
    let mut lines = String::new();

    for y in 0..game.map.height {
        for x in 0..game.map.width {
            if let Some(unit) = game.units.iter().find(
                |unit| { unit.x == x && unit.y == y }
                ) {
                let ch = match unit.kind {
                    UnitKind::Settler => '@'
                };
                lines.push(ch);
            } else {
                let tile = game.map.get(x, y);

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


// ------------------------------------------------------------------------------------------
// Main
// ------------------------------------------------------------------------------------------

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut std_out = io::stdout();
    
    execute!(
        std_out,
        EnterAlternateScreen
        )?;
    
    let backend = CrosstermBackend::new(std_out);
    let mut terminal = Terminal::new(backend)?;
    
    let mut game = Game::new();

    loop {
        terminal.draw(|frame| {
            let paragraph = render(&game);
            frame.render_widget(paragraph, frame.area());
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q')  => break,
                KeyCode::Tab        => {
                    if game.units.is_empty() {
                        continue
                    }
                    let next = game
                        .selected
                        .map_or(0, |i| (i + 1) % game.units.len());
                    
                    game.selected = Some(next);
                },
                KeyCode::Char('b')  => game.found_city(),
                KeyCode::Up         => game.move_selected(0, -1),
                KeyCode::Down       => game.move_selected(0, 1),
                KeyCode::Left       => game.move_selected(-1, 0),
                KeyCode::Right      => game.move_selected(1, 0),
                KeyCode::Char('n')  => game.step(),
                _                   => {}
            }
        }
    }

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
        )?;
    Ok(())
}
