use macroquad::prelude::*;

use crate::piece::{Piece, PieceType};
use crate::types::Color;


pub struct Cell {
    pub color: Color,
    pub piece: Option<Piece>,
    pub active: bool,
    pub selected_move: bool
}

impl Cell {
    pub fn new(color: Color, piece: Option<Piece>) -> Cell {
        return Cell {
            color, piece, active: false, selected_move: false
        }
    }
}

pub struct State {
    pub cells: Vec<Vec<Cell>>,
    active: Option<(usize, usize)>
}

impl State {
    pub fn new() -> State {

        let mut row8 = Vec::<Cell>::new();
        row8.push(Cell::new(Color::White, Some(Piece { color: Color::Black, piece_type: PieceType::Rook })));
        row8.push(Cell::new(Color::Black, Some(Piece { color: Color::Black, piece_type: PieceType::Knight }) ));
        row8.push(Cell::new(Color::White, Some(Piece { color: Color::Black, piece_type: PieceType::Bishop }) ));
        row8.push(Cell::new(Color::Black, Some(Piece { color: Color::Black, piece_type: PieceType::Queen }) ));
        row8.push(Cell::new(Color::White, Some(Piece { color: Color::Black, piece_type: PieceType::King }) ));
        row8.push(Cell::new(Color::Black, Some(Piece { color: Color::Black, piece_type: PieceType::Bishop }) ));
        row8.push(Cell::new(Color::White, Some(Piece { color: Color::Black, piece_type: PieceType::Knight }) ));
        row8.push(Cell::new(Color::Black, Some(Piece { color: Color::Black, piece_type: PieceType::Rook }) ));
        
        let mut row7 = Vec::<Cell>::new();
        row7.push(Cell::new(Color::Black, Some(Piece { color: Color::Black, piece_type: PieceType::Pawn }) ));
        row7.push(Cell::new(Color::White, Some(Piece { color: Color::Black, piece_type: PieceType::Pawn }) ));
        row7.push(Cell::new(Color::Black, Some(Piece { color: Color::Black, piece_type: PieceType::Pawn }) ));
        row7.push(Cell::new(Color::White, Some(Piece { color: Color::Black, piece_type: PieceType::Pawn }) ));
        row7.push(Cell::new(Color::Black, Some(Piece { color: Color::Black, piece_type: PieceType::Pawn }) ));
        row7.push(Cell::new(Color::White, Some(Piece { color: Color::Black, piece_type: PieceType::Pawn }) ));
        row7.push(Cell::new(Color::Black, Some(Piece { color: Color::Black, piece_type: PieceType::Pawn }) ));
        row7.push(Cell::new(Color::White, Some(Piece { color: Color::Black, piece_type: PieceType::Pawn }) ));

        let mut row6 = Vec::<Cell>::new();
        row6.push(Cell::new(Color::White, None));
        row6.push(Cell::new(Color::Black, None));
        row6.push(Cell::new(Color::White, None));
        row6.push(Cell::new(Color::Black, None));
        row6.push(Cell::new(Color::White, None));
        row6.push(Cell::new(Color::Black, None));
        row6.push(Cell::new(Color::White, None));
        row6.push(Cell::new(Color::Black, None));

        let mut row5 = Vec::<Cell>::new();
        row5.push(Cell::new(Color::Black, None));
        row5.push(Cell::new(Color::White, None));
        row5.push(Cell::new(Color::Black, None));
        row5.push(Cell::new(Color::White, None));
        row5.push(Cell::new(Color::Black, None));
        row5.push(Cell::new(Color::White, None));
        row5.push(Cell::new(Color::Black, None));
        row5.push(Cell::new(Color::White, None));

        let mut row4 = Vec::<Cell>::new();
        row4.push(Cell::new(Color::White, None));
        row4.push(Cell::new(Color::Black, None));
        row4.push(Cell::new(Color::White, None));
        row4.push(Cell::new(Color::Black, None));
        row4.push(Cell::new(Color::White, None));
        row4.push(Cell::new(Color::Black, None));
        row4.push(Cell::new(Color::White, None));
        row4.push(Cell::new(Color::Black, None));

        let mut row3 = Vec::<Cell>::new();
        row3.push(Cell::new(Color::Black, None));
        row3.push(Cell::new(Color::White, None));
        row3.push(Cell::new(Color::Black, None));
        row3.push(Cell::new(Color::White, None));
        row3.push(Cell::new(Color::Black, None));
        row3.push(Cell::new(Color::White, None));
        row3.push(Cell::new(Color::Black, None));
        row3.push(Cell::new(Color::White, None));

        let mut row2 = Vec::<Cell>::new();
        row2.push(Cell::new(Color::White, Some(Piece { color: Color::White, piece_type: PieceType::Pawn })));
        row2.push(Cell::new(Color::Black, Some(Piece { color: Color::White, piece_type: PieceType::Pawn }) ));
        row2.push(Cell::new(Color::White, Some(Piece { color: Color::White, piece_type: PieceType::Pawn }) ));
        row2.push(Cell::new(Color::Black, Some(Piece { color: Color::White, piece_type: PieceType::Pawn }) ));
        row2.push(Cell::new(Color::White, Some(Piece { color: Color::White, piece_type: PieceType::Pawn }) ));
        row2.push(Cell::new(Color::Black, Some(Piece { color: Color::White, piece_type: PieceType::Pawn }) ));
        row2.push(Cell::new(Color::White, Some(Piece { color: Color::White, piece_type: PieceType::Pawn }) ));
        row2.push(Cell::new(Color::Black, Some(Piece { color: Color::White, piece_type: PieceType::Pawn }) ));

        let mut row1 = Vec::<Cell>::new();
        row1.push(Cell::new(Color::Black, Some(Piece { color: Color::White, piece_type: PieceType::Rook }) ));
        row1.push(Cell::new(Color::White, Some(Piece { color: Color::White, piece_type: PieceType::Knight })));
        row1.push(Cell::new(Color::Black, Some(Piece { color: Color::White, piece_type: PieceType::Bishop })));
        row1.push(Cell::new(Color::White, Some(Piece { color: Color::White, piece_type: PieceType::Queen }) ));
        row1.push(Cell::new(Color::Black, Some(Piece { color: Color::White, piece_type: PieceType::King }) ));
        row1.push(Cell::new(Color::White, Some(Piece { color: Color::White, piece_type: PieceType::Bishop }) ));
        row1.push(Cell::new(Color::Black, Some(Piece { color: Color::White, piece_type: PieceType::Knight }) ));
        row1.push(Cell::new(Color::White, Some(Piece { color: Color::White, piece_type: PieceType::Rook }) ));

        let mut cells = Vec::<Vec<Cell>>::new();

        cells.push(row8);
        cells.push(row7);
        cells.push(row6);
        cells.push(row5);
        cells.push(row4);
        cells.push(row3);
        cells.push(row2);
        cells.push(row1);

        return State { cells, active: None };
    }

    fn try_to_select_for_move(&mut self, i: usize, j: usize) -> bool {
        if i < 0 || j < 0 {
            return false;
        }
        if let Some(_) = self.cells[i][j].piece {
            return false;
        }
        self.cells[i][j].selected_move = true;
        return true;
    }

    fn set_selected_moves_for_piece(&mut self, i: usize, j: usize) {
        let piece = self.cells[i][j].piece;
        if piece.is_none() {
            return;
        }
        let piece = piece.unwrap();
        let moves = piece.get_possible_moves(i, j);
        for m in moves {
            self.cells[m.0][m.1].selected_move = true;
        }
    }

    pub fn on_click(&mut self, i: usize, j: usize) {
        // TODO: do nothing if ther eis no figure
        if self.active == Some((i, j)) {
            self.active = None;
            self.cells[i][j].active = false;

        } else {
            if let Some((a, b)) = self.active {
                self.cells[a][b].active = false;
            }
            self.active = Some((i, j));
            self.cells[i][j].active = true;
            self.set_selected_moves_for_piece(i, j);
        }
    }

}