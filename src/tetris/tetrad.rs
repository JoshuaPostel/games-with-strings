extern crate ansi_term;
extern crate rand;
extern crate ndarray;
extern crate itertools;
extern crate termion;

use rand::Rng;
use rand::prelude::SliceRandom;

use crate::grid::rgb::RGB;
use crate::grid::grid::colored_char;

use super::tile::{Tile, OUTLINED_SQUARE};

#[derive(Clone, Debug)]
pub struct Tetrad {
    pub tiles: [Tile; 4],
    pub center: (f32, f32),
    pub render: String,
    pub name: String,
}


impl Tetrad {

    pub fn new_i() -> Tetrad {
        let light_blue  = RGB { r: 102, g: 255, b: 255 };
        let character = colored_char(OUTLINED_SQUARE, light_blue);
        let mut render = String::new();
        render.push_str("\n");
        render.push_str(&character);
        render.push_str(&character);
        render.push_str(&character);
        render.push_str(&character);
        render.push_str("\n ");

        Tetrad {
            tiles: [Tile { empty: true, color: light_blue, utf8: OUTLINED_SQUARE, row: 1, column: 3},
                    Tile { empty: true, color: light_blue, utf8: OUTLINED_SQUARE, row: 1, column: 4},
                    Tile { empty: true, color: light_blue, utf8: OUTLINED_SQUARE, row: 1, column: 5},
                    Tile { empty: true, color: light_blue, utf8: OUTLINED_SQUARE, row: 1, column: 6}
            ],
            center: (0.0, 5.0),
            render,
            name: "I".to_string(),
        }
    }

    pub fn new_o() -> Tetrad {
        let yello  = RGB { r: 255, g: 255, b: 102 };
        let character = colored_char(OUTLINED_SQUARE, yello);
        let mut render = String::new();
        render.push_str("\n");
        render.push_str(&character);
        render.push_str(&character);
        render.push_str("  ");
        render.push_str("  ");
        render.push_str("\n");
        render.push_str(&character);
        render.push_str(&character);

        Tetrad {
            tiles: [Tile { empty: true, color: yello, utf8: OUTLINED_SQUARE, row: 0, column: 4},
                    Tile { empty: true, color: yello, utf8: OUTLINED_SQUARE, row: 1, column: 4},
                    Tile { empty: true, color: yello, utf8: OUTLINED_SQUARE, row: 1, column: 5},
                    Tile { empty: true, color: yello, utf8: OUTLINED_SQUARE, row: 0, column: 5}
            ],
            center: (0.5, 4.5),
            render,
            name: "O".to_string(),

        }
    }

    pub fn new_t() -> Tetrad {
        let purple  = RGB { r: 178, g: 102, b: 255 };
        let character = colored_char(OUTLINED_SQUARE, purple);
        let mut render = String::new();
        render.push_str("\n");
        render.push_str("  ");
        render.push_str(&character);
        render.push_str("  ");
        render.push_str("  ");
        render.push_str("\n");
        render.push_str(&character);
        render.push_str(&character);
        render.push_str(&character);

        Tetrad {
            tiles: [Tile { empty: true, color: purple, utf8: OUTLINED_SQUARE, row: 1, column: 3},
                    Tile { empty: true, color: purple, utf8: OUTLINED_SQUARE, row: 1, column: 4},
                    Tile { empty: true, color: purple, utf8: OUTLINED_SQUARE, row: 1, column: 5},
                    Tile { empty: true, color: purple, utf8: OUTLINED_SQUARE, row: 0, column: 4}
            ],
            center: (1.0, 4.0),
            render,
            name: "T".to_string(),

        }
    }

    pub fn new_s() -> Tetrad {
        let green  = RGB { r: 102, g: 255, b: 102 };
        let character = colored_char(OUTLINED_SQUARE, green);
        let mut render = String::new();
        render.push_str("\n");
        render.push_str(&character);
        render.push_str(&character);
        render.push_str("  ");
        render.push_str("  ");
        render.push_str("\n");
        render.push_str("  ");
        render.push_str(&character);
        render.push_str(&character);

        Tetrad {
            tiles: [Tile { empty: true, color: green, utf8: OUTLINED_SQUARE, row: 1, column: 5},
                    Tile { empty: true, color: green, utf8: OUTLINED_SQUARE, row: 1, column: 6},
                    Tile { empty: true, color: green, utf8: OUTLINED_SQUARE, row: 0, column: 5},
                    Tile { empty: true, color: green, utf8: OUTLINED_SQUARE, row: 0, column: 4}
            ],
            center: (0.0, 5.0),
            render,
            name: "S".to_string(),

        }
    }

