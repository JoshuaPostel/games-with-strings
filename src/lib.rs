extern crate ansi_term;
extern crate rand;
extern crate ndarray;

use std::fmt;
use std::cmp;
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

    pub fn mix_color(&mut self, color: RGB, weight: f32) {
        let complement = 1. - weight;
        let r = (self.r as f32 * complement) + (color.r as f32 * weight);
        let g = (self.g as f32 * complement) + (color.g as f32 * weight);
        let b = (self.b as f32 * complement) + (color.b as f32 * weight);
        self.r = cmp::max(0, cmp::min(255, r as i32)) as u8;
        self.g = cmp::max(0, cmp::min(255, g as i32)) as u8;
        self.b = cmp::max(0, cmp::min(255, b as i32)) as u8;
    }
}


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
