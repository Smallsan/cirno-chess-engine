use crate::types::{BoardPiece, ChessPieces, Move, MoveType, PieceColor};

/*
 * Promotion left.
 */
pub fn generate_pawn_moves(
    start_square: usize,
    board: &[BoardPiece; 64],
    current_player_color: &PieceColor,
    moves: &mut Vec<Move>,
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
            if matches!(direction_offset, -8 | 8) && target_piece.piece_type == ChessPieces::Empty {
                let movement = Move {
                    start_square: start_square as i16,
                    target_square,
                    move_type: MoveType::NoCapture,
                };
                moves.push(movement);

                // Add double move for pawns from initial position
                let initial_rank = match current_player_color {
                    PieceColor::White => 1,
                    PieceColor::Black => 6,
                    _ => 0, // Placeholder
                };

                let start_rank = start_square / 8;

                // moving the pieces
                if start_rank == initial_rank && target_piece.piece_type == ChessPieces::Empty {
                    let double_target_square = start_square as i16 + direction_offset * 2;
                    let double_movement = Move {
                        start_square: start_square as i16,
                        target_square: double_target_square,
                        move_type: MoveType::NoCapture,
                    };
                    moves.push(double_movement);
                }
            }

            // eating the pieces.
            //
            // 
            if matches!(direction_offset, -7 | 7 | -9 | 9) && target_piece.piece_type != ChessPieces::Empty {
                let movement = Move {
                    start_square: start_square as i16,
                    target_square,
                    move_type: MoveType::Normal,
                };
                moves.push(movement);
            }
        }
    }
}
