extern crate ansi_term;
extern crate rand;
extern crate ndarray;

use std::fmt;

use super::rgb::RGB;

pub trait Depict {
    
    fn color(&self) -> RGB;
    fn utf8(&self) -> [u8; 4];

}

pub fn colored_char(utf8: &[u8; 4], rgb: RGB) -> String {
    let character = std::str::from_utf8(utf8).unwrap();
    let colored_character = ansi_term::Color::RGB(rgb.r, rgb.g, rgb.b)
        .paint(character)
        .to_string();
    colored_character
}

#[derive(Clone, Debug)]
pub struct Grid<T: Depict> {
    pub width: usize,
    pub height: usize,
    pub grid: ndarray::ArrayBase<ndarray::OwnedRepr<T>, ndarray::Dim<[usize; 2]>>,
}

impl<T: Depict> Grid<T> {
    
    pub fn display_string(&self) -> String {
        let mut display_string: String = String::new();
        for row in self.grid.genrows() {
            for tile in row {
                //TODO figure out the ownership issue here
                let utf8 = &tile.utf8();
                let character = colored_char(utf8, tile.color());
                display_string.push_str(&character);
            }
            //display_string.push_str("\n\r");
            display_string.push_str("\n");
        }
        display_string
    }
}

impl<T: Depict> fmt::Display for Grid<T> {

    //TODO stay dry
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display_string: String = String::new();
        display_string.push_str("\x1B[2J");
        for row in self.grid.genrows() {
            for tile in row {
                //TODO figure out the ownership issue here
                let utf8 = &tile.utf8();
                let character = colored_char(utf8, tile.color());
                display_string.push_str(&character);
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
