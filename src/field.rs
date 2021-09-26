use macroquad::prelude::*;

use tuple_conv::*;


use std::collections::HashMap;

use crate::game::Game;
use crate::state::{ Cell, Piece, Color, PieceType };

pub struct PiecesTextures {
    pub map: HashMap<Piece, Texture2D>
}

impl PiecesTextures {

    async fn new() -> anyhow::Result<PiecesTextures> {
        let map = HashMap::new();
        let mut res = PiecesTextures { map };
        res.load_textures().await?;
        return Ok(res);
    }

    async fn load_textures(&mut self) -> anyhow::Result<()> {

        let res = futures::join!(
            self.load_piece_texture(Color::White, PieceType::Pawn),
            self.load_piece_texture(Color::White, PieceType::Bishop),
            self.load_piece_texture(Color::White, PieceType::King),
            self.load_piece_texture(Color::White, PieceType::Knight),
            self.load_piece_texture(Color::White, PieceType::Queen),
            self.load_piece_texture(Color::White, PieceType::Rook),
            self.load_piece_texture(Color::Black, PieceType::Pawn),
            self.load_piece_texture(Color::Black, PieceType::Bishop),
            self.load_piece_texture(Color::Black, PieceType::King),
            self.load_piece_texture(Color::Black, PieceType::Knight),
            self.load_piece_texture(Color::Black, PieceType::Queen),
            self.load_piece_texture(Color::Black, PieceType::Rook)
        );

        let r:Vec<anyhow::Result<(Piece, Texture2D)>> = res.to_vec();
        for k in r {
            let kk = k.unwrap();
            self.map.insert(kk.0, kk.1);
        }

        return Ok(());
        
        
    }

    async fn load_piece_texture(&self, color: Color, piece_type: PieceType) -> anyhow::Result<(Piece, Texture2D)> {
        let path = format!(
            "textures/{}_{}.png", 
            piece_type.to_string().to_lowercase(),
            color.to_string().to_lowercase()
        );
        let t = load_texture(path.as_str()).await?;
        return Ok((Piece::new(color, piece_type), t));
    }
}

pub struct Field {
    pieces_textures: PiecesTextures
}

impl Field {
    pub async fn new() -> anyhow::Result<Field> {
        return Ok(Field { pieces_textures: PiecesTextures::new().await? });
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