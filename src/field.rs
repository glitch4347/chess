use macroquad::prelude::*;

use std::collections::HashMap;

use crate::game::Game;
use crate::state::{ Cell, Piece, Color, PieceType };

pub struct PiecesTextures {
    pub map: HashMap<Piece, Texture2D>
}

impl PiecesTextures {

    async fn new() -> PiecesTextures {
        let mut map = HashMap::new();

        map.insert(
            Piece::new(Color::White, PieceType::Pawn),
            load_texture("textures/pawn_white.png").await.unwrap()
        );
        map.insert(
            Piece::new(Color::White, PieceType::Bishop),
            load_texture("textures/bishop_white.png").await.unwrap()
        );
        map.insert(
            Piece::new(Color::White, PieceType::King),
            load_texture("textures/king_white.png").await.unwrap()
        );
        map.insert(
            Piece::new(Color::White, PieceType::Knight),
            load_texture("textures/knight_white.png").await.unwrap()
        );
        map.insert(
            Piece::new(Color::White, PieceType::Queen),
            load_texture("textures/queen_white.png").await.unwrap()
        );
        map.insert(
            Piece::new(Color::White, PieceType::Rook),
            load_texture("textures/rook_white.png").await.unwrap()
        );

        map.insert(
            Piece::new(Color::Black, PieceType::Pawn),
            load_texture("textures/pawn_black.png").await.unwrap()
        );
        map.insert(
            Piece::new(Color::Black, PieceType::Bishop),
            load_texture("textures/bishop_black.png").await.unwrap()
        );
        map.insert(
            Piece::new(Color::Black, PieceType::King),
            load_texture("textures/king_black.png").await.unwrap()
        );
        map.insert(
            Piece::new(Color::Black, PieceType::Knight),
            load_texture("textures/knight_black.png").await.unwrap()
        );
        map.insert(
            Piece::new(Color::Black, PieceType::Queen),
            load_texture("textures/queen_black.png").await.unwrap()
        );
        map.insert(
            Piece::new(Color::Black, PieceType::Rook),
            load_texture("textures/rook_black.png").await.unwrap()
        );
        
        return PiecesTextures { map };
    }
}

pub struct Field {
    pieces_textures: PiecesTextures
}

impl Field {
    pub async fn new() -> Field {
        return Field { pieces_textures: PiecesTextures::new().await };
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

        if cell.piece.is_some() {
            // render texture
            let t = self.pieces_textures.map.get(&cell.piece.unwrap()).unwrap();
            
            let dp = DrawTextureParams {
                dest_size: Some(macroquad::math::Vec2::new(r_width, r_height)),
                source: None,
                rotation: 0.,
                pivot: None,
                flip_x: false,
                flip_y: false,
            };

            draw_texture_ex(*t, x, y, WHITE, dp);
        }
        
    }
}