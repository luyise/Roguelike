use super::Point;
use std::fs::File;
use std::io::Write;

pub struct Player {
    pub pos: Point,
    pub sprite: char,
}
impl Player {
    pub fn new() -> Player {
        Player {
            pos: Point { x: 4, y: 5 },
            sprite: '@',
        }
    }

    pub fn save(self, f: &mut File) -> std::io::Result<usize> {
        f.write(b"Player {\n\t pos:\n")?;
        self.pos.save(f)?;
        let mut s = String::from("\t sprite: '");
        s.push(self.sprite);
        s.push_str("'\n}\n");
        f.write(s.as_bytes())
    }
}
