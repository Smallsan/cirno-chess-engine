use crate::color::is_opponent_color;
use crate::helpers::color::is_color;
use crate::types::{BoardPiece, Castle, ChessPieces, Move, MoveType, PieceColor};

/**
 * King:
 * - Castling (in progress)
 * - Pins
 *
 */

pub fn generate_king_moves(
    start_square: usize,
    board: &[BoardPiece; 64],
    moves: &mut Vec<Move>,
    is_able_to_castle: &Castle,
) {
    let king_moves = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let start_piece = board[start_square];

    // this relies on the piece's square
    //
    // which means it actually scans the piece's rank and file
    // rather than the board's rank and file
    //
    // not a big deal since this bug will be caught by the FEN castling flag
    let (start_rank, start_file) = (start_square / 8, start_square % 8);
    // rank = horizontal
    // file = vertical

    let first_square_of_rank = start_rank * 8;
    let last_square_of_rank = (start_rank + 1) * 8;
    let rank_range = first_square_of_rank..last_square_of_rank;
    let (queenside, kingside) = check_castle_condition(
        board.map(|f| f.piece_type)[rank_range].try_into().unwrap(),
        is_able_to_castle,
        &start_piece.piece_color,
    );

    if queenside {
        moves.push(Move {
            start_square: start_square as i16,
            target_square: start_square as i16 - 2,
            move_type: MoveType::Castle,
        });
    }
    if kingside {
        moves.push(Move {
            start_square: start_square as i16,
            target_square: start_square as i16 + 2,
            move_type: MoveType::Castle,
        });
    }

    for (rank_offset, file_offset) in king_moves {
        let new_rank = start_rank as i16 + rank_offset;
        let new_file = start_file as i16 + file_offset;

        if new_rank >= 0 && new_rank < 8 && new_file >= 0 && new_file < 8 {
            let target_square = (new_rank * 8 + new_file) as i16;
            let target_piece = board[target_square as usize];

            if is_color(&start_piece.piece_color, &target_piece.piece_color) {
                continue;
            }

            if is_opponent_color(&target_piece.piece_color, &start_piece.piece_color) {
                moves.push(Move {
                    start_square: start_square as i16,
                    target_square,
                    move_type: MoveType::Normal,
                });
                continue;
            }

            moves.push(Move {
                start_square: start_square as i16,
                target_square,
                move_type: MoveType::Normal,
            });
        }
    }
}

// is_able_to_castle is a struct that's built from FEN and is modified by a piece moving.
fn check_castle_condition(ranks: &[ChessPieces; 8], is_able_to_castle: &Castle, current_color: &PieceColor) -> (bool, bool) {
    let mut sides = (false, false);
    if is_able_to_castle.black_queenside && current_color == &PieceColor::Black {
        if let [ChessPieces::Rooks, ChessPieces::Empty, ChessPieces::Empty, ChessPieces::Empty, ..] =
            ranks
        {
            sides.0 = true;
        } else {
            sides.0 = false;
        }
    };

    if is_able_to_castle.black_kingside && current_color == &PieceColor::Black {
        if let [.., ChessPieces::Empty, ChessPieces::Empty, ChessPieces::Rooks] = ranks {
            sides.1 = true;
        } else {
            sides.1 = false;
        }
    };
    sides
}
