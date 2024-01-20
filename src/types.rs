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
    NoCapture,
    EnPassant,
    Promotion,

    Piercing, // useful for deteching pins.
    // it can go through 2 pieces.
    #[default]
    Normal, // piece can capture.
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Move {
    pub start_square: i16,
    pub target_square: i16,
    pub move_type: MoveType,
}

#[derive(Debug, Default, Clone)]
pub struct Castle {
    pub queenside: bool,
    pub kingside: bool,
}
