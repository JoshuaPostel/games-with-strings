extern crate grid;

extern crate ansi_term;
extern crate rand;
extern crate ndarray;
extern crate itertools;

use grid::grid::grid::{Depict, Grid};
use grid::grid::rgb::RGB;

use rand::Rng;
use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
struct Tile {
    alive: bool,
    color: RGB,
}

impl Tile {

    fn new(alive: bool) -> Tile {
        let color = if alive {
            RGB { r: 225, g: 225, b: 102}
        } else {
            RGB { r: 47, g: 79, b: 79}
        };
        Tile {
            alive: alive,
            color: color
        }
    }

    fn new_random(probability_alive: f32) -> Tile {
        let alive = rand::thread_rng().gen_range(0.0, 1.0) <= probability_alive;
        Tile::new(alive)
    }
}

impl Depict for Tile {

    fn color(&self) -> RGB {
        self.color
    }

    fn utf8(&self) -> [u8; 4] {
        [0xE2, 0x96, 0xA0, 0x20]
    }
}

fn moore_neighbors(gol_grid: &Grid<Tile>, x: usize, y: usize) -> Vec<&Tile> {
    let mut neighbors = Vec::new(); 
    for (dx, dy) in (-1..2).cartesian_product(-1..2) {
        if (dx, dy) != (0, 0) {
            let x_idx = x as i32 + dx;
            let y_idx = y as i32 + dy;
            if x_idx < 0 || y_idx < 0 { continue }
            match gol_grid.grid.get((x_idx as usize, y_idx as usize)) {
                Some(tile) => neighbors.push(tile),
                None => continue,
            }
        }  
    }
    neighbors
}

fn becomes_alive(gol_grid: &Grid<Tile>, x: usize, y: usize) -> bool {
    let tiles: Vec<&Tile> = moore_neighbors(gol_grid, x, y);
    let live_neighbors = tiles.iter().map(|x| x.alive as i32).sum::<i32>();
    let was_alive = gol_grid.grid.get((x, y)).unwrap().alive;
    let alive = if was_alive {
        2 <= live_neighbors && live_neighbors <= 3
    } else {
        3 == live_neighbors 
    };
    alive
}

trait Update {

    fn update(&mut self);            
}


impl Update for Grid<Tile> {

    fn update(&mut self) {
        let mut tiles: Vec<Tile> = Vec::new();
        for ((x, y), _tile) in self.grid.indexed_iter() {
            let alive = becomes_alive(self, x, y);
            tiles.push(Tile::new(alive));
        }
        self.grid = ndarray::Array::from_shape_vec((self.height, self.width), tiles).unwrap();
    }
}


fn main() {

    let width = 50;
    let height = 50;
    let mut tiles: Vec<Tile> = Vec::new();
    for _ in 0..(width*height) {
        tiles.push(Tile::new_random(0.25))
    }

    let mut g = Grid::new(width, height, tiles);
    loop {
        print!("{}[2J", 27 as char);
        println!("{}", g);
        g.update();
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}
