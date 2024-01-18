use crate::{ChessPieces, Move};

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