    pub fn new_z() -> Tetrad {
        let red  = RGB { r: 255, g: 0, b: 0 };
        let character = colored_char(OUTLINED_SQUARE, red);
        let mut render = String::new();
        render.push_str("\n");
        render.push_str("  ");
        render.push_str(&character);
        render.push_str(&character);
        render.push_str("  ");
        render.push_str("\n");
        render.push_str(&character);
        render.push_str(&character);

        Tetrad {
            tiles: [Tile { empty: true, color: red, utf8: OUTLINED_SQUARE, row: 0, column: 4},
                    Tile { empty: true, color: red, utf8: OUTLINED_SQUARE, row: 0, column: 5},
                    Tile { empty: true, color: red, utf8: OUTLINED_SQUARE, row: 1, column: 3},
                    Tile { empty: true, color: red, utf8: OUTLINED_SQUARE, row: 1, column: 4}
            ],
            center: (0.0, 4.0),
            render,
            name: "Z".to_string(),

        }
    }

    pub fn new_j() -> Tetrad {
        let orange  = RGB { r: 255, g: 153, b: 51 };
        let character = colored_char(OUTLINED_SQUARE, orange);
        let mut render = String::new();
        render.push_str("\n");
        render.push_str(&character);
        render.push_str("  ");
        render.push_str("  ");
        render.push_str("  ");
        render.push_str("\n");
        render.push_str(&character);
        render.push_str(&character);
        render.push_str(&character);

        Tetrad {
            tiles: [Tile { empty: true, color: orange, utf8: OUTLINED_SQUARE, row: 0, column: 3},
                    Tile { empty: true, color: orange, utf8: OUTLINED_SQUARE, row: 1, column: 4},
                    Tile { empty: true, color: orange, utf8: OUTLINED_SQUARE, row: 1, column: 5},
                    Tile { empty: true, color: orange, utf8: OUTLINED_SQUARE, row: 1, column: 3}
            ],
            center: (1.0, 4.0),
            render,
            name: "J".to_string(),

        }
    }

    pub fn new_l() -> Tetrad {
        let dark_blue  = RGB { r: 0, g: 0, b: 255 };
        let character = colored_char(OUTLINED_SQUARE, dark_blue);
        let mut render = String::new();
        render.push_str("\n");
        render.push_str("  ");
        render.push_str("  ");
        render.push_str(&character);
        render.push_str("  ");
        render.push_str("\n");
        render.push_str(&character);
        render.push_str(&character);
        render.push_str(&character);
        //render.push_str("\r");

        Tetrad {
            tiles: [Tile { empty: true, color: dark_blue, utf8: OUTLINED_SQUARE, row: 0, column: 6},
                    Tile { empty: true, color: dark_blue, utf8: OUTLINED_SQUARE, row: 1, column: 4},
                    Tile { empty: true, color: dark_blue, utf8: OUTLINED_SQUARE, row: 1, column: 5},
                    Tile { empty: true, color: dark_blue, utf8: OUTLINED_SQUARE, row: 1, column: 6}
            ],
            center: (1.0, 5.0),
            render,
            name: "L".to_string(),

        }
    }

    pub fn new_random() -> Tetrad {
        let x = rand::thread_rng().gen_range(0,7);
        match x {
            0 => Tetrad::new_i(),
            1 => Tetrad::new_o(),
            2 => Tetrad::new_t(),
            3 => Tetrad::new_s(),
            4 => Tetrad::new_z(),
            5 => Tetrad::new_j(),
            6 => Tetrad::new_l(),
            _ => Tetrad::new_l(),
        }
    }

    pub fn new_by_name(name: &str) -> Tetrad {
        match name {
            "I" => Tetrad::new_i(),
            "O" => Tetrad::new_o(),
            "T" => Tetrad::new_t(),
            "S" => Tetrad::new_s(),
            "Z" => Tetrad::new_z(),
            "J" => Tetrad::new_j(),
            "L" => Tetrad::new_l(),
            _ => Tetrad::new_l(),
        }
    }
}

#[derive(Default)]
pub struct Queue {
    pub tetrads: Vec<Tetrad>
}

impl Queue {

    pub fn new() -> Queue {
        Queue { tetrads: Queue::new_shuffled_seven() }
    }
    
    pub fn next_tetrad(&mut self) -> Tetrad {
        match self.tetrads.len() {
            0 ..= 6 => {
                self.tetrads.reverse();
                self.tetrads.append(&mut Queue::new_shuffled_seven());
                self.tetrads.reverse();
                self.tetrads.pop().unwrap()
            },
            _ => self.tetrads.pop().unwrap() 
        }
    }

    fn new_shuffled_seven() -> Vec<Tetrad> {
        let mut rng = rand::thread_rng();
        let mut queue: Vec<Tetrad> = Vec::new();
        queue.push(Tetrad::new_i());
        queue.push(Tetrad::new_o());
        queue.push(Tetrad::new_t());
        queue.push(Tetrad::new_s());
        queue.push(Tetrad::new_z());
        queue.push(Tetrad::new_j());
        queue.push(Tetrad::new_l());
        queue.shuffle(&mut rng);
        queue
    }
}
