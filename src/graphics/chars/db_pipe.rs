pub const NSEW: char = '\u{256C}'; // ╬   box drawings double vertical and horizontal

pub const _SEW: char = '\u{2566}'; // ╦   box drawings double down and horizontal
pub const N_EW: char = '\u{2569}'; // ╩   box drawings double up and horizontal
pub const NS_W: char = '\u{2563}'; // ╣
pub const NSE_: char = '\u{2560}'; // ╠   box drawings double vertical and right

pub const NS__: char = '\u{2551}'; // ║
pub const N_E_: char = '\u{255A}'; // ╚   box drawings double up and right
pub const N__W: char = '\u{255D}'; // ╝   box drawings double up and left
pub const _SE_: char = '\u{2554}'; // ╔   box drawings double down and right
pub const __EW: char = '\u{2550}'; // ═   box drawings double horizontal
pub const _S_W: char = '\u{2557}'; // ╗   box drawings double down and left

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
    ____, N___, _S__, NS__,
    __E_, N_E_, _SE_, NSE_,
    ___W, N__W, _S_W, NS_W,
    __EW, N_EW, _SEW, NSEW
];