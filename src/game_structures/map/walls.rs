use crossterm::style::Color;
use super::MapElement;
use crate::colors::*;
use crate::graphics::chars::db_pipe;
use std::boxed::Box;

#[derive(Debug, Copy, Clone)]
pub struct Wall {
    sprite: char,
}

impl Wall {
    pub fn new(s: &str) -> Self {
        Self {
            sprite: match s {
                "NSEW" => db_pipe::NSEW,
                "_SEW" => db_pipe::_SEW,
                "N_EW" => db_pipe::N_EW,
                "NS_W" => db_pipe::NS_W,
                "NSE_" => db_pipe::NSE_,
                "NS__" => db_pipe::NS__,
                "N_E_" => db_pipe::N_E_,
                "N__W" => db_pipe::N__W,
                "_SE_" => db_pipe::_SE_,
                "__EW" => db_pipe::__EW,
                "_S_W" => db_pipe::_S_W,
                _ => panic!("Invalid argument for wall function")
            }
        }
    }

    pub fn to_box(self) -> Box<Self> {
        Box::new(self)
    }
}

impl MapElement for Wall {
    fn can_step_on(&self) -> bool {
        false
    }

    fn interact_short(&mut self) -> String {
        (String::from("You shouldn't be here...")
    }

    fn interact_long(&mut self) -> String {
        String::from("Wall is immuable")
    }

    fn get_char(&self) -> char {
        self.sprite
    }

    fn get_color(&self) -> Color {
        WALLS_CLR
    }

    fn get_info(&self) -> Option<[String; 9]> {
        Some([
            String::from(" A wall.            "),
            String::from("                    "),
            String::from("                    "),
            String::from("                    "),
            String::from("                    "),
            String::from("                    "),
            String::from("                    "),
            String::from("                    "),
            String::from("                    ")
        ])
    }
}