use crate::piece::PieceType;
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

