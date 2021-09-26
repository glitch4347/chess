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

        let width = game.width();
        let height = game.height();

        let r_width = width / game.state.cells[0].len() as f32;
        let r_heigh = height / game.state.cells.len() as f32;

        for i in 0..game.state.cells.len() {
            for j in 0..game.state.cells[i].len() {
                let color = &game.state.cells[i][j].color;
                let x = r_width * i as f32;
                let y = r_heigh * j as f32;
                draw_rectangle(x, y, r_width, r_heigh, color.to_color());
            }
        }
        
    }
}