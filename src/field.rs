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

        let rows = 8;
        let cols = 8;

        let width = game.width();
        let height = game.height();

        let r_width = width / rows as f32;
        let r_heigh = height / cols as f32;

        for i in 0..rows {
            for j in 0..cols {
                let color = if i % 2 != j % 2 { BLACK } else { WHITE };
                let x = r_width * i as f32;
                let y = r_heigh * j as f32;
                draw_rectangle(x, y, r_width, r_heigh, color);
            }
        }
        
    }
}