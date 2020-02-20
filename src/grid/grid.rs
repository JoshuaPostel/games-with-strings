extern crate ansi_term;
extern crate rand;
extern crate ndarray;

use std::fmt;
use std::cmp;
use rand::Rng;

use super::rgb::RGB;

pub trait Depict {
    
    fn color(&self) -> RGB;
    fn utf8(&self) -> [u8; 4];

}

#[derive(Clone, Debug)]
pub struct Grid<T: Depict> {
    pub width: usize,
    pub height: usize,
    pub grid: ndarray::ArrayBase<ndarray::OwnedRepr<T>, ndarray::Dim<[usize; 2]>>,
}

impl<T: Depict> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //TODO add clear scree character here
        let mut display_string: String = String::new();
        for row in self.grid.genrows() {
            for tile in row {
                //TODO figure out the ownership issue here
                let utf8 = &tile.utf8();
                let character = std::str::from_utf8(utf8).unwrap();
                let color = &tile.color();
                let colored_square = ansi_term::Color::RGB(color.r, color.g, color.b)
                    .paint(character)
                    .to_string();
                display_string.push_str(&colored_square);
            }
            display_string.push_str("\r\n");
        }
        write!(f, "{}", display_string)
    }
}


impl<T: Depict> Grid<T> {
    pub fn new(width: usize, height: usize, tiles: Vec<T>) -> Grid<T> {
        let grid = ndarray::Array::from_shape_vec((height, width), tiles).unwrap();
        Grid { width: width, height: height, grid: grid }
    }
}
