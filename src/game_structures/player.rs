use super::Point;

pub struct Player {
    pub pos: Point,
    pub sprite: char,
}
impl Player {
    pub fn new() -> Player {
        Player {
            pos: Point { x: 0, y: 0 },
            sprite: '@',
        }
    }
}
