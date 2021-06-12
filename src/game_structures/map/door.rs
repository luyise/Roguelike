use crossterm::style::Color;
use super::MapElement;
use crate::colors::*;


#[derive(Debug, Copy, Clone)]
pub struct Door {
    opened: bool,
    sprite: char,
}

impl Door {
    pub fn vertical() -> Self {
        Self {
            opened: false,
            sprite: '|',
        }
    }

    pub fn horizontal() -> Self {
        Self {
            opened: false,
            sprite: '-',
        }
    }
}

impl MapElement for Door {
    fn can_step_on(&self) -> bool {
        self.opened
    }

    fn interact_short(&mut self) -> String {
        String::from("Try not to get hurt!")
    }

    fn interact_long(&mut self) -> String {
        if self.opened {
            self.opened = false;
            String::from("The door is now closed")
        } else {
            self.opened = true;
            String::from("The door is now opened")
        }
    }

    fn get_char(&self) -> char {
        if self.opened {
            ' '
        } else {
            self.sprite
        }
    }

    fn get_color(&self) -> Color {
        DOORS_CLR
    }

    fn get_info(&self) -> [String; 9] {
        if self.opened {
            [
                String::from(" An opened door,    "),
                String::from("don't catch a cold! "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    ")
            ]
        } else {
            [
                String::from(" A closed door, it  "),
                String::from("doesn't seems to be "),
                String::from("locked.             "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    ")
            ]
        }
    }
}