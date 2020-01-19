extern crate grid;

extern crate ansi_term;
extern crate rand;
extern crate ndarray;
extern crate itertools;
extern crate termion;

use rand::Rng;
use std::io::Read;
use termion::raw::IntoRawMode;

use grid::{Color, RGB, Grid};

#[derive(Copy, Clone, Debug)]
struct Tile {
    empty: bool,
    color: RGB,
    row: usize,
    column: usize,
}

impl Default for Tile {
    fn default() -> Tile {
        Tile {
            empty: true,
            color: RGB { r: 47, g: 79, b: 79},
            row: 0,
            column: 0,
        }
    }
}

impl Tile {

    fn new(row: usize, column: usize) -> Tile {
        Tile { row, column, ..Default::default() }
    }
}

impl Color for Tile {

    fn color(&self) -> RGB {
        self.color
    }
}

impl Tile {

    fn advance(&mut self) {
        self.row += 1;
    }
}


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
    
        //let full_row = 20;
        let full_row = full_rows[0];
        for full_row in full_rows {
            let _ = self.grid.row_mut(full_row).map_mut(std::mem::take);
            for row_index in (0..full_row).rev() {
                //println!("{}", row_index);
                let bottom_row = self.grid.row_mut(row_index).map_mut(std::mem::take);
                self.grid.row_mut(row_index + 1).assign(&bottom_row);
            }
        }
    }
}

#[derive(Clone)]
struct Tetrad {
    tiles: [Tile; 4],
    center: (f32, f32)
}

impl Tetrad {

    fn new_I() -> Tetrad {
        let light_blue  = RGB { r: 102, g: 255, b: 255 };
        Tetrad {
            tiles: [Tile { empty: false, color: light_blue, row: 1, column: 3},
                    Tile { empty: false, color: light_blue, row: 1, column: 4},
                    Tile { empty: false, color: light_blue, row: 1, column: 5},
                    Tile { empty: false, color: light_blue, row: 1, column: 6}
            ],
            center: (0.0, 5.0)
        }
    }

    fn new_O() -> Tetrad {
        let yello  = RGB { r: 255, g: 255, b: 102 };
        Tetrad {
            tiles: [Tile { empty: false, color: yello, row: 0, column: 4},
                    Tile { empty: false, color: yello, row: 1, column: 4},
                    Tile { empty: false, color: yello, row: 1, column: 5},
                    Tile { empty: false, color: yello, row: 0, column: 5}
            ],
            center: (0.5, 4.5)
        }
    }

    fn new_T() -> Tetrad {
        let purple  = RGB { r: 178, g: 102, b: 255 };
        Tetrad {
            tiles: [Tile { empty: false, color: purple, row: 1, column: 3},
                    Tile { empty: false, color: purple, row: 1, column: 4},
                    Tile { empty: false, color: purple, row: 1, column: 5},
                    Tile { empty: false, color: purple, row: 0, column: 4}
            ],
            center: (1.0, 4.0)
        }
    }

    fn new_S() -> Tetrad {
        let green  = RGB { r: 102, g: 255, b: 102 };
        Tetrad {
            tiles: [Tile { empty: false, color: green, row: 1, column: 5},
                    Tile { empty: false, color: green, row: 1, column: 6},
                    Tile { empty: false, color: green, row: 0, column: 5},
                    Tile { empty: false, color: green, row: 0, column: 4}
            ],
            center: (0.0, 5.0)
        }
    }

    fn new_Z() -> Tetrad {
        let red  = RGB { r: 255, g: 0, b: 0 };
        Tetrad {
            tiles: [Tile { empty: false, color: red, row: 0, column: 4},
                    Tile { empty: false, color: red, row: 0, column: 5},
                    Tile { empty: false, color: red, row: 1, column: 3},
                    Tile { empty: false, color: red, row: 1, column: 4}
            ],
            center: (0.0, 4.0)
        }
    }

    fn new_J() -> Tetrad {
        let orange  = RGB { r: 255, g: 153, b: 51 };
        Tetrad {
            tiles: [Tile { empty: false, color: orange, row: 0, column: 3},
                    Tile { empty: false, color: orange, row: 1, column: 4},
                    Tile { empty: false, color: orange, row: 1, column: 5},
                    Tile { empty: false, color: orange, row: 1, column: 3}
            ],
            center: (1.0, 4.0)
        }
    }

    fn new_L() -> Tetrad {
        let dark_blue  = RGB { r: 0, g: 0, b: 255 };
        Tetrad {
            tiles: [Tile { empty: false, color: dark_blue, row: 0, column: 6},
                    Tile { empty: false, color: dark_blue, row: 1, column: 4},
                    Tile { empty: false, color: dark_blue, row: 1, column: 5},
                    Tile { empty: false, color: dark_blue, row: 1, column: 6}
            ],
            center: (1.0, 5.0)
        }
    }

    fn new_random() -> Tetrad {
        let x = rand::thread_rng().gen_range(0,7);
        match x {
            0 => Tetrad::new_I(),
            1 => Tetrad::new_O(),
            2 => Tetrad::new_T(),
            3 => Tetrad::new_S(),
            4 => Tetrad::new_Z(),
            5 => Tetrad::new_J(),
            6 => Tetrad::new_L(),
            _ => Tetrad::new_L(),
        }
    
    }

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
}

impl Tetris {

    fn move_active_tetrad(&mut self, tetrad_mover: Box<dyn Fn(&mut Tetrad)>) { 
        let mut tetrad = self.active_tetrad.clone();
        self.grid.remove_tetrad(&self.active_tetrad);
        tetrad.move_tetrad(&self.grid, tetrad_mover);
        self.grid.add_tetrad(&tetrad);
        self.active_tetrad = tetrad;
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
            for tile in tetrad.tiles.iter_mut() {
                tile.column += 1;
            }
            tetrad.center.1 += 1.0;
        }
        self.move_active_tetrad(Box::new(move_tetrad_right))
    }
    
    fn move_down(&mut self) { 
        fn move_tetrad_down(tetrad: &mut Tetrad) {
            for tile in tetrad.tiles.iter_mut() {
                tile.row += 1;
            }
            tetrad.center.0 += 1.0;
        }
        self.move_active_tetrad(Box::new(move_tetrad_down))
    }

    fn rotate(&mut self) {
        fn rotate_tetrad(tetrad: &mut Tetrad) {
            for tile in tetrad.tiles.iter_mut() {
                let row = tile.row as f32;
                let center_row = tetrad.center.0 as f32;
                let column = tile.column as f32;
                let center_column = tetrad.center.1 as f32;

                let normalized = ndarray::arr2(
                    &[[row - center_row],[column - center_column]]);

                let rotation_matrix = ndarray::arr2(&[[0.,-1.],[1.,0.]]);

                let rotated = rotation_matrix.dot(&normalized);

                let new_row = rotated[[0,0]] + center_row;
                let new_column = rotated[[1,0]] + center_column;
                tile.row = new_row as usize;
                tile.column = new_column as usize;
            }
        }
        self.move_active_tetrad(Box::new(rotate_tetrad))
    }
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

    let mut g = Grid::new(width, height, tiles);
    let i = Tetrad::new_L();
    let mut tetris = Tetris { grid: g, active_tetrad: i };
    tetris.grid.add_tetrad(&tetris.active_tetrad);
    let mut row_before = 0;
    let mut column_before = 0;

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut input = termion::async_stdin().bytes();

    let mut game_live = true;

    while game_live {
        //print!("{}[2J", 27 as char);
        println!("{}", tetris.grid);
        tetris.move_down();
        if tetris.active_tetrad.tiles[0].row == row_before && tetris.active_tetrad.tiles[0].column == column_before {

            let full_rows = tetris.grid.full_rows();
            if full_rows.len() > 0 {
                //println!("full rows: {:?}", full_rows);
                tetris.grid.clear_rows(full_rows);
            }

            tetris.active_tetrad = Tetrad::new_random();

            let valid_move = tetris.active_tetrad.tiles
                .iter()
                .all(|tile| tetris.grid.valid_tile(*tile));
            if valid_move {    
                tetris.grid.add_tetrad(&tetris.active_tetrad);
            } else {
                break
            }
        }
        row_before = tetris.active_tetrad.tiles[0].row;
        column_before = tetris.active_tetrad.tiles[0].column;

        let mut next_drop = std::time::Duration::from_millis(1000);
        let last_drop = std::time::Instant::now();

        loop {
            let time_elapsed = last_drop.elapsed();
            if time_elapsed >= next_drop {
                break;
            }
            match input.next() {
                None => continue,
                Some(Ok(b'h')) => tetris.move_left(),
                Some(Ok(b'j')) => tetris.move_down(),
                Some(Ok(b'k')) => tetris.rotate(),
                Some(Ok(b'l')) => tetris.move_right(),
                Some(Ok(b'q')) => {
                    game_live = false;
                    break;
                },
                _ => break
            }
            //print!("{}[2J", 27 as char);
            println!("{}", tetris.grid);
            next_drop -= time_elapsed;
        }
    }
    println!("GAME OVER\r\n");
}
