
use crate::types::{BoardPiece, Move};
use crate::helpers::color::*;

pub fn generate_knight_moves(start_square: usize, board: &[BoardPiece; 64], moves: &mut Vec<Move>, attacked_squares: &mut Vec<i16>) {
    let knight_moves = [
        (-1, -2), (-2, -1), (-2, 1), (-1, 2),
        (1, -2), (2, -1), (2, 1), (1, 2),
    ];
    let (start_rank, start_file) = (start_square / 8, start_square % 8);

    for (file_offset, rank_offset) in knight_moves {
        let new_rank = start_rank as i16 + rank_offset;
        let new_file = start_file as i16 + file_offset;
        
        if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
            let target_square = new_rank * 8 + new_file;
            let target_piece = board[target_square as usize];

            if is_color(&target_piece.1, &board[start_square].1) {
                continue;
            }

            let movement = Move { start_square: start_square as i16, target_square: target_square as i16 };
            moves.push(movement);

            if is_opponent_color(&target_piece.1, &board[start_square].1) {
                let movement = Move {
                    start_square: start_square as i16,
                    target_square,
                };
                
                if !attacked_squares.contains(&target_square) {
                    moves.push(movement);
                    attacked_squares.push(target_square);
                }
        }
    }
}
}
