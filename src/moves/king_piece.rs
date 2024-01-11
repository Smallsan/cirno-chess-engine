
use crate::helpers::color::is_color;
use crate::types::{BoardPiece, Move, Castle, ChessPieces};
use crate::color::is_opponent_color;

/**
 * King:
 * - Castling (in progress)
 * - Pins
 *
 * make FEN decoder support castling
 */
pub fn generate_king_moves(start_square: usize, board: &[BoardPiece; 64], moves: &mut Vec<Move>, attacked_squares: &mut Vec<i16>, is_able_to_castle: &Castle) {
    let king_moves = [
        (-1, -1), (-1, 0), (-1, 1), (0, -1),
        (0, 1), (1, -1), (1, 0), (1, 1),
    ];

    let (start_rank, start_file) = (start_square / 8, start_square % 8);

    check_castle_condition(board.map(|f| f.0)[0..8].try_into().unwrap(), is_able_to_castle);

    for (rank_offset, file_offset) in king_moves {
        let new_rank = start_rank as i16 + rank_offset;
        let new_file = start_file as i16 + file_offset;

        if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
            let target_square = (new_rank * 8 + new_file) as i16;
            let target_piece = board[target_square as usize];

            if is_color(&board[start_square].1, &target_piece.1) {
                continue;
            }

            moves.push(Move {
                start_square: start_square as i16,
                target_square,
                move_type: Default::default() 
            });


            if is_opponent_color(&target_piece.1, &board[start_square].1) {
                moves.push(Move {
                    start_square: start_square as i16,
                    target_square,
                    move_type: Default::default() 
                });
                attacked_squares.push(target_square);
            }
        }
    }
}

fn check_castle_condition(rank: [ChessPieces; 8], is_able_to_castle: &Castle) {
    // is_able_to_castle will be used in the FEN string so we can maintain state.

    let kingside = [
        ChessPieces::Rooks, ChessPieces::Empty, ChessPieces::Empty, 
    ];
    let queenside = [
        ChessPieces::Empty, ChessPieces::Empty, ChessPieces::Empty, ChessPieces::Rooks
    ];
    if rank[0..3] == kingside && is_able_to_castle.kingside {
        println!("King can castle. Kingside.");
    } else if rank[5..8] == queenside && is_able_to_castle.queenside {
        println!("King can castle. Queenside.");
    }
}
