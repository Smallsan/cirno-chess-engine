
use crate::types::{BoardPiece, Move, ChessPieces, PieceColor};
use crate::color::is_opponent_color;

/*
 * Promotion left.
 */
pub fn generate_pawn_moves(
    start_square: usize,
    board: &[BoardPiece; 64],
    current_player_color: &PieceColor,
    moves: &mut Vec<Move>,
    attacked_squares: &mut Vec::<i16>,
) {
    let direction_offsets = match current_player_color {
        PieceColor::White => [8, 7, 9],    // White pawn moves
        PieceColor::Black => [-8, -7, -9], // Black pawn moves
        _ => [0; 3],                       // Placeholder for None or other colors
    };

    for &direction_offset in &direction_offsets {
        let target_square = start_square as i16 + direction_offset;
        let target_piece = board.get(target_square as usize);

        if let Some(target_piece) = target_piece {
            if matches!(direction_offset, -8 | 8) && target_piece.0 == ChessPieces::Empty {
                let movement = Move {
                    start_square: start_square as i16,
                    target_square,
                    move_type: Default::default() 
                };
                moves.push(movement);

                // Add double move for pawns from initial position
                let initial_rank = match current_player_color {
                    PieceColor::White => 1,
                    PieceColor::Black => 6,
                    _ => 0, // Placeholder
                };

                let start_rank = start_square / 8;

                if start_rank == initial_rank && target_piece.0 == ChessPieces::Empty {
                    let double_target_square = start_square as i16 + direction_offset * 2;
                    let double_movement = Move {
                        start_square: start_square as i16,
                        target_square: double_target_square,
                        move_type: Default::default() 
                    };
                    moves.push(double_movement);
                }
            }

            // eating.
            if matches!(direction_offset, -7 | 7 | -9 | 9) {
                if is_opponent_color(&target_piece.1, current_player_color) {
                    let movement = Move {
                        start_square: start_square as i16,
                        target_square,
                        move_type: Default::default() 
                    };
                    moves.push(movement);
                    attacked_squares.push(target_square);
                }
            }
        }
    }

}
