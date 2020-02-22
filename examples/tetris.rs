extern crate grid;

extern crate ansi_term;
extern crate rand;
extern crate ndarray;
extern crate itertools;
extern crate termion;

use rand::Rng;
use rand::prelude::SliceRandom;
use std::io::Read;
use termion::raw::IntoRawMode;

use grid::grid::rgb::{RGB};
use grid::grid::grid::{Grid, Depict};
use grid::tetris::tile::Tile;
use grid::tetris::tetrad::Tetrad;

const square: [u8; 4] = [0xE2, 0x96, 0xA0, 0x20];
const square_outline: [u8; 4] = [0xE2, 0x96, 0xA1, 0x20];
const outlined_square: [u8; 4] = [0xE2, 0x96, 0xA3, 0x20];

trait Update {

    fn add_tetrad(&mut self, tetrad: &Tetrad);
    fn remove_tetrad(&mut self, tetrad: &Tetrad);            

    fn add_tile(&mut self, tile: Tile);            
    fn remove_tile(&mut self, tile: Tile);            

    fn valid_tile(&self, tile: Tile) -> bool;            
    fn full_rows(&self) -> Vec<usize>;
    fn clear_rows(&mut self, full_rows: Vec<usize>);
}

impl Update for Grid<Tile> {


    fn add_tetrad(&mut self, tetrad: &Tetrad) {
        for tile in tetrad.tiles.iter() {
            self.add_tile(*tile);
        }
    }

    fn remove_tetrad(&mut self, tetrad: &Tetrad) {
        for tile in tetrad.tiles.iter() {
            self.remove_tile(*tile);
        }
    }

    fn add_tile(&mut self, tile: Tile) {
        self.grid[[tile.row, tile.column]] = tile;
    }

    fn remove_tile(&mut self, tile: Tile) {
        let default_tile = Tile { row: tile.row, column: tile.column, ..Default::default() };
        self.grid[[tile.row, tile.column]] = default_tile;
    }

    //TODO refactor
    fn valid_tile(&self, tile: Tile) -> bool {
        let row_good = 0 <= tile.row && tile.row < self.height;
        let column_good = 0 <= tile.column && tile.column < self.width;
        let mut location_ocupied = false;
        if row_good && column_good {
            location_ocupied = self.grid[[tile.row, tile.column]].empty;
        }
        row_good && column_good && location_ocupied
    }

    fn full_rows(&self) -> Vec<usize> {
        let mut full_rows = Vec::new();
        for row in self.grid.genrows() {
            let full_row = row.into_iter().all(|tile| tile.empty == false);
            if full_row {
                full_rows.push(row[0].row)
            }
        }
        full_rows
    }

    fn clear_rows(&mut self, full_rows: Vec<usize>) {
    
        for full_row in full_rows {
            let _ = self.grid.row_mut(full_row).map_mut(std::mem::take);
            for row_index in (0..full_row).rev() {
                let mut bottom_row = self.grid.row_mut(row_index).map_mut(std::mem::take);
                //not equiv?
                //bottom_row.iter_mut().map(|tile| tile.row += 1);
                for tile in bottom_row.iter_mut() {
                    tile.row += 1;
                }
                self.grid.row_mut(row_index + 1).assign(&bottom_row);
            }
        }
    }
}

trait Move {

    //TODO rename to move
    fn move_tetrad(&mut self, grid: &Grid<Tile>, tetrad_mover: Box<dyn Fn(&mut Tetrad)>);
}

impl Move for Tetrad {

    fn move_tetrad(&mut self, grid: &Grid<Tile>, tetrad_mover: Box<dyn Fn(&mut Tetrad)>) {
        let mut new_tetrad = self.clone();
        tetrad_mover(&mut new_tetrad);

        let valid_move = new_tetrad.tiles
            .iter()
            .all(|tile| grid.valid_tile(*tile));
        if valid_move {    
            tetrad_mover(self);
        }
    }
}

struct Tetris {
    grid: Grid<Tile>,
    active_tetrad: Tetrad,
    tetrad_shadow: Tetrad,
}

impl Tetris {

