use std::fmt;
use crate::types::Color;


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
