pub const NSEW: char = '\u{253C}'; // ┼   box drawings light vertical and horizontal

pub const NS_W: char = '\u{2524}'; // ┤
pub const NSE_: char = '\u{251C}'; // ├   box drawings light vertical and right
pub const N_EW: char = '\u{2534}'; // ┴   box drawings light up and horizontal
pub const _SEW: char = '\u{252C}'; // ┬   box drawings light down and horizontal

pub const NS__: char = '\u{2502}'; // |
pub const N_E_: char = '\u{2514}'; // └   box drawings light up and right
pub const N__W: char = '\u{2518}';
pub const _SE_: char = '\u{250C}';
pub const _S_W: char = '\u{2510}'; // ┐   box drawings light down and left
pub const __EW: char = '\u{2500}'; // ─   box drawings light horizontal

pub const N___: char = ' ';
pub const _S__: char = ' ';
pub const __E_: char = ' ';
pub const ___W: char = ' ';

pub const ____: char = ' ';

/*
const N  : u8 = 1 << 0;
const S  : u8 = 1 << 1;
const  E : u8 = 1 << 2;
const  W : u8 = 1 << 3;
*/

pub const TABLE: [char; 16] = [
    ____, N___, _S__, NS__, __E_, N_E_, _SE_, NSE_, ___W, N__W, _S_W, NS_W, __EW, N_EW, _SEW, NSEW,
];
