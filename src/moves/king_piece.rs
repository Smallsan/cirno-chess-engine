
use crate::types::{BoardPiece, ChessPieces, Move};
use crate::color::is_opponent_color;

/**
 * King = Not done: Castling
 */
pub fn generate_king_moves(start_square: usize, board: &[BoardPiece; 64], moves: &mut Vec<Move>, attacked_squares: &mut Vec<i16>) {
    let king_moves = [
        (-1, -1), (-1, 0), (-1, 1), (0, -1),
        (0, 1), (1, -1), (1, 0), (1, 1),
    ];

    let (start_rank, start_file) = (start_square / 8, start_square % 8);

    for (rank_offset, file_offset) in king_moves {
        let new_rank = start_rank as i16 + rank_offset;
        let new_file = start_file as i16 + file_offset;

        if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
            let target_square = (new_rank * 8 + new_file) as i16;
            let target_piece = board[target_square as usize];

            if target_piece.0 == ChessPieces::Empty {
                continue;
            }

            let movement = Move {
                start_square: start_square as i16,
                target_square,
            };
            moves.push(movement);

            if is_opponent_color(&target_piece.1, &board[start_square].1) {
                attacked_squares.push(target_square);
                continue;
            }
        }
    }
}
