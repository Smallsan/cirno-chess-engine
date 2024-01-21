use crate::{
    chess_state::{make_move, ChessState},
    helpers::checks::detect_check,
    types::SquaresToEdge,
    generate_moves,
    switch_color,
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
    // if we were generating fully legal moves,
    //      this would've been very easy.
    // fully legal moves are extremely hard to generate though
    //
    // try every move and if there's at least one legal move,
    //      it's not a stalemate
    
    let mut fen_state = fen_state.clone(); // it won't cost thaat much.
    fen_state.color_to_move = switch_color(&fen_state.color_to_move);
    let (_, friendly_movements) = generate_moves(
        &fen_state.board,
        &fen_state.color_to_move,
        &fen_state.is_able_to_castle,
        squares_to_edge,
    );
    dbg!(&friendly_movements);

    // somehow include checkmates into this.
    let trapped = friendly_movements.iter().all(|mov| {
        match make_move(
            &fen_state,
            &friendly_movements,
            mov.start_square as u32,
            mov.target_square as u32,
        ) {
            Ok(modified_fen_state) => {
                let (friendly_piece_locations, _) = generate_moves(
                    &modified_fen_state.board,
                    &modified_fen_state.color_to_move,
                    &modified_fen_state.is_able_to_castle,
                    squares_to_edge,
                );
                let (_, enemy_movements) = generate_moves(
                    &modified_fen_state.board,
                    &switch_color(&modified_fen_state.color_to_move),
                    &modified_fen_state.is_able_to_castle,
                    squares_to_edge,
                );
                let is_in_check = detect_check(&friendly_piece_locations, &enemy_movements);
                is_in_check
            }
            Err(err) => {
                println!("{}", err);
                false
            }
        }
    });
    if trapped && is_in_check {
        Mate::Checkmate
    } else if trapped {
        Mate::Stalemate
    } else {
        Mate::No
    }
}
