extern crate ansi_term;
extern crate rand;
extern crate ndarray;

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
