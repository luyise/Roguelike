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
            sprite: '\u{25A0}'
        }
    }
}

impl MapElement for Floor {
    fn can_step_on(&self) -> bool {
        true
    }

    fn interact_short(&mut self) -> (String, Color) {
        (String::from("You shouldn't be here..."), Color::White)
    }

    fn interact_long(&mut self) -> (String, Color) {
        (String::from("Nothing to do with this thing"), Color::White)
    }

    fn get_char(&self) -> char {
        self.image
    }

    fn get_color(&self) -> Color {
        OBSTACLES_CLR
    }

    fn get_info(&self) -> Option<[String; 9]> {
        Some([
            String::from(" A U+25A0 character "),
            String::from("that seems to have  "),
            String::from("arrived there by    "),
            String::from("mistake.            "),
            String::from("                    "),
            String::from("                    "),
            String::from("                    "),
            String::from("                    "),
            String::from("                    ")
        ])
    }
}