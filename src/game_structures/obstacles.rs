use super::Point;
use std::convert::TryInto;

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
}