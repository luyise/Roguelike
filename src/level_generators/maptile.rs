
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MapTile {
    Wall = 0,
    Empty = 1,
    DoorV = 2,
    DoorH = 3,
}