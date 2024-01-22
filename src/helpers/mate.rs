use crate::{
    chess_state::{make_move, ChessState},
    generate_moves,
    helpers::checks::detect_check,
    switch_color,
    types::SquaresToEdge,
    Move,
};

pub enum Mate {
    Stalemate,
    Checkmate,
    No,
}

/**
 * Detects stalemates including fifty-move rule, insufficient material, etc.
 *
 * https://www.chessprogramming.org/Stalemate#Detecting_Stalemate
 */
pub fn detect_mate(
    fen_state: &ChessState,
    squares_to_edge: &SquaresToEdge,
    is_in_check: bool,
) -> Mate {
    let color_to_move = switch_color(&fen_state.color_to_move);
    let (_, friendly_movements) = generate_moves(
        &fen_state,
        squares_to_edge,
        false,
    );

    let trapped = friendly_movements
        .iter()
        .all(|mov| is_move_into_check(&friendly_movements, fen_state, mov, squares_to_edge));

    match (trapped, is_in_check) {
        (true, true) => Mate::Checkmate,
        (true, false) => Mate::Stalemate,
        _ => Mate::No,
    }
}

fn is_move_into_check(
    friendly_movements: &Vec<Move>,
    fen_state: &ChessState,
    mov: &Move,
    squares_to_edge: &SquaresToEdge,
) -> bool {
    match make_move(
        fen_state,
        friendly_movements,
        mov.start_square as u32,
        mov.target_square as u32,
    ) {
        Ok(modified_fen_state) => {
            let (friendly_piece_locations, _) = generate_moves(
                &modified_fen_state,
                squares_to_edge,
                false,
            );
            let (_, enemy_movements) = generate_moves(
                &fen_state,
                squares_to_edge,
                true,
            );
            detect_check(&friendly_piece_locations, &enemy_movements)
        }
        Err(err) => {
            println!("{}", err);
            false
        }
    }
}