    fn move_active_tetrad(&mut self, tetrad_mover: Box<dyn Fn(&mut Tetrad)>) { 
        let mut tetrad = self.active_tetrad.clone();
        self.grid.remove_tetrad(&self.active_tetrad);
        tetrad.move_tetrad(&self.grid, tetrad_mover);
        self.grid.add_tetrad(&tetrad);
        self.active_tetrad = tetrad;
        self.update_shadow();
    }

    fn get_shadow(&self) -> Tetrad {
        let mut shadow = self.active_tetrad.clone();
        let mut rows_to_drop = 0;
        while shadow.tiles.iter().all(|tile| self.grid.valid_tile(*tile)) {
            shadow.tiles.iter_mut().for_each(|tile| tile.row += 1);
            rows_to_drop += 1;
        }
        for tile in shadow.tiles.iter_mut() {
            tile.row -= 1;
            tile.utf8 = square_outline;
        }
        shadow
    }

    fn update_shadow(&mut self) {
        self.grid.remove_tetrad(&self.tetrad_shadow);
        let shadow = self.get_shadow();
        self.grid.add_tetrad(&shadow);
        self.tetrad_shadow = shadow;
        self.grid.add_tetrad(&self.active_tetrad);
    } 

    fn hard_drop(&mut self) { 
        self.grid.remove_tetrad(&self.active_tetrad);
        let color = self.active_tetrad.tiles[0].color;
        let utf8 = self.active_tetrad.tiles[0].utf8;
        self.active_tetrad.tiles = self.tetrad_shadow.tiles;
        for tile in self.active_tetrad.tiles.iter_mut() {
            tile.color = color;
            tile.utf8 = utf8;
        }
        //self.move_active_tetrad(Box::new(hard_drop_tetrad))
    }

    //TODO
    //better python like function wrapping?
    fn move_left(&mut self) { 
        fn move_tetrad_left(tetrad: &mut Tetrad) {
            //TODO shouldnt need a specific check for move left 
            let legal = tetrad.tiles.iter().all(|x| x.column > 0);
            if legal {
                for tile in tetrad.tiles.iter_mut() {
                    tile.column -= 1;
                }
                tetrad.center.1 -= 1.0;
            }
        }
        self.move_active_tetrad(Box::new(move_tetrad_left))
    }

    fn move_right(&mut self) { 
        fn move_tetrad_right(tetrad: &mut Tetrad) {
            tetrad.tiles.iter_mut().for_each(|tile| tile.column += 1);
            tetrad.center.1 += 1.0;
        }
        self.move_active_tetrad(Box::new(move_tetrad_right))
    }
    
    fn move_down(&mut self) { 
        fn move_tetrad_down(tetrad: &mut Tetrad) {
            tetrad.tiles.iter_mut().for_each(|tile| tile.row += 1);
            tetrad.center.0 += 1.0;
        }
        self.move_active_tetrad(Box::new(move_tetrad_down))
    }

    fn rotate_tetrad(tetrad: &mut Tetrad, rotation_matrix: [[f32; 2]; 2]) {
        for tile in tetrad.tiles.iter_mut() {
            let row = tile.row as f32;
            let center_row = tetrad.center.0 as f32;
            let column = tile.column as f32;
            let center_column = tetrad.center.1 as f32;
            let normalized = ndarray::arr2(
                &[[row - center_row],[column - center_column]]);
            let rotation_matrix = ndarray::arr2(&rotation_matrix);
            let rotated = rotation_matrix.dot(&normalized);
            let new_row = rotated[[0,0]] + center_row;
            let new_column = rotated[[1,0]] + center_column;
            tile.row = new_row as usize;
            tile.column = new_column as usize;
        }
    }

    fn rotate_left(&mut self) {
        
        fn rotate_tetrad_left(tetrad: &mut Tetrad) {
            let rotation_matrix: [[f32; 2]; 2] = [[0.,-1.],[1.,0.]];
           // println!("{:?}", rotation_matrix);
            Tetris::rotate_tetrad(tetrad, rotation_matrix)
        }

        self.move_active_tetrad(Box::new(rotate_tetrad_left))
    }

    fn rotate_right(&mut self) {
        
        fn rotate_tetrad_right(tetrad: &mut Tetrad) {
            let rotation_matrix: [[f32; 2]; 2] = [[0.,1.],[-1.,0.]];
           // println!("{:?}", rotation_matrix);
            Tetris::rotate_tetrad(tetrad, rotation_matrix)
        }

        self.move_active_tetrad(Box::new(rotate_tetrad_right))
    }
}

