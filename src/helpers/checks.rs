use crate::{
    generate_moves, unmake_move, BoardPiece, ChessPieces, ChessState, Move, SquaresToEdge,
};

/**
 * Detects checks the friendly has.
 * If there's a target square that hits with the king's start square,
 *      this will return those target squares.
 */
pub fn detect_check(
    friendly_piece_locations: &Vec<(ChessPieces, usize)>,
    enemy_movements: &Vec<Move>,
) -> bool {
    // finding the king
    let king_position = friendly_piece_locations
        .iter()
        .find(|x| x.0 == ChessPieces::Kings);
    if let Some(king_position) = king_position {
        // finding if the enemy has a movement in the king
        let has_enemy_intersection_with_king = enemy_movements
            .into_iter()
            .any(|mov| mov.target_square as usize == king_position.1);
        has_enemy_intersection_with_king
    } else {
        false
    }
}

/**
 * Unmakes moves on check.
 */
pub fn unmake_move_based_on_check(
    board: &mut [BoardPiece; 64],
    previous_move: Option<(Move, BoardPiece, BoardPiece)>,
    is_in_check: bool,
) -> Result<(), &'static str> {
    if is_in_check {
        if let Some(previous_move) = previous_move {
            match unmake_move(board, previous_move) {
                Ok(()) => {
                    return Err("Move resulted in a check, unmade move.");
                }
                Err(err) => {
                    println!("Error. {}", err);
                    return Err(err);
                }
            }
        }
    }
    Ok(())
}
