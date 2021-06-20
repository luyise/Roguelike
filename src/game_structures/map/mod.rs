use crossterm::style::Color;
use std::boxed::Box;
use std::fmt;

use super::ScreenState;
use std::fs::File;
use std::io::Write;
use crate::level_generators::maptile::MapTile;

pub mod door;
pub mod floor;
pub mod obstacle;
pub mod walls;

pub trait MapElement {
    fn to_box(self) -> Box<dyn MapElement>;

    fn can_step_on(&self) -> bool;

    fn interact_short(&mut self) -> (String, Color);

    fn interact_long(&mut self) -> (String, Color);

    fn get_char(&self) -> char;

    fn get_color(&self) -> Color;

    fn get_info(&self) -> Option<[String; 9]>;
}

pub struct Map {
    height: usize,
    width: usize,
    data: Vec<Vec<Box<dyn MapElement>>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let mut data: Vec<Vec<Box<dyn MapElement>>> = Vec::new();
        for _ in 0..height {
            let mut line: Vec<Box<dyn MapElement>> = Vec::new();
            for _ in 0..width {
                line.push(Box::new(floor::Floor::new()))
            }
            data.push(line);
        }
        Self {
            height,
            width,
            data,
        }
    }

    pub fn from_tile(data_raw: Vec<Vec<MapTile>>) -> Self {
        let mut data = Vec::new();
        let height = data_raw.len();
        let width;
        if height == 0 {
            width = 0;
        } else {
            width = data_raw[0].len();
            for row_raw in data_raw.iter() {
                let mut row: Vec<Box<dyn MapElement>> = Vec::new();
                for case in row_raw.iter() {
                    row.push(match case {
                            MapTile::Wall => Box::new(
                                walls::Wall::full()
                            ),
                            MapTile::Empty => Box::new(
                                floor::Floor::new()
                            ),
                            MapTile::DoorV => Box::new(
                                door::Door::vertical()
                            ),
                            MapTile::DoorH => Box::new(
                                door::Door::horizontal()
                            ),
                            MapTile::DoorD1 => Box::new(
                                door::Door::from_char('\\')
                            ),
                            MapTile::DoorD2 => Box::new(
                                door::Door::from_char('/')
                            ),
                            MapTile::Door => Box::new(
                                door::Door::from_char('+')
                            )
                        }
                    )
                }
                data.push(row)
            }
        }
        Self {
            height,
            width,
            data,
        }
    }

    pub fn get_element(&self, x: usize, y: usize) -> Result<&dyn MapElement, ()> {
        if x < self.width && y < self.height {
            Ok(&*self.data[x][y])
        } else {
            Err(())
        }
    }

    pub fn get_element_as_mut(&mut self, x: usize, y: usize) -> Result<&mut dyn MapElement, ()> {
        if x < self.width && y < self.height {
            Ok(&mut *self.data[x][y])
        } else {
            Err(())
        }
    }

    pub fn set_element(
        &mut self,
        x: usize,
        y: usize,
        map_element: Box<dyn MapElement>,
    ) -> Result<(), ()> {
        if x < self.width && y < self.height {
            self.data[x][y] = map_element;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    fn get_char(&self, x: usize, y: usize) -> (char, Color) {
        (self.data[x][y].get_char(), self.data[x][y].get_color())
    }

    pub fn get_screen(&self, left: usize, top: usize) -> ScreenState {
        let mut ss = ScreenState::new();
        for y in 0..(crate::options::N_HEIGHT as usize) {
            for x in 0..(crate::options::N_WIDTH as usize) {
                ss.set_element(x, y, self.get_char(x + left, y + top))
            }
        }
        ss
    }

    pub fn save(&self, file: &mut File) -> std::io::Result<usize> {
        file.write(b"Map saving not implemented")
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Map")
            .field("h", &self.height)
            .field("w", &self.width)
            .finish()
    }
}