use super::Point;
use std::convert::TryInto;
use super::super::graphics::chars::*;

pub struct Obstacle {
    pub pos: Point,
    pub sprite: char,
    pub info: [String; 9],
}

impl Obstacle {
    pub fn single(i: u16, j: u16) -> Obstacle {
        Obstacle {
            pos: Point { x: i.try_into().unwrap(), y: j.try_into().unwrap() },
            sprite: '\u{25A0}',
            info: 
                [
                    String::from(" A U+25A0 character "),
                    String::from("that seems to have  "),
                    String::from("arrived there by    "),
                    String::from("mistake             "),
                    String::from("                    "),
                    String::from("                    "),
                    String::from("                    "),
                    String::from("                    "),
                    String::from("                    ")
                ]
        }
    }

    pub fn wall(s: &str, i: u16, j: u16) -> Obstacle {
        let c = match s {
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
        };

        Obstacle {
            pos: Point { x: i.try_into().unwrap(), y: j.try_into().unwrap() },
            sprite: c,
            info: 
                [
                    String::from(" A wall             "),
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
}