use std::boxed::Box;
use crossterm::style::Color;

use crate::colors;

use super::ScreenState;

pub mod floor;
pub mod door;
pub mod walls;
pub mod obstacle;

pub trait MapElement {
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

    pub fn get_element(&self, x: usize, y: usize) -> Result<&dyn MapElement, ()> {
        if x < self.width && y < self.height {
            Ok(&*self.data[y][x])
        } else {
            Err(())
        }
    }

    pub fn get_element_as_mut(&mut self, x: usize, y: usize) -> Result<&mut dyn MapElement, ()> {
        if x < self.width && y < self.height {
            Ok(&mut *self.data[y][x])
        } else {
            Err(())
        } 
    }

    pub fn set_element(&mut self, x: usize, y: usize, map_element: Box<dyn MapElement>) -> Result<(), ()> {
        if x < self.width && y < self.height {
            self.data[y][x] = map_element;
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
        (self.data[y][x].get_char(), colors::DOORS_CLR)
    }

    pub fn get_screen(&self, left: usize, top: usize) -> ScreenState {
        let mut ss = ScreenState::new();
        for y in 0..(crate::options::N_HEIGHT as usize) {
            for x in 0..(crate::options::N_WIDTH as usize) {
                ss.set_element(x, y, self.get_char(x + left, y + top))
            }
        };
        ss
    }

}