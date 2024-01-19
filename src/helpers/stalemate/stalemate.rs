use crate::{
    helpers::checks::{detect_check, unmake_move_based_on_check},
    types::{BoardPiece, ChessPieces},
    Move, chess_state::{make_move, ChessState, unmake_move},
};

/**
 * Detects stalemates including fifty-move rule, insufficient material, etc.
 *
 * https://www.chessprogramming.org/Stalemate#Detecting_Stalemate
 */
pub fn detect_stalemate(
    board: &[BoardPiece; 64],
    friendly_piece_locations: &Vec<(ChessPieces, usize)>,
    friendly_movements: &Vec<Move>,
    enemy_movements: &Vec<Move>,
) {
    // if we were generating fully legal moves,
    //      this would've been very easy.
    // fully legal moves are extremely hard to generate though
    //
    // try every move and if there's at least one legal move,
    //      it's not a stalemate

    let mut state = board.clone(); // it won't cost thaat much.

    for (piece_type, piece_index) in friendly_piece_locations {
        // separate notation logic from make_move
        match make_move(&mut board, &friendly_movements, notation.as_str()) {
            Ok(previous_move) => {
                let is_in_check = detect_check(friendly_piece_locations, enemy_movements);
                unmake_move(&mut board, previous_move);
                println!("Moved to {}", notation);
            }
            Err(err) => {
                println!("{}", err);
            }
        };
    }

}
