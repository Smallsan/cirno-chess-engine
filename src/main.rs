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

use chess_state::ChessState;

use crate::chess_state::make_move;
use crate::chess_state::unmake_move;
use crate::helpers::color::switch_color;
use crate::helpers::*;
use crate::moves::sliding_piece::find_pinned_pieces_in_square;
use crate::moves::*;
use crate::types::*;

use std::io::stdin;
use std::time::Instant;

// TODO:
// Pawn Pieces
//      - En Passant
//      - Promotions
// Detect checkmate or stalemate
//
// BUGS:
// Castling movement broken
//
// Check logic doesn't work for both sides.
//      ChessState has one boolean variable determining if
//      someone is in check or not.
//      It should have different booleans for both sides.
// Castling might have the same issue as the check logic.
//
fn main() {
    let fen = "8/pppppppp/8/8/8/8/PPPPPPPP/8 b HAha - 0 1";

    let squares_to_edge = generate_moves::precompute_squares_to_edge();

    let mut fen_state = match fen::load_fen_state(fen.to_string()) {
        Ok(state) => state,
        Err(err) => {
            println!("Error! {}", err);
            return;
        }
    };

    ////////////////////////////////////
    let (_, friendly_movements) = generate_moves(
        &fen_state.board,
        &fen_state.color_to_move,
        &fen_state.is_able_to_castle,
        &squares_to_edge,
    );
    let pinned_pieces =
        find_pinned_pieces_in_board(&fen_state.board, &friendly_movements, &squares_to_edge);
    ////////////////////////////////////

    /////////////////// GAME LOOP ////////////////////
    let mut previous_move = None;
    loop {
        let before = Instant::now();

        let friendly =
            generate_moves_based_on_check(&mut fen_state, &squares_to_edge, previous_move);
        if let Some(friendly) = &friendly {
            let (_, friendly_movements) = friendly;
            display::display_chess_tui(&fen_state, &friendly_movements);

            let notation = get_user_input().expect("Failed to scan notation.");

            previous_move =
                match make_move(&mut fen_state.board, &friendly_movements, notation.as_str()) {
                    Ok(mov) => {
                        println!("Moved to {}", notation);
                        Some(mov)
                    }
                    Err(err) => {
                        fen_state.color_to_move = switch_color(&fen_state.color_to_move);
                        println!("{}", err);
                        None
                    }
                };
        }

        println!("Elapsed time: {:.2?}", before.elapsed());
    }
}

fn generate_moves_based_on_check(
    fen_state: &mut ChessState,
    sqs_to_edge: &SquaresToEdge,
    previous_move: Option<(Move, BoardPiece, BoardPiece)>,
) -> Option<(Vec<(ChessPieces, usize)>, Vec<Move>)> {
    let (friendly_piece_locations, _) = generate_moves(
        &fen_state.board,
        &fen_state.color_to_move,
        &fen_state.is_able_to_castle,
        &sqs_to_edge,
    );
    let (_, enemy_movements) = generate_moves(
        &fen_state.board,
        &switch_color(&fen_state.color_to_move),
        &fen_state.is_able_to_castle,
        &sqs_to_edge,
    );
    dbg!(previous_move.is_some());
    let has_check = detect_check(&friendly_piece_locations, &enemy_movements);
    if let Some(check) = has_check {
        if check {
            if let Some(previous_move) = previous_move {
                match unmake_move(&mut fen_state.board, previous_move) {
                    Ok(()) => {
                        println!("Move resulted in a check, unmade move.");
                        let mov = Some(generate_moves(
                            &fen_state.board,
                            &fen_state.color_to_move,
                            &fen_state.is_able_to_castle,
                            &sqs_to_edge,
                        ));
                        return mov;
                    }
                    Err(err) => {
                        println!("Error. {}", err);
                        return None;
                    }
                }
            } else {
                Some(generate_moves(
                    &fen_state.board,
                    &fen_state.color_to_move,
                    &fen_state.is_able_to_castle,
                    &sqs_to_edge,
                ))
            }
        } else {
            Some(generate_moves(
                &fen_state.board,
                &fen_state.color_to_move,
                &fen_state.is_able_to_castle,
                &sqs_to_edge,
            ))
        }
    } else {
        fen_state.color_to_move = switch_color(&fen_state.color_to_move);
        // 
        Some(generate_moves(
            &fen_state.board,
            &fen_state.color_to_move,
            &fen_state.is_able_to_castle,
            &sqs_to_edge,
        ))
    }

}

fn get_user_input() -> Result<String, &'static str> {
    let mut input = String::new();
    println!("Enter your move (e.g. e2e4, f4e2):");
    stdin().read_line(&mut input).expect("Failed to read line");

    Ok(input)
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
