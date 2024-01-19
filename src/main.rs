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
mod error_types;
mod helpers;
mod moves;
mod types;

use chess_state::ChessState;

use crate::error_types::GameError;

use crate::chess_state::algebraic_notation_decoder;
use crate::helpers::*;
use crate::moves::*;
use crate::types::*;

use crate::chess_state::make_move;
use crate::chess_state::unmake_move;
use crate::generate_moves::generate_moves;
use crate::helpers::checks::unmake_move_based_on_check;
use crate::helpers::color::switch_color;
use crate::helpers::stalemate::stalemate::detect_stalemate;

use std::io::stdin;
use std::time::Instant;

// TODO:
// Board => FEN
//
// DOING:
//      Stalemates & Checkmates (Alice, post-movegen)
//      En Passant & Promotions (Small, movegen)
//
// BUGS:
// Castling movement broken
// Castling might have the same issue as the check logic.
//
fn main() {
    let fen = "4k3/8/8/8/4P3/8/2q5/4K3 b - - 0 1";

    let squares_to_edge = generate_moves::precompute_squares_to_edge();

    let mut fen_state = load_fen_state(fen.to_string());

    let mut previous_move = None;

    loop {
        let before = Instant::now();

        match game_loop(&mut fen_state, &squares_to_edge, previous_move) {
            Ok(move_made) => previous_move = move_made,
            Err(err) => println!("Error: {}", err),
        }

        println!("Elapsed time: {:.2?}", before.elapsed());
    }
}

fn game_loop(
    fen_state: &mut ChessState,
    squares_to_edge: &[[i16; 8]; 64],
    previous_move: Option<(Move, BoardPiece, BoardPiece)>,
) -> Result<Option<(Move, BoardPiece, BoardPiece)>, GameError> {
    let (friendly_piece_locations, friendly_movements) = generate_moves(
        &fen_state.board,
        &fen_state.color_to_move,
        &fen_state.is_able_to_castle,
        squares_to_edge,
    );
    let (_, enemy_movements) = generate_moves(
        &fen_state.board,
        &switch_color(&fen_state.color_to_move),
        &fen_state.is_able_to_castle,
        squares_to_edge,
    );
    let is_in_check = checks::detect_check(&friendly_piece_locations, &enemy_movements);

    if !is_in_check {
        let _is_in_stalemate = detect_stalemate(
            &fen_state.board,
            &friendly_piece_locations,
            &friendly_movements,
            &enemy_movements,
        );
    }

    fen_state.color_to_move = switch_color(&fen_state.color_to_move);
    match unmake_move_based_on_check(&mut fen_state.board, previous_move, is_in_check) {
        Ok(()) => (),
        Err(err) => {
            fen_state.color_to_move = switch_color(&fen_state.color_to_move);
            println!("{}", err)
        },
    }


    let (_, friendly_movements) = generate_moves(
        &fen_state.board,
        &fen_state.color_to_move,
        &fen_state.is_able_to_castle,
        &squares_to_edge,
    );

    display::display_chess_tui(&fen_state, &friendly_movements);

    let user_input = match get_user_move() {
        Ok(input) => input,
        Err(_) => {
            fen_state.color_to_move = switch_color(&fen_state.color_to_move);
            return Err(GameError::UserMoveError("Failed to get user move".to_string()));
        },
    };

    let (start_square_index, end_square_index) = algebraic_notation_decoder(&user_input)
        .map_err(|_| GameError::NotationDecoderError("Failed to decode notation".to_string()))?;

    Ok(make_user_move(
        fen_state,
        &friendly_movements,
        start_square_index,
        end_square_index,
        user_input,
    ))
}

fn load_fen_state(fen: String) -> ChessState {
    match fen::load_fen_state(fen) {
        Ok(state) => state,
        Err(err) => {
            println!("Error! {}", err);
            std::process::exit(1);
        }
    }
}

fn make_user_move(
    fen_state: &mut ChessState,
    friendly_moves: &Vec<Move>,
    start_square_index: u32,
    end_square_index: u32,
    user_input: String,
) -> Option<(Move, BoardPiece, BoardPiece)> {
    match make_move(
        &mut fen_state.board,
        friendly_moves,
        start_square_index,
        end_square_index,
    ) {
        Ok(move_made) => {
            println!("Moved to {}", user_input);
            Some(move_made)
        }
        Err(err) => {
            fen_state.color_to_move = switch_color(&fen_state.color_to_move);
            println!("{}", err);
            None
        }
    }
}

fn get_user_move() -> Result<String, &'static str> {
    let mut input = String::new();
    println!("Enter your move (e.g. e2e4, f4e2):");
    stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim();
    if input.len() != 4 {
        return Err("Invalid input. Please enter a move like e2e4.");
    }

    Ok(input.to_string())
}
