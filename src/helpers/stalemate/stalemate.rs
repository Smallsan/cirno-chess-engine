use crate::{
    chess_state::{make_move, unmake_move, ChessState},
    helpers::checks::detect_check,
    types::SquaresToEdge,
    generate_moves,
    switch_color,
    Move
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
    friendly_movements: &Vec<Move>,
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

    // somehow include checkmates into this.
    let trapped = friendly_movements.iter().all(|mov| {
        match make_move(
            &mut fen_state.board,
            &friendly_movements,
            mov.start_square as u32,
            mov.target_square as u32,
        ) {
            Ok(previous_move) => {
                let (friendly_piece_locations, _) = generate_moves(
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
                let is_in_check = detect_check(&friendly_piece_locations, &enemy_movements);
                match unmake_move(&mut fen_state.board, previous_move) {
                    Ok(()) => (),
                    Err(err) => println!("{}", err),
                };
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
