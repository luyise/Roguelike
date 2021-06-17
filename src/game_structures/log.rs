use crate::array;
use crate::options::*;
use crossterm::style::Color;

pub struct Log {
    pub colors: [Color; DISP_HEIGHT as usize],
    pub messages: [String; DISP_HEIGHT as usize],
}

impl Log {
    pub fn new() -> Log {
        assert_eq!(DISP_HEIGHT, 15);
        Log {
            colors: array![Color::White; 15],
            messages: array![String::new(); 15],
        }
    }

    fn scroll(&mut self) {
        for i in 0..(DISP_HEIGHT - 1) as usize {
            self.messages[i] = self.messages[i + 1].clone();
            self.colors[i] = self.colors[i + 1].clone()
        }
    }

    pub fn push(&mut self, m: String, clr: Color) {
        self.scroll();
        self.messages[(DISP_HEIGHT - 1) as usize] = m;
        self.colors[(DISP_HEIGHT - 1) as usize] = clr
    }

    pub fn clear(&mut self) {
        for i in 0..DISP_HEIGHT as usize {
            self.messages[i] = String::new();
            self.colors[i] = Color::White
        }
    }
}
