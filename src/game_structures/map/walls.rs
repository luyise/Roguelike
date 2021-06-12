use crossterm::style::Color;
use super::MapElement;
use crate::colors::*;


#[derive(Debug, Copy, Clone)]
pub struct Wall {
    sprite: char,
}

impl Wall {
    pub fn new(s: &str) -> self {
        self {
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
}

impl MapElement for Floor {
    fn can_step_on(&self) -> bool {
        false
    }

    fn interact_short(&mut self) {
        panic!("not implemented")
    }

    fn interact_long(&mut self) {
        panic!("not implemented")
    }

    fn get_char(&self) -> char {
        self.sprite
    }

    fn get_color(&self) -> Color {
        WALLS_CLR
    }

    fn get_info(&self) -> [String; 9] {
        [
            String::from(" A wall.            "),
            String::from("                    "),
            String::from("                    "),
            String::from("                    "),
            String::from("                    "),
            String::from("                    "),
            String::from("                    "),
            String::from("                    "),
            String::from("                    ")
        ]
    }
}