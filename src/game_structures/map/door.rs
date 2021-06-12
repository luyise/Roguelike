use super::MapElement;


#[derive(Debug, Copy, Clone)]
pub struct Door {
    opened: bool,
    image: char,
}

impl Door {
    pub fn vertical() -> Self {
        Self {
            opened: false,
            image: '|',
        }
    }

    pub fn horizontal() -> Self {
        Self {
            opened: false,
            image: '-',
        }
    }
}

impl MapElement for Door {
    fn can_step_on(&self) -> bool {
        self.opened
    }

    fn interact_short(&mut self) {
        panic!("not implemented")
    }

    fn interact_long(&mut self) {
        self.opened = !self.opened
    }

    fn get_char(&self) -> char {
        if self.opened {
            ' '
        } else {
            self.image
        }
    }

    fn get_info(&self) -> [String; 9] {
        if self.opened {
            [
                String::new(), String::new(), String::new(),
                String::new(), String::new(), String::new(),
                String::new(), String::new(), String::new()
            ]
        } else {
            [
                String::from(" A closed door, it  "),
                String::from("doesn't seems to be "),
                String::from("locked.             "),
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