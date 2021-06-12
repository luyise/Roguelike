use crossterm::style::Color;
use super::MapElement;
use crate::colors::*;


#[derive(Debug, Copy, Clone)]
pub struct Floor {
    sprite: char,
}

impl Floor {
    pub fn new() -> Self {
        Self {
            sprite: ' '
        }
    }
}

impl MapElement for Floor {
    fn can_step_on(&self) -> bool {
        true
    }

    fn interact_short(&mut self) -> String {
        String::from("Nothing here")
    }

    fn interact_long(&mut self) -> String {
        String::from("Nothing here")
    }

    fn get_char(&self) -> char {
        self.sprite
    }

    fn get_color(&self) -> Color {
        BACKGROUND_CLR
    }

    fn get_info(&self) -> Option<[String; 9]> {
        None
    }
}