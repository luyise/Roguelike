use super::Point;
use crate::colors::*;
use crate::graphics::chars::*;
use crossterm::style::Color;
use std::convert::TryInto;
use std::fs::File;
use std::io::Write;

pub struct Obstacle {
    pub color: Color,
    pub state: String,
    pub pos: Point,
    pub sprite: char,
    pub info: [String; 9],
}

impl Obstacle {
    pub fn single(i: u16, j: u16) -> Obstacle {
        Obstacle {
            color: OBSTACLES_CLR,
            state: String::new(),
            pos: Point {
                x: i.try_into().unwrap(),
                y: j.try_into().unwrap(),
            },
            sprite: '\u{25A0}',
            info: [
                String::from(" A U+25A0 character "),
                String::from("that seems to have  "),
                String::from("arrived there by    "),
                String::from("mistake.            "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    "),
            ],
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
            _ => panic!("Invalid argument for wall function"),
        };

        Obstacle {
            color: WALLS_CLR,
            state: String::new(),
            pos: Point {
                x: i.try_into().unwrap(),
                y: j.try_into().unwrap(),
            },
            sprite: c,
            info: [
                String::from(" A wall.            "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    "),
            ],
        }
    }

    pub fn door(s: &str, i: u16, j: u16) -> Obstacle {
        let c = match &*s.to_lowercase() {
            "v" | "ver" | "vert" => sg_pipe::NS__,
            "h" | "hor" | "hori" => sg_pipe::__EW,
            _ => panic!("Invalid argument for door function"),
        };

        Obstacle {
            color: DOORS_CLR,
            state: String::from("closed"),
            pos: Point {
                x: i.try_into().unwrap(),
                y: j.try_into().unwrap(),
            },
            sprite: c,
            info: [
                String::from(" A closed door, it  "),
                String::from("doesn't seems to be "),
                String::from("locked.             "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    "),
                String::from("                    "),
            ],
        }
    }

    pub fn save(&self, f: &mut File) -> std::io::Result<usize> {
        f.write(b"\tstate: \"")?;
        f.write(self.state.as_bytes())?;
        f.write(b"\"\n\tpos:\n")?;
        self.pos.save(f)?;
        let mut s = String::from("\tsprite: ");
        s.push(self.sprite);
        s.push_str("\n");
        f.write(s.as_bytes())?;
        f.write(b"\t info: [\n")?;
        for i in self.info.iter() {
            f.write(b"\t\t\"")?;
            f.write(i.as_bytes())?;
            f.write(b"\",\n")?;
        }
        f.write(b"\t]\n")
    }
}
