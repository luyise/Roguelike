use super::MapElement;
use crate::colors::*;
use crossterm::style::Color;

#[derive(Debug, Copy, Clone)]
pub struct Floor {
    sprite: char,
}

impl Floor {
    pub fn new() -> Self {
        Self { sprite: ' ' }
    }
}

impl MapElement for Floor {
    fn to_box(self) -> Box<dyn MapElement> {
        Box::new(self)
    }

    fn can_step_on(&self) -> bool {
        true
    }

    fn interact_short(&mut self) -> (String, Color) {
        (String::from("Nothing here"), Color::White)
    }

    fn interact_long(&mut self) -> (String, Color) {
        (String::from("Nothing here"), Color::White)
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
