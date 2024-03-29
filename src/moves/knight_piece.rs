use crate::helpers::color::*;
use crate::types::{BoardPiece, Move, MoveType};

pub fn generate_knight_moves(start_square: usize, board: &[BoardPiece; 64], moves: &mut Vec<Move>) {
    let knight_moves = [
        (-1, -2),
        (-2, -1),
        (-2, 1),
        (-1, 2),
        (1, -2),
        (2, -1),
        (2, 1),
        (1, 2),
    ];
    let (start_rank, start_file) = (start_square / 8, start_square % 8);

    for (file_offset, rank_offset) in knight_moves {
        let new_rank = start_rank as i16 + rank_offset;
        let new_file = start_file as i16 + file_offset;

        if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
            let target_square = new_rank * 8 + new_file;
            let target_piece = board[target_square as usize];

            if is_color(&target_piece.piece_color, &board[start_square].piece_color) {
                continue;
            }

            if is_opponent_color(&target_piece.piece_color, &board[start_square].piece_color) {
                let movement = Move {
                    start_square: start_square as i16,
                    target_square,
                    move_type: MoveType::Normal,
                };
                moves.push(movement);
                continue;
            }

            let movement = Move {
                start_square: start_square as i16,
                target_square: target_square as i16,
                move_type: MoveType::Normal,
            };
            moves.push(movement);
        }
    }
}
