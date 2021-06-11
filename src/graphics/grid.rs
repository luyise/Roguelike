

use crate::graphics::chars;
use std::cmp::{max, min};

const NW : u8 = 1 << 0;
const  W : u8 = 1 << 1;
const SW : u8 = 1 << 2;
const N  : u8 = 1 << 3;
const S  : u8 = 1 << 4;
const NE : u8 = 1 << 5;
const  E : u8 = 1 << 6;
const SE : u8 = 1 << 7;

pub enum GridStyle {
    Single,
    Double,
}


// Structure to grad the lines for a grid
pub struct Grid {
    map : Vec<Vec<u8>>,
}

impl Grid {

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            map : vec![vec![0; width]; height]
        }
    }

    pub fn add_point(&mut self, x: usize, y: usize, index: u8) -> Result<(), ()> {
        if y >= self.map.len() {
            Err(())
        } else if x >= self.map[0].len() {
            Err(())
        } else {
            self.map[y][x] = index;
            Ok(())
        }
    }


    // Draw a line of id: index from (x1, y1) to (x2, y2). The line must be horizontal or vertical!
    pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2 : usize, index: u8) -> Result<(), ()> {
        if x1 == x2 {
            if !self.map.is_empty() && self.map[0].len() > x1 {
                if self.map.len() > y1 && self.map.len() > y2 {
                    for y in min(y1, y2)..=max(y1, y2) {
                        self.map[y][x1] = index;
                    }
                    Ok(())
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        } else {
            if y1 == y2 {
                if self.map.len() > y1 {
                    if self.map[0].len() > x1 && self.map[0].len() > x2 {
                        for x in min(x1, x2)..=max(x1, x2) {
                            self.map[y1][x] = index;
                        }
                        Ok(())
                    } else {
                        Err(())
                    }
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        }
    }


    // Choose type of grid
    pub fn to_string(&self, grid_type: GridStyle) -> Vec<String> {
        let mut v = Vec::new();
        for (y, line) in self.map.iter().enumerate() {
            let mut s = String::new();
            for (x, pos) in line.iter().enumerate() {
                if *pos == 0 {
                    s.push(' ');
                } else {
                    let neig = self.get_identical_neigh(x, y);
                    s.push(char_of_grid(neig, &grid_type));
                }
            }
            v.push(s)
        }
        v
    }

    // Assumes the map is not empty and that (x, y) is inside it
    // Get a bitflag indicating which neighbourds of a pos is of the same index
    fn get_identical_neigh(&self, x: usize, y: usize) -> u8 {
        let mut n = 0;
        if x > 0 {
            if self.map[y][x] == self.map[y][x - 1] {
                n |= W;
            }
            if y > 0 && self.map[y][x] == self.map[y - 1][x - 1] {
                n |= NW;
            }

            if y < self.map.len() - 1 && self.map[y][x] == self.map[y + 1][x - 1] {
                n |= SW;
            }
        }

        if y > 0 && self.map[y][x] == self.map[y - 1][x] {
            n |= N;
        }

        if y < self.map.len() - 1 && self.map[y][x] == self.map[y + 1][x] {
            n |= S;
        }

        if x < self.map[0].len() - 1 {
            if self.map[y][x] == self.map[y][x + 1] {
                n |= E;
            }
            if y > 0 && self.map[y][x] == self.map[y - 1][x + 1] {
                n |= NE;
            }

            if y < self.map.len() - 1 && self.map[y][x] == self.map[y + 1][x + 1] {
                n |= SE;
            }
        }
        n
    }
}

fn contains(a: u8, b: u8) -> bool {
    a == a | b
}


macro_rules! base_case {
    ($gs: ident; $before: ident) => {
        match $gs {
            GridStyle::Single => chars::sg_pipe::$before,
            GridStyle::Double => chars::db_pipe::$before,
        }
    };
}


fn char_of_grid(n: u8, grid_style: &GridStyle) -> char {
    if contains(n, N) {
        if contains(n, S) {
            if contains(n, E) {
                if contains(n, W) {
                    base_case!(grid_style; NSEW)
                } else {
                    base_case!(grid_style; NSE_)
                }
            } else {
                if contains(n, W) {
                    base_case!(grid_style; NS_W)
                } else {
                    base_case!(grid_style; NS__)
                }
            }
        } else {
            if contains(n, E) {
                if contains(n, W) {
                    base_case!(grid_style; N_EW)
                } else {
                    base_case!(grid_style; N_E_)
                }
            } else {
                if contains(n, W) {
                    base_case!(grid_style; N__W)
                } else {
                    base_case!(grid_style; N___)
                }
            }
        }
    } else {
        if contains(n, S) {
            if contains(n, E) {
                if contains(n, W) {
                    base_case!(grid_style; _SEW)
                } else {
                    base_case!(grid_style; _SE_)
                }
            } else {
                if contains(n, W) {
                    base_case!(grid_style; _S_W)
                } else {
                    base_case!(grid_style; _S__)
                }
            }
        } else {
            if contains(n, E) {
                if contains(n, W) {
                    base_case!(grid_style; __EW)
                } else {
                    base_case!(grid_style; __E_)
                }
            } else {
                if contains(n, W) {
                    base_case!(grid_style; ___W)
                } else {
                    base_case!(grid_style; ____)
                }
            }
        }
    }
}