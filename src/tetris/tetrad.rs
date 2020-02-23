
extern crate ansi_term;
extern crate rand;
extern crate ndarray;
extern crate itertools;
extern crate termion;

use rand::Rng;
use rand::prelude::SliceRandom;
use std::io::Read;
use termion::raw::IntoRawMode;

use crate::grid::rgb::RGB;
use crate::grid::grid::{Grid, Depict, colored_char};

use super::tile::Tile;

const square: [u8; 4] = [0xE2, 0x96, 0xA0, 0x20];
const square_outline: [u8; 4] = [0xE2, 0x96, 0xA1, 0x20];
const outlined_square: [u8; 4] = [0xE2, 0x96, 0xA3, 0x20];

#[derive(Clone, Debug)]
pub struct Tetrad {
    pub tiles: [Tile; 4],
    pub center: (f32, f32),
    pub render: String
}


impl Tetrad {

    pub fn new_I() -> Tetrad {
        let light_blue  = RGB { r: 102, g: 255, b: 255 };
        let character = colored_char(&outlined_square, light_blue);
        let mut render = String::new();
        render.push_str("\n\r");
        render.push_str(&character);
        render.push_str(&character);
        render.push_str(&character);
        render.push_str(&character);
        render.push_str("\r");

        Tetrad {
            tiles: [Tile { empty: true, color: light_blue, utf8: outlined_square, row: 1, column: 3},
                    Tile { empty: true, color: light_blue, utf8: outlined_square, row: 1, column: 4},
                    Tile { empty: true, color: light_blue, utf8: outlined_square, row: 1, column: 5},
                    Tile { empty: true, color: light_blue, utf8: outlined_square, row: 1, column: 6}
            ],
            center: (0.0, 5.0),
            render: render
        }
    }

    pub fn new_O() -> Tetrad {
        let yello  = RGB { r: 255, g: 255, b: 102 };
        let character = colored_char(&outlined_square, yello);
        let mut render = String::new();
        render.push_str("\n\r");
        render.push_str(&character);
        render.push_str(&character);
        render.push_str("\n\r");
        render.push_str(&character);
        render.push_str(&character);
        render.push_str("\r");

        Tetrad {
            tiles: [Tile { empty: true, color: yello, utf8: outlined_square, row: 0, column: 4},
                    Tile { empty: true, color: yello, utf8: outlined_square, row: 1, column: 4},
                    Tile { empty: true, color: yello, utf8: outlined_square, row: 1, column: 5},
                    Tile { empty: true, color: yello, utf8: outlined_square, row: 0, column: 5}
            ],
            center: (0.5, 4.5),
            render: render
        }
    }

    pub fn new_T() -> Tetrad {
        let purple  = RGB { r: 178, g: 102, b: 255 };
        let character = colored_char(&outlined_square, purple);
        let mut render = String::new();
        render.push_str("\n\r");
        render.push_str("  ");
        render.push_str(&character);
        render.push_str("\n\r");
        render.push_str(&character);
        render.push_str(&character);
        render.push_str(&character);
        render.push_str("\r");

        Tetrad {
            tiles: [Tile { empty: true, color: purple, utf8: outlined_square, row: 1, column: 3},
                    Tile { empty: true, color: purple, utf8: outlined_square, row: 1, column: 4},
                    Tile { empty: true, color: purple, utf8: outlined_square, row: 1, column: 5},
                    Tile { empty: true, color: purple, utf8: outlined_square, row: 0, column: 4}
            ],
            center: (1.0, 4.0),
            render: render
        }
    }

    pub fn new_S() -> Tetrad {
        let green  = RGB { r: 102, g: 255, b: 102 };
        let character = colored_char(&outlined_square, green);
        let mut render = String::new();
        render.push_str("\n\r");
        render.push_str("  ");
        render.push_str(&character);
        render.push_str(&character);
        render.push_str("\n\r");
        render.push_str(&character);
        render.push_str(&character);
        render.push_str("\r");

        Tetrad {
            tiles: [Tile { empty: true, color: green, utf8: outlined_square, row: 1, column: 5},
                    Tile { empty: true, color: green, utf8: outlined_square, row: 1, column: 6},
                    Tile { empty: true, color: green, utf8: outlined_square, row: 0, column: 5},
                    Tile { empty: true, color: green, utf8: outlined_square, row: 0, column: 4}
            ],
            center: (0.0, 5.0),
            render: render
        }
    }

    pub fn new_Z() -> Tetrad {
        let red  = RGB { r: 255, g: 0, b: 0 };
        let character = colored_char(&outlined_square, red);
        let mut render = String::new();
        render.push_str("\n\r");
        render.push_str(&character);
        render.push_str(&character);
        render.push_str("\n\r");
        render.push_str("  ");
        render.push_str(&character);
        render.push_str(&character);
        render.push_str("\r");
        Tetrad {
            tiles: [Tile { empty: true, color: red, utf8: outlined_square, row: 0, column: 4},
                    Tile { empty: true, color: red, utf8: outlined_square, row: 0, column: 5},
                    Tile { empty: true, color: red, utf8: outlined_square, row: 1, column: 3},
                    Tile { empty: true, color: red, utf8: outlined_square, row: 1, column: 4}
            ],
            center: (0.0, 4.0),
            render: render
        }
    }

    pub fn new_J() -> Tetrad {
        let orange  = RGB { r: 255, g: 153, b: 51 };
        let character = colored_char(&outlined_square, orange);
        let mut render = String::new();
        render.push_str("\n\r");
        render.push_str(&character);
        render.push_str("\n\r");
        render.push_str(&character);
        render.push_str(&character);
        render.push_str(&character);
        render.push_str("\r");

        Tetrad {
            tiles: [Tile { empty: true, color: orange, utf8: outlined_square, row: 0, column: 3},
                    Tile { empty: true, color: orange, utf8: outlined_square, row: 1, column: 4},
                    Tile { empty: true, color: orange, utf8: outlined_square, row: 1, column: 5},
                    Tile { empty: true, color: orange, utf8: outlined_square, row: 1, column: 3}
            ],
            center: (1.0, 4.0),
            render: render
        }
    }

    pub fn new_L() -> Tetrad {
        let dark_blue  = RGB { r: 0, g: 0, b: 255 };
        let character = colored_char(&outlined_square, dark_blue);
        let mut render = String::new();
        render.push_str("\n\r");
        render.push_str("  ");
        render.push_str("  ");
        render.push_str(&character);
        render.push_str("\n\r");
        render.push_str(&character);
        render.push_str(&character);
        render.push_str(&character);
        render.push_str("\r");

        Tetrad {
            tiles: [Tile { empty: true, color: dark_blue, utf8: outlined_square, row: 0, column: 6},
                    Tile { empty: true, color: dark_blue, utf8: outlined_square, row: 1, column: 4},
                    Tile { empty: true, color: dark_blue, utf8: outlined_square, row: 1, column: 5},
                    Tile { empty: true, color: dark_blue, utf8: outlined_square, row: 1, column: 6}
            ],
            center: (1.0, 5.0),
            render: render
        }
    }

    pub fn new_random() -> Tetrad {
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

    pub fn new_queue() -> Vec<Tetrad> {
        let mut rng = rand::thread_rng();
        let mut queue: Vec<Tetrad> = Vec::new();
        queue.push(Tetrad::new_I());
        queue.push(Tetrad::new_O());
        queue.push(Tetrad::new_T());
        queue.push(Tetrad::new_S());
        queue.push(Tetrad::new_Z());
        queue.push(Tetrad::new_J());
        queue.push(Tetrad::new_L());
        queue.shuffle(&mut rng);
        queue
    }

}
