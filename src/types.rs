
pub type Chess = (ChessPieces, PieceColor);

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ChessPieces {
    Kings,
    Queens,
    Rooks,
    Bishops,
    Knights,
    Pawns,

    #[default]
    Empty,
}

pub type SquaresToEdge = [[i16; 8]; 64];

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum PieceColor {
    White, Black, 
    #[default]
    None
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub start_square: i16,
    pub target_square: i16,
}

pub struct ChessState {
    pub board: [Chess; 64],
    pub color_to_move: PieceColor,
}

impl Default for ChessState {
    fn default() -> ChessState {
        ChessState {
            board: [(ChessPieces::Empty, PieceColor::None); 64],
            color_to_move: PieceColor::Black,
        }
    }
}
