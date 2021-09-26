use macroquad::prelude::*;


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

pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

pub struct Piece {
    pub color: Color,
    pub pieceType: PieceType
}

pub struct Cell {
    pub color: Color,
    pub piece: Option<Piece>
}

pub struct State {
    pub cells: Vec<Vec<Cell>>
}

impl State {
    pub fn new() -> State {

        let mut row8 = Vec::<Cell>::new();
        row8.push(Cell {color: Color::White, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Rook }) });
        row8.push(Cell {color: Color::Black, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Knight }) });
        row8.push(Cell {color: Color::White, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Bishop })});
        row8.push(Cell {color: Color::Black, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Queen }) });
        row8.push(Cell {color: Color::White, piece: Some(Piece { color: Color::Black, pieceType: PieceType::King }) });
        row8.push(Cell {color: Color::Black, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Bishop }) });
        row8.push(Cell {color: Color::White, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Knight }) });
        row8.push(Cell {color: Color::Black, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Rook }) });
        
        let mut row7 = Vec::<Cell>::new();
        row7.push(Cell {color: Color::Black, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Pawn }) });
        row7.push(Cell {color: Color::White, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Pawn })});
        row7.push(Cell {color: Color::Black, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Pawn }) });
        row7.push(Cell {color: Color::White, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Pawn }) });
        row7.push(Cell {color: Color::Black, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Pawn }) });
        row7.push(Cell {color: Color::White, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Pawn }) });
        row7.push(Cell {color: Color::Black, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Pawn }) });
        row7.push(Cell {color: Color::White, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Pawn }) });

        let mut row6 = Vec::<Cell>::new();
        row6.push(Cell {color: Color::White, piece: None});
        row6.push(Cell {color: Color::Black, piece: None});
        row6.push(Cell {color: Color::White, piece: None});
        row6.push(Cell {color: Color::Black, piece: None});
        row6.push(Cell {color: Color::White, piece: None});
        row6.push(Cell {color: Color::Black, piece: None});
        row6.push(Cell {color: Color::White, piece: None});
        row6.push(Cell {color: Color::Black, piece: None});

        let mut row5 = Vec::<Cell>::new();
        row5.push(Cell {color: Color::Black, piece: None});
        row5.push(Cell {color: Color::White, piece: None});
        row5.push(Cell {color: Color::Black, piece: None});
        row5.push(Cell {color: Color::White, piece: None});
        row5.push(Cell {color: Color::Black, piece: None});
        row5.push(Cell {color: Color::White, piece: None});
        row5.push(Cell {color: Color::Black, piece: None});
        row5.push(Cell {color: Color::White, piece: None});

        let mut row4 = Vec::<Cell>::new();
        row4.push(Cell {color: Color::White, piece: None});
        row4.push(Cell {color: Color::Black, piece: None});
        row4.push(Cell {color: Color::White, piece: None});
        row4.push(Cell {color: Color::Black, piece: None});
        row4.push(Cell {color: Color::White, piece: None});
        row4.push(Cell {color: Color::Black, piece: None});
        row4.push(Cell {color: Color::White, piece: None});
        row4.push(Cell {color: Color::Black, piece: None});

        let mut row3 = Vec::<Cell>::new();
        row3.push(Cell {color: Color::Black, piece: None});
        row3.push(Cell {color: Color::White, piece: None});
        row3.push(Cell {color: Color::Black, piece: None});
        row3.push(Cell {color: Color::White, piece: None});
        row3.push(Cell {color: Color::Black, piece: None});
        row3.push(Cell {color: Color::White, piece: None});
        row3.push(Cell {color: Color::Black, piece: None});
        row3.push(Cell {color: Color::White, piece: None});

        let mut row2 = Vec::<Cell>::new();
        row2.push(Cell {color: Color::White, piece: Some(Piece { color: Color::White, pieceType: PieceType::Pawn })});
        row2.push(Cell {color: Color::Black, piece: Some(Piece { color: Color::White, pieceType: PieceType::Pawn }) });
        row2.push(Cell {color: Color::White, piece: Some(Piece { color: Color::White, pieceType: PieceType::Pawn }) });
        row2.push(Cell {color: Color::Black, piece: Some(Piece { color: Color::White, pieceType: PieceType::Pawn }) });
        row2.push(Cell {color: Color::White, piece: Some(Piece { color: Color::White, pieceType: PieceType::Pawn }) });
        row2.push(Cell {color: Color::Black, piece: Some(Piece { color: Color::White, pieceType: PieceType::Pawn }) });
        row2.push(Cell {color: Color::White, piece: Some(Piece { color: Color::White, pieceType: PieceType::Pawn }) });
        row2.push(Cell {color: Color::Black, piece: Some(Piece { color: Color::White, pieceType: PieceType::Pawn }) });

        let mut row1 = Vec::<Cell>::new();
        row1.push(Cell {color: Color::Black, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Rook }) });
        row1.push(Cell {color: Color::White, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Knight }) });
        row1.push(Cell {color: Color::Black, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Bishop })});
        row1.push(Cell {color: Color::White, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Queen }) });
        row1.push(Cell {color: Color::Black, piece: Some(Piece { color: Color::Black, pieceType: PieceType::King }) });
        row1.push(Cell {color: Color::White, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Bishop }) });
        row1.push(Cell {color: Color::Black, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Knight }) });
        row1.push(Cell {color: Color::White, piece: Some(Piece { color: Color::Black, pieceType: PieceType::Rook }) });

        let mut cells = Vec::<Vec<Cell>>::new();

        cells.push(row8);
        cells.push(row7);
        cells.push(row6);
        cells.push(row5);
        cells.push(row4);
        cells.push(row3);
        cells.push(row2);
        cells.push(row1);

        return State { cells };
    }


}