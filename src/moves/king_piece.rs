
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

fn check_castle_condition(ranks: &[ChessPieces; 8], is_able_to_castle: &Castle) {
    if is_able_to_castle.queenside {
        if let [ChessPieces::Rooks, ChessPieces::Empty, ChessPieces::Empty, ..] = ranks {
            println!("King can castle. Queenside.");
        }
    }

    if is_able_to_castle.kingside {
        if let [.., ChessPieces::Empty, ChessPieces::Empty, ChessPieces::Rooks] = ranks {
            println!("King can castle. Kingside.");
        }
    }
}