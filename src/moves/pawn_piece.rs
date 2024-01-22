use crate::{
    chess_state::ChessState,
    types::{BoardPiece, ChessPieces, Move, MoveType, PieceColor},
};

/*
 * Promotion left.
 */
pub fn generate_pawn_moves(
    en_passant_target: &Option<i16>,
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
            // moving the pieces
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

                if start_rank == initial_rank {
                    let double_target_square = start_square as i16 + direction_offset * 2;
                    let first_blocking_piece =
                        board.get((start_square as i16 + direction_offset) as usize);
                    let second_blocking_piece = board.get(double_target_square as usize);
                    if let (Some(first_blocking_piece), Some(second_blocking_piece)) =
                        (first_blocking_piece, second_blocking_piece)
                    {
                        if first_blocking_piece.piece_type == ChessPieces::Empty
                            && second_blocking_piece.piece_type == ChessPieces::Empty
                        {
                            let double_movement = Move {
                                start_square: start_square as i16,
                                target_square: double_target_square,
                                move_type: MoveType::NoCapture,
                            };
                            moves.push(double_movement);
                        }
                    }
                }
            }
            // eating the pieces.
            //
            //
            // Inside your generate_pawn_moves function...
            if matches!(direction_offset, -7 | 7 | -9 | 9) {
                // Check if the pawn is not moving off the edge of the board when capturing diagonally
                let start_file = start_square % 8;
                let target_file = target_square as usize % 8;
                if (start_file == 0 && target_file == 7) || (start_file == 7 && target_file == 0) {
                    continue;
                }
                // Normal diagonal capture
                if target_piece.piece_type != ChessPieces::Empty {
                    let movement = Move {
                        start_square: start_square as i16,
                        target_square,
                        move_type: MoveType::Normal,
                    };
                    moves.push(movement);
                }
                // En passant capture
                else if is_en_passant_capture(target_square, en_passant_target) {
                    let movement = Move {
                        start_square: start_square as i16,
                        target_square,
                        move_type: MoveType::EnPassant,
                    };
                    moves.push(movement);
                }

                //    else if is_promotion_square(target_square, current_player_color) {
                //        let movement = Move {
                //            start_square: start_square as i16,
                //            target_square,
                //            move_type: MoveType::Promotion,
                //        };
                //        moves.push(movement);
                //    }
            }
        }
    }
}

fn is_en_passant_capture(target_square: i16, en_passant_target: &Option<i16>) -> bool {
    match en_passant_target {
        Some(ep_square) => target_square == *ep_square,
        None => false,
    }
}

//fn is_promotion_square(square: i16, color: &PieceColor) -> bool {
//    match color {
//        PieceColor::White => square / 8 == 7,
//        PieceColor::Black => square / 8 == 0,
//        _ => false,
//    }
//}
