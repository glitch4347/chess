use macroquad::prelude::*;
use crate::field::{self, Field};
use crate::state::State;

pub struct Game {
    field: Field,
    pub state: State
}

impl Game {
    pub fn new() -> Game {
        return Game {
            field: Field::new(),
            state: State::new()
        };
    }

    pub fn handle_keys(&self) {

    }

    pub fn render(&self) {
        self.field.render(&self);
    }

    pub fn width(&self) -> f32 {
        return screen_width();
    }
    pub fn height(&self) -> f32 {
        return screen_height();
    }
}

