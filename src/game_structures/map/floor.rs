use super::MapElement;


#[derive(Debug, Copy, Clone)]
pub struct Floor {
    image: char,
}

impl Floor {
    pub fn new() -> Self {
        Self {
            image: ' '
        }
    }
}

impl MapElement for Floor {
    fn can_step_on(&self) -> bool {
        true
    }

    fn interact_short(&mut self) {
        panic!("not implemented")
    }

    fn interact_long(&mut self) {
        panic!("not implemented")
    }

    fn get_char(&self) -> char {
        self.image
    }
}