fn vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matches = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matches == a.len() && matches == b.len()
}

fn main() {

    let width: usize = 10;
    let height: usize = 24;
    let mut tiles: Vec<Tile> = Vec::new();
    for x in 0..width {
    	for y in 0..height {
        	tiles.push(Tile::new(x, y))
		}
    }

    let g = Grid::new(width, height, tiles);
    let mut tetrad_queue = Tetrad::new_queue();

    let mut tetris = Tetris { grid: g, 
        active_tetrad: tetrad_queue.pop().unwrap(), 
        tetrad_shadow: Tetrad::new_L() };
    tetris.update_shadow();
    tetris.grid.add_tetrad(&tetris.active_tetrad);

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut input = termion::async_stdin().bytes();

    let mut game_live = true;


    while game_live {
        //print!("{}[2J", 27 as char);
        println!("{}", tetris.grid);

        //TODO stay dry
        let mut rows_before: Vec<usize> = Vec::new();
        let mut columns_before: Vec<usize> = Vec::new();
        for i in 0..4 {
            rows_before.push(tetris.active_tetrad.tiles[i].row);
            columns_before.push(tetris.active_tetrad.tiles[i].column);
        }
        tetris.move_down();

        //TODO stay dry
        let mut tetrad_rows: Vec<usize> = Vec::new();
        let mut tetrad_columns: Vec<usize> = Vec::new();
        for i in 0..4 {
            tetrad_rows.push(tetris.active_tetrad.tiles[i].row);
            tetrad_columns.push(tetris.active_tetrad.tiles[i].column);
        }

        if vecs_match(&tetrad_rows, &rows_before) && vecs_match(&tetrad_columns, &columns_before) {

            tetris.active_tetrad.tiles.iter_mut().for_each(|tile| tile.empty = false);
            tetris.grid.add_tetrad(&tetris.active_tetrad);

            for tile in tetris.active_tetrad.tiles.iter_mut() {
                tile.empty = false;
            }

            let full_rows = tetris.grid.full_rows();
            if full_rows.len() > 0 {
                tetris.grid.clear_rows(full_rows);
            }

            let new_tetrad = match tetrad_queue.pop() {
                Some(x) => x,
                _ => {
                    tetrad_queue = Tetrad::new_queue();
                    tetrad_queue.pop().unwrap()
                }
            };

            tetris.active_tetrad = new_tetrad;
            tetris.tetrad_shadow = tetris.get_shadow();
            tetris.grid.add_tetrad(&tetris.tetrad_shadow);

            let valid_move = tetris.active_tetrad.tiles
                .iter()
                .all(|tile| tetris.grid.valid_tile(*tile));
            if valid_move {    
                tetris.grid.add_tetrad(&tetris.active_tetrad);
            } else {
                break
            }
        }

        let mut next_drop = std::time::Duration::from_millis(1000);
        let last_drop = std::time::Instant::now();

        //TODO replace counter with time compairison?
        let mut counter = 0;
        loop {
            if counter == 0 {
                //print!("{}[2J", 27 as char);
                println!("{}", tetris.grid);
            }
            counter += 1;
            let mut hard_dropped = false;
            let time_elapsed = last_drop.elapsed();
            if time_elapsed >= next_drop {
                break;
            }
            match input.next() {
                None => continue,
                Some(Ok(b'j')) => tetris.move_left(),
                Some(Ok(b'k')) => tetris.move_right(),
                Some(Ok(b'g')) => tetris.move_down(),
                Some(Ok(b'd')) => tetris.rotate_left(),
                Some(Ok(b'f')) => tetris.rotate_right(),
                Some(Ok(b' ')) => {
                    tetris.hard_drop();
                    hard_dropped = true;
                    },
                Some(Ok(b'q')) => {
                    game_live = false;
                    break;
                },
                _ => break
            }
            if hard_dropped {
                break;
            }
            //print!("{}[2J", 27 as char);
            println!("{}", tetris.grid);
            next_drop -= time_elapsed;
        }
    }
    println!("GAME OVER\r\n");
}
