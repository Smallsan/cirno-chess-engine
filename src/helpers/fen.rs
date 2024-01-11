use crate::types::{BoardPiece, ChessPieces, PieceColor};
use std::collections::HashMap;

/**
 * Should be used once only due to it reversing the board array.
 *
 * Doesn't support the entire FEN string yet. Castling doesn't work yet.
 */
pub fn load_position_from_fen(fen: String, board: &mut [BoardPiece; 64]) {
    let piece_type_from_symbol: HashMap<char, BoardPiece> = HashMap::from([
        ('r', (ChessPieces::Rooks, PieceColor::Black)),
        ('n', (ChessPieces::Knights, PieceColor::Black)),
        ('b', (ChessPieces::Bishops, PieceColor::Black)),
        ('q', (ChessPieces::Queens, PieceColor::Black)),
        ('k', (ChessPieces::Kings, PieceColor::Black)),
        ('p', (ChessPieces::Pawns, PieceColor::Black)),
        ('R', (ChessPieces::Rooks, PieceColor::White)),
        ('N', (ChessPieces::Knights, PieceColor::White)),
        ('B', (ChessPieces::Bishops, PieceColor::White)),
        ('Q', (ChessPieces::Queens, PieceColor::White)),
        ('K', (ChessPieces::Kings, PieceColor::White)),
        ('P', (ChessPieces::Pawns, PieceColor::White)),
    ]);

    let fen_board: Vec<String> = fen.split(" ").map(|str| str.to_string()).collect();
    let fen_notation = fen_board.get(0);
    let mut file = 0;
    let mut rank = 7;

    if let Some(fen_notation) = fen_notation {
        for letter in fen_notation.chars() {
            if letter == '/' {
                file = 0;
                rank -= 1;
            } else {
                if letter.is_digit(10) {
                    // movement | 8
                    file += letter.to_digit(10).unwrap();
                } else {
                    let piece_type = piece_type_from_symbol.get(&letter).unwrap();
                    let index = usize::try_from(rank * 8 + file).unwrap();
                    board[index] = piece_type.to_owned();
                    file += 1;
                }
            }
        }
    }
}
