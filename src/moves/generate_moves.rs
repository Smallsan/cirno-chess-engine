
use core::cmp::min;
use crate::types::SquaresToEdge;

/**
 * Caching moves to save computation.
 * It returns the "amount of steps" a piece takes in order to reach the "edge" of the board
 */
pub fn precompute_squares_to_edge() -> SquaresToEdge {
    let mut sq_to_edge = [[0; 8]; 64];

    for file in 0..8 { // vertical
        for rank in 0..8 { // horizontal
            let north = 7 - rank;
            let south = rank.clone();
            let west = file.clone();
            let east = 7 - file;
            
            let nw = min(north, west);
            let se = min(south, east);
            let ne = min(north, east);
            let sw = min(south, west);

            let square_index = rank * 8 + file;

            sq_to_edge[square_index as usize] = [
                north, south, west, east,
                nw, se, ne, sw,
            ];
        }
    }

    sq_to_edge
}
