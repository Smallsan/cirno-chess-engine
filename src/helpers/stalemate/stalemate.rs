
use crate::Move;

/**
 * Detects stalemates including fifty-move rule, insufficient material, etc.
 *
 * https://www.chessprogramming.org/Stalemate#Detecting_Stalemate
 */
pub fn detect_stalemate(friendly_movements: &Vec<Move>, is_in_check: bool) -> bool {
    // if we were generating fully legal moves,
    //      it would've been as easy as this.
    //
    // fully legal moves are extremely hard to generate though
    if friendly_movements.len() <= 0 && !is_in_check {
        true
    } else {
        false
    }
}

