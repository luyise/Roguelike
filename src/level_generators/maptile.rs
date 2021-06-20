
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MapTile {
    Wall = 0,
    Empty = 1,
    DoorV = 2,
    DoorH = 3,
    DoorD1 = 4,
    DoorD2 = 5,
    Door = 6,
}

pub fn get_char(tile: MapTile) -> char {
    match tile {
        MapTile::Wall => '\u{2593}',
        MapTile::Empty => ' ',
        MapTile::DoorV => '|',
        MapTile::DoorH => '-',
        MapTile::DoorD1 => '\\',
        MapTile::DoorD2 => '/',
        MapTile::Door => '+',
    }
}