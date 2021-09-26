use macroquad::prelude::*;

use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Color {
    Black,
    White
}

impl Color {
    pub fn to_color(&self) -> macroquad::color::Color {
        match self {
            Self::Black => macroquad::color::Color::from_rgba(209, 139, 71 ,255),
            Self::White => macroquad::color::Color::from_rgba(255, 206, 158 ,255),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Piece {
    pub color: Color,
    pub piece_type: PieceType
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Piece {
        return Piece { color, piece_type };
    }
}

pub struct Cell {
    pub color: Color,
    pub piece: Option<Piece>,
    pub active: bool
}

impl Cell {
    pub fn new(color: Color, piece: Option<Piece>) -> Cell {
        return Cell {
            color, piece, active: false
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

    pub fn on_click(&mut self, i: usize, j: usize) {
        if self.active == Some((i, j)) {
            self.active = None;
            self.cells[i][j].active = false;
        } else {
            if let Some((a, b)) = self.active {
                self.cells[a][b].active = false;
            }
            self.active = Some((i, j));
            self.cells[i][j].active = true;
        }
        

    }

}