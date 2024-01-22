use crate::{
    color, king_piece, knight_piece, pawn_piece, sliding_piece, types::SquaresToEdge, BoardPiece,
    Castle, ChessPieces, Move, PieceColor, chess_state::ChessState,
};
use core::cmp::min;

/**
 * Caching moves to save computation.
 * It returns the "amount of steps" a piece takes in order to reach the "edge" of the board.
 */
pub fn precompute_squares_to_edge() -> SquaresToEdge {
    let mut sq_to_edge = [[0; 8]; 64];

    for file in 0..8 {
        // vertical
        for rank in 0..8 {
            // horizontal
            let north = 7 - rank;
            let south = rank.clone();
            let west = file.clone();
            let east = 7 - file;

            let nw = min(north, west);
            let se = min(south, east);
            let ne = min(north, east);
            let sw = min(south, west);

            let square_index = rank * 8 + file;

            sq_to_edge[square_index as usize] = [north, south, west, east, nw, se, ne, sw];
        }
    }

    sq_to_edge
}

/**
 * Generates available moves.
 */
pub fn generate_moves(
    fen_state: &ChessState,
    board: &[BoardPiece; 64],
    current_player_color: &PieceColor,
    is_able_to_castle: &Castle,
    sqs_to_edge: &SquaresToEdge,
) -> (Vec<(ChessPieces, usize)>, Vec<Move>) {
    let mut pieces = Vec::<(ChessPieces, usize)>::new();
    let mut moves = Vec::<Move>::new();

    for start_square in 0..64 {
        // we're currently just caching all moves that a piece can do in a vector
        // it scans every square for a piece
        let piece = board.get(start_square);
        if let Some(piece) = piece {
            if piece.piece_type != ChessPieces::Empty
                && color::is_color(&piece.piece_color, current_player_color)
            {
                pieces.push((piece.piece_type, start_square));
                match &piece.piece_type {
                    ChessPieces::Kings => king_piece::generate_king_moves(
                        start_square,
                        board,
                        &mut moves,
                        is_able_to_castle,
                    ),
                    ChessPieces::Knights => {
                        knight_piece::generate_knight_moves(start_square, board, &mut moves)
                    }
                    ChessPieces::Bishops | ChessPieces::Queens | ChessPieces::Rooks => {
                        sliding_piece::generate_sliding_pieces(
                            start_square,
                            board,
                            &mut moves,
                            sqs_to_edge,
                        )
                    }
                    ChessPieces::Pawns => pawn_piece::generate_pawn_moves(
                        fen_state,
                        start_square,
                        board,
                        current_player_color,
                        &mut moves,
                    ),
                    _ => (),
                };
            }
        }
    }

    return (pieces, moves);
}
