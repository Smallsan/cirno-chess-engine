
use crate::chess_state::ChessState;
use crate::types::{BoardPiece, Castle, ChessPieces, PieceColor};

/**
 * Doesn't support the entire FEN string yet.
 * Halfmoves (essential for tracking stalemates) don't work yet.
 */
pub fn save_fen_state(state: ChessState) -> Result<String, &'static str> {
    Ok("".to_string())
}
