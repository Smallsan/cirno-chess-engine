use crate::types::PieceColor;
/**
 * Checks if a current piece has the same color
 */
pub fn is_color(piece_color: &PieceColor, other_piece_color: &PieceColor) -> bool {
    piece_color == other_piece_color
}

/**
 * Checks if a current piece has the opposite color
 */
pub fn is_opponent_color(piece_color: &PieceColor, other_piece_color: &PieceColor) -> bool {
    match piece_color {
        PieceColor::White => other_piece_color == &PieceColor::Black,
        PieceColor::Black => other_piece_color == &PieceColor::White,
        PieceColor::None => false,
    }
}

pub fn is_not_empty(piece_color: &PieceColor) -> bool {
    piece_color != &PieceColor::None
}
