use crate::color::{is_color, is_opponent_color};
use crate::types::{BoardPiece, ChessPieces, Move, MoveType, SquaresToEdge};

pub fn generate_sliding_pieces(
    start_square: usize,
    board: &[BoardPiece; 64],
    moves: &mut Vec<Move>,
    sqs_to_edge: &SquaresToEdge,
) {
    let start_piece = &board[start_square];
    let direction_offsets: [i16; 8] = [
        8, -8, -1, 1, // Up, Down, Left, Right
        7, -7, 9, -9, // Diagonals
    ];

    let (start_direction_index, end_direction_index) = match start_piece.piece_type {
        ChessPieces::Queens => (0, 8),
        ChessPieces::Bishops => (4, 8),
        ChessPieces::Rooks => (0, 4),
        _ => (0, 8),
    };

    for direction_index in start_direction_index..end_direction_index {
        for n in 0..sqs_to_edge[start_square][direction_index] {
            // loops through the "directions"
            let target_square = start_square as i16 + direction_offsets[direction_index] * (n + 1);
            let target_piece = board.get(target_square as usize);

            if let Some(target_piece) = target_piece {
                // blocked by a friendly piece, don't go further
                let movement = Move {
                    start_square: start_square as i16,
                    target_square,
                    move_type: MoveType::Normal,
                };
                if is_color(&target_piece.piece_color, &start_piece.piece_color) {
                    break;
                }
                if is_opponent_color(&target_piece.piece_color, &start_piece.piece_color) {
                    moves.push(movement);
                    break;
                }
                moves.push(movement);
            }
        }
    }
}
