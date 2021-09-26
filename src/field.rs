use macroquad::prelude::*;

use crate::game::Game;
use crate::state::{ Cell };

pub struct Field {
}

impl Field {
    pub fn new() -> Field {
        return Field { };
    }
    pub fn render(&self, game: &Game) {
        clear_background(WHITE);

        for i in 0..game.state.cells.len() {
            for j in 0..game.state.cells[i].len() {
                self.render_cell(&game, i as u32, j as u32, &game.state.cells[i][j]);
            }
        }
    }

    fn render_cell(&self, game: &Game, i: u32, j: u32, cell: &Cell) {

        let width = game.width();
        let height = game.height();

        let r_width = width / game.state.cells[0].len() as f32;
        let r_height = height / game.state.cells.len() as f32;

        let color = cell.color.to_color();

        let x = r_width * j as f32;
        let y = r_height * i as f32;
        draw_rectangle(x, y, r_width, r_height, color);
    }
}