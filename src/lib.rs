extern crate ansi_term;
extern crate rand;
extern crate ndarray;

use std::fmt;
use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn new_random() -> RGB {
        RGB { 
            r: rand::thread_rng().gen_range(0, 255),
            g: rand::thread_rng().gen_range(0, 255),
            b: rand::thread_rng().gen_range(0, 255),
        }
    }
}

pub trait Color {
    
    fn color(&self) -> RGB;

}

#[derive(Clone, Debug)]
pub struct Grid<T: Color> {
    pub width: usize,
    pub height: usize,
    pub grid: ndarray::ArrayBase<ndarray::OwnedRepr<T>, ndarray::Dim<[usize; 2]>>,
}

impl<T: Color> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display_string: String = String::new();
        let square = std::str::from_utf8(&[0xE2, 0x96, 0xA0, 0x20]).unwrap();
        for row in self.grid.genrows() {
            for tile in row {
                let color = &tile.color();
                let colored_square = ansi_term::Color::RGB(color.r, color.g, color.b)
                    .paint(square)
                    .to_string();
                display_string.push_str(&colored_square);
            }
            display_string.push_str("\r\n");
        }
        write!(f, "{}", display_string)
    }
}


impl<T: Color> Grid<T> {
    pub fn new(width: usize, height: usize, tiles: Vec<T>) -> Grid<T> {
        let grid = ndarray::Array::from_shape_vec((height, width), tiles).unwrap();
        Grid { width: width, height: height, grid: grid }
    }
}
