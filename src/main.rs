// ROADMAP: Movement (We are here.) => Search functions => Evaluation functions => Make the move.
//
// Search functions
//
// Evaluation Functions can "evaluate" certain positions with certain pieces
//      Implementation:
//      1. Extract all legal moves (We're here!)
//      2. Make those moves and use the evaluation function to get a score
//      3. Get all those moves and pick the best one
//
//      Minimax algorithm:
//      1. Takes in a board and a depth, indicating how deep we'll search. We'll return the best
//         achievable evaluation.
//      2. We'll search to a depth of 2,
//         white => black's response to that move => white's response to those moves
//
// Future issues:
//
// How do you "make" moves in this engine?
// Our state is currently in a FEN string.
//
// We could have a function that actually
//      moves the pieces in the chess board using Algebraic Notation.

mod helpers;
mod moves;
mod types;

use crate::helpers::*;
use crate::moves::sliding_piece::find_pinned_pieces_in_square;
use crate::moves::*;
use crate::types::*;
use std::time::Instant;

// TODO
// Pawn Pieces
//      - En Passant
//      - Promotions
// King
//      - King Checks (In Progress)
//          - Restricting movement to only the king.
//      - King Pins
//          - Restricting allowed movement of the pinned piece.
//
// Loop over pieces instead of the entire board.
// Algebraic Notation for User Input
fn main() {
    let _pawn = vec![
        "8/8/8/8/2r1P1K1/8/8/8 w",
        "8/8/8/8/8/4p3/3P1P2/8 b",
        "8/8/8/8/8/3npb2/3P1P2/8 w",
    ];
    let _castling = vec![
        "rnbqk2r/ppp5/8/8/8/8/P7/R2QKBNR b KQkq - 0 1",
        "rnbqk2r/ppp5/8/8/8/8/P7/R3K2R w KQkq - 0 1",
    ];
    let check = vec![
        "rnbq1bnr/pppppppp/8/8/2k1Q3/8/PPPP1PPP/RNB1KBNR w KQ - 0 1",
        "8/8/8/8/2k1Q3/8/8/8 w - 0 1",
        "8/8/8/8/2k1Q3/8/8/2R5 b",
        "8/8/3B4/8/2k1Q3/6B1/8/2R5 w",
    ];

    let squares_to_edge = generate_moves::precompute_squares_to_edge();

    let before = Instant::now();
    for fen in check {
        let fen_state = match fen::load_fen_state(fen.to_string()) {
            Ok(state) => state,
            Err(err) => {
                println!("Error! {}", err);
                break;
            }
        };

        let friendly_movements = generate_moves(
            &fen_state.board,
            &fen_state.color_to_move,
            &fen_state.is_able_to_castle,
            &squares_to_edge,
        );
        // detect check and pinned here.
        let has_check = detect_check(&fen_state.board, &friendly_movements);
        let pinned_pieces =
            find_pinned_pieces_in_board(&fen_state.board, &friendly_movements, &squares_to_edge);
        let friendly_movements = if let Some(has_check) = has_check {
            for piece_index in 0..64 {
                let piece = &fen_state.board[piece_index as usize];
                if piece.0 == ChessPieces::Kings {
                    return friendly_movements.iter().filter(|moves| moves.start_square == piece_index).collect();
                }
            }
            friendly_movements
        } else {
            friendly_movements
        };
        
        display::display_chess_tui(&fen_state, &friendly_movements);
    }
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn find_pinned_pieces_in_board(
    board: &[BoardPiece; 64],
    movement: &Vec<Move>,
    sqs_to_edge: &SquaresToEdge,
) -> Vec<(i16, ChessPieces, PieceColor)> {
    let mut pinned_pieces: Vec<(i16, ChessPieces, PieceColor)> = vec![];
    for moves in movement {
        if matches!(
            board[moves.start_square as usize].0,
            ChessPieces::Rooks | ChessPieces::Queens | ChessPieces::Bishops
        ) {
            let mut pinned =
                find_pinned_pieces_in_square(board, moves.start_square as usize, sqs_to_edge);
            pinned_pieces.extend(pinned.drain(..));
        }
    }
    pinned_pieces
}

fn detect_check(board: &[BoardPiece; 64], movement: &Vec<Move>) -> Option<(usize, usize)> {
    let mut position = 0;
    for start_square in board {
        if let Some(king_piece_move) = movement.iter().find(|moves| {
            &board[moves.start_square as usize] == start_square
                && board[moves.target_square as usize].0 == ChessPieces::Kings
                && !color::is_color(&board[moves.target_square as usize].1, &start_square.1)
                && moves.move_type != MoveType::NoCapture
        }) {
            return Some((position as usize, king_piece_move.target_square as usize));
        }
        position += 1;
    }
    None
}

/**
 * Generates available moves.
 */
fn generate_moves(
    board: &[BoardPiece; 64],
    current_player_color: &PieceColor,
    is_able_to_castle: &Castle,
    sqs_to_edge: &SquaresToEdge,
) -> Vec<Move> {
    let mut moves = Vec::<Move>::new();

    for start_square in 0..64 {
        // we're currently just caching all moves that a piece can do in a vector
        // it scans every square for a piece
        let piece = board.get(start_square);
        if let Some(piece) = piece {
            if piece.0 != ChessPieces::Empty && color::is_color(&piece.1, current_player_color) {
                match &piece.0 {
                    ChessPieces::Kings => king_piece::generate_king_moves(
                        start_square,
                        board,
                        &mut moves,
                        is_able_to_castle,
                    ),
                    ChessPieces::Knights => {
                        knight_piece::generate_knight_moves(start_square, board, &mut moves)
                    }
                    ChessPieces::Bishops | ChessPieces::Queens | ChessPieces::Rooks => {
                        sliding_piece::generate_sliding_pieces(
                            start_square,
                            board,
                            &mut moves,
                            sqs_to_edge,
                        )
                    }
                    ChessPieces::Pawns => pawn_piece::generate_pawn_moves(
                        start_square,
                        board,
                        current_player_color,
                        &mut moves,
                    ),
                    _ => (),
                };
            }
        }
    }

    return moves;
}
