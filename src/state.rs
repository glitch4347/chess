
enum Color {
    Black,
    White
}

enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

struct Piece {
    color: Color,
    t: PieceType
}

struct Cell {
    color: Color,
    piece: Option<Piece>
}

pub struct State {
    cells: Vec<Vec<Cell>>
}

impl State {
    pub fn new() -> State {

        let row = Vec::<Cell>::new();

        let mut cells = Vec::<Vec<Cell>>::new();

        
        cells.push(row);

        return State {
            cells: cells
        };
    }
}