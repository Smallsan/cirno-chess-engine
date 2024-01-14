pub type SquaresToEdge = [[i16; 8]; 64];

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BoardPiece {
    pub piece_type: ChessPieces,
    pub piece_color: PieceColor,
}

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

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum PieceColor {
    White,
    Black,

    #[default]
    None,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum MoveType {
    Castle,
    NoCapture, // pawns can't capture forwards, only diagonal forwards.
    // this enum describes this rule pawns have
    EnPassant,
    Promotion,

    Piercing, // useful for deteching pins.
    // it can go through 2 pieces.
    #[default]
    Normal, // piece can capture.
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub start_square: i16,
    pub target_square: i16,
    pub move_type: MoveType,
}

pub struct ChessState {
    pub board: [BoardPiece; 64],
    pub color_to_move: PieceColor,
    pub is_able_to_castle: Castle,
    pub pinned_pieces: Vec<BoardPiece>,
}

#[derive(Debug, Default)]
pub struct Castle {
    pub queenside: bool,
    pub kingside: bool,
}

impl Default for ChessState {
    fn default() -> ChessState {
        ChessState {
            board: [BoardPiece {
                ..Default::default()
            }; 64],
            color_to_move: PieceColor::Black,
            is_able_to_castle: Default::default(),
            pinned_pieces: vec![],
        }
    }
}
