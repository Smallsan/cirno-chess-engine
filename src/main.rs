

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

mod helpers;
mod types;
mod moves;

use crate::moves::*;
use crate::types::*;
use crate::helpers::*;

// TODO
// Pawn Pieces
//      - En Passant
//      - Promotions
// King
//      - Castling
//
// Algebraic Notation for User Input
fn main() {
    let fens = vec![
        "r1bk3r/p2pBpNp/n4n2/1p1NP2P/6P1/3P4/P1P1K3/q5b1",
        "3k4/8/8/3p4/3P4/8/8/3K4",
        "8/8/4P3/1P2Q3/8/2P3N1/4N3/R7",
        "rnbqkbnr/ppp1p1pp/3p4/5p2/4P3/3P4/PPP2PPP/RNBQKBNR",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
        "8/3K1R2/6B1/8/8/3r4/3k4/8"
    ];

    for fen in fens {
        let mut chess_state = ChessState { color_to_move: PieceColor::White, ..Default::default() };
        let squares_to_edge = generate_moves::precompute_squares_to_edge();

        fen::load_position_from_fen(fen.to_string(), &mut chess_state.board);

        let (friendly_movements, friendly_attacking) = generate_moves(&chess_state.board, &chess_state.color_to_move, &squares_to_edge);
        display::display_chess_tui(&chess_state, &friendly_movements);
        println!("{}", format_attacking_squares(&chess_state.board, &friendly_attacking));

        chess_state.color_to_move = PieceColor::Black;

        let (enemy_movements, enemy_attacking) = generate_moves(&chess_state.board, &chess_state.color_to_move, &squares_to_edge);
        display::display_chess_tui(&chess_state, &enemy_movements);
        println!("{}", format_attacking_squares(&chess_state.board, &enemy_attacking));
    }
}

fn format_attacking_squares(board: &[BoardPiece; 64], enemy_attacking: &Vec<i16>) -> String {
    let mut str = String::new();
    for enemy_attack_index in enemy_attacking {
        let square = board[*enemy_attack_index as usize];
        dbg!(square, enemy_attack_index);
        let mut piece_string = helpers::display::format_piece(square);
        piece_string.insert_str(0, ", ");
        str += &piece_string;
    }
    str
}

/**
 * Generates available moves
 */
fn generate_moves(board: &[BoardPiece; 64], current_player_color: &PieceColor, sqs_to_edge: &SquaresToEdge) -> (Vec<Move>, Vec<i16>) {
    let mut moves = Vec::<Move>::new();
    let mut attacked_squares = Vec::<i16>::new(); // new addition
                                                  // let me give you a rundown.
                                                  //
                                                  // Attacked squares take in the attacked squares
                                                  // of White or Black.

    for start_square in 0..64 { 
        // we're currently just caching all moves that a piece can do in a vector
        // it scans every square for a piece
        let piece = board.get(start_square);
        if let Some(piece) = piece {
            if piece.0 != ChessPieces::Empty && color::is_color(&piece.1, current_player_color) {
                match &piece.0 {
                    ChessPieces::Kings => king_piece::generate_king_moves(start_square, board, &mut moves, &mut attacked_squares),
                    ChessPieces::Knights => knight_piece::generate_knight_moves(start_square, board, &mut moves, &mut attacked_squares),
                    ChessPieces::Bishops |
                    ChessPieces::Queens |
                    ChessPieces::Rooks => sliding_piece::generate_sliding_pieces(start_square, board, &mut moves, &mut attacked_squares, sqs_to_edge),
                    ChessPieces::Pawns => pawn_piece::generate_pawn_moves(start_square, board, current_player_color, &mut moves, &mut attacked_squares),
                    _ => (),
                };
            }
        }
    }

    return (moves, attacked_squares);
}
