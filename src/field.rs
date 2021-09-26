use macroquad::prelude::*;

use crate::game::Game;

pub struct Field {
    
}

impl Field {
    pub fn new() -> Field {
        return Field {};
    }
    pub fn render(&self, game: &Game) {
        clear_background(WHITE);
    }
}