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

mod chess_state;
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
        "8/8/8/8/8/8/8/R3K3 w Q - 0 1",
        "8/8/8/8/8/8/8/R3Kp1R w KQha - 0 1",
        "8/8/8/8/8/8/8/R3K2R w KQha - 0 1",
    ];
    let check = vec![
        "rnbqkbnr/pppp1ppp/8/8/8/8/PPPPQPPP/RNB1KBNR w KQkq - 0 1",
        "rnbqkbnr/pppp1ppp/8/8/8/8/PPPPQPPP/RNB1KBNR b KQkq - 0 1",
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

        let (friendly_piece_locations, friendly_movements) = generate_moves(
            &fen_state.board,
            &fen_state.color_to_move,
            &fen_state.is_able_to_castle,
            &squares_to_edge,
        );
        let (enemy_piece_locations, enemy_movements) = generate_moves(
            &fen_state.board,
            match &fen_state.color_to_move {
                PieceColor::White => &PieceColor::Black,
                PieceColor::Black => &PieceColor::White,
                PieceColor::None => &PieceColor::White, // this will never happen so its safe to
                                                        // ignore it.
            },
            &fen_state.is_able_to_castle,
            &squares_to_edge,
        );
        // detect check and pinned here.
        let has_check = detect_check(&friendly_piece_locations, &enemy_movements);
        let pinned_pieces =
            find_pinned_pieces_in_board(&fen_state.board, &friendly_movements, &squares_to_edge);

        display::display_chess_tui(&fen_state, &friendly_movements);
    }
    println!("Elapsed time: {:.2?}", before.elapsed());
}

/**
 * Unimplemented: What direction a piece can go in the pin.
 */
fn find_pinned_pieces_in_board(
    board: &[BoardPiece; 64],
    movement: &Vec<Move>,
    sqs_to_edge: &SquaresToEdge,
) -> Vec<(i16, ChessPieces, PieceColor)> {
    // We could run the diagonal traversal again, or label "pierced" moves
    //      and compile all of them here.

    let mut pinned_pieces: Vec<(i16, ChessPieces, PieceColor)> = vec![];
    for moves in movement {
        if matches!(
            board[moves.start_square as usize].piece_type,
            ChessPieces::Rooks | ChessPieces::Queens | ChessPieces::Bishops
        ) {
            let mut pinned =
                find_pinned_pieces_in_square(board, moves.start_square as usize, sqs_to_edge);
            pinned_pieces.extend(pinned.drain(..));
        }
    }
    pinned_pieces
}

/**
 * Detects checks the friendly has.
 * If there's a target square that hits with the king's start square,
 *      this will return those target squares.
 */
fn detect_check(
    friendly_piece_locations: &Vec<(ChessPieces, usize)>,
    enemy_movements: &Vec<Move>,
) -> Option<bool> {
    // finding the king
    let king_position = friendly_piece_locations
        .iter()
        .find(|x| x.0 == ChessPieces::Kings);
    if let Some(king_position) = king_position {
        // finding if the enemy has a movement in the king
        let has_enemy_intersection_with_king = enemy_movements
            .into_iter()
            .any(|mov| mov.target_square as usize == king_position.1);
        Some(has_enemy_intersection_with_king)
    } else {
        None
    }
}

/**
 * Generates available moves.
 */
fn generate_moves(
    board: &[BoardPiece; 64],
    current_player_color: &PieceColor,
    is_able_to_castle: &Castle,
    sqs_to_edge: &SquaresToEdge,
) -> (Vec<(ChessPieces, usize)>, Vec<Move>) {
    let mut pieces = Vec::<(ChessPieces, usize)>::new();
    let mut moves = Vec::<Move>::new();

    for start_square in 0..64 {
        // we're currently just caching all moves that a piece can do in a vector
        // it scans every square for a piece
        let piece = board.get(start_square);
        if let Some(piece) = piece {
            if piece.piece_type != ChessPieces::Empty
                && color::is_color(&piece.piece_color, current_player_color)
            {
                pieces.push((piece.piece_type, start_square));
                match &piece.piece_type {
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

    return (pieces, moves);
}
