
use crate::helpers::color::is_color;
use crate::types::{BoardPiece, Move, Castle, ChessPieces, PieceColor, MoveType};
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
    let start_piece = board[start_square];

    let (start_rank, start_file) = (start_square / 8, start_square % 8);
    // rank = horizontal
    // file = vertical

    let first_square_of_rank = start_rank * 8;
    let last_square_of_rank = (start_rank + 1) * 8;
    let rank_range = first_square_of_rank .. last_square_of_rank;
    let castle = check_castle_condition(board.map(|f| f.0)[rank_range].try_into().unwrap(), is_able_to_castle);

    if castle.queenside {
        moves.push(Move { 
            start_square: start_square as i16, 
            target_square: start_square as i16 - 3, 
            move_type: MoveType::Castle 
        });
    }
    if castle.kingside {
        moves.push(Move { 
            start_square: start_square as i16, 
            target_square: start_square as i16 + 2, 
            move_type: MoveType::Castle 
        });
    }

    for (rank_offset, file_offset) in king_moves {
        let new_rank = start_rank as i16 + rank_offset;
        let new_file = start_file as i16 + file_offset;

        if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
            let target_square = (new_rank * 8 + new_file) as i16;
            let target_piece = board[target_square as usize];

            if is_color(&start_piece.1, &target_piece.1) {
                continue;
            }

            moves.push(Move {
                start_square: start_square as i16,
                target_square,
                move_type: Default::default() 
            });


            if is_opponent_color(&target_piece.1, &start_piece.1) {
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

// is_able_to_castle is a struct that's built from FEN and is modified by a piece moving.
fn check_castle_condition(ranks: &[ChessPieces; 8], is_able_to_castle: &Castle) -> Castle {
    let mut sides = Castle {
        queenside: false,
        kingside: false,
    };
    if is_able_to_castle.queenside {
        if let [ChessPieces::Rooks, ChessPieces::Empty, ChessPieces::Empty, ChessPieces::Empty, ..] = ranks {
            sides.queenside = true;
        } else {
            sides.queenside = false;
        }
    };

    if is_able_to_castle.kingside {
        if let [.., ChessPieces::Empty, ChessPieces::Empty, ChessPieces::Rooks] = ranks {
            sides.kingside = true;
        } else {
            sides.kingside = false;
        }
    };
    sides
}
