use crate::field::{self, Field};

pub struct Game {
    field: Field
}

impl Game {
    pub fn new() -> Game {
        return Game {
            field: Field::new()
        };
    }

    pub fn handle_keys(&self) {

    }

    pub fn render(&self) {
        self.field.render(&self);
    }

    pub fn width() {
        return 
    }
}

