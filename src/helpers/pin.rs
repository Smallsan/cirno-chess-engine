use crate::{BoardPiece, Move, SquaresToEdge, ChessPieces, PieceColor};

/**
 * Unimplemented: What direction a piece can go in the pin.
 */
pub fn find_pinned_pieces_in_board(
    board: &[BoardPiece; 64],
    movement: &Vec<Move>,
    sqs_to_edge: &SquaresToEdge,
) -> Vec<(i16, ChessPieces, PieceColor)> {
    // We could run the diagonal traversal again, or label "pierced" moves
    //      and compile all of them here.

    let mut pinned_pieces: Vec<(i16, ChessPieces, PieceColor)> = vec![];
    for moves in movement {
        if matches!(
            board[moves.start_square as usize].piece_type,
            ChessPieces::Rooks | ChessPieces::Queens | ChessPieces::Bishops
        ) {
            let mut pinned =
                find_pinned_pieces_in_square(board, moves.start_square as usize, sqs_to_edge);
            pinned_pieces.extend(pinned.drain(..));
        }
    }
    pinned_pieces
}

pub fn find_pinned_pieces_in_square(
    board: &[BoardPiece; 64],
    start_square: usize,
    sqs_to_edge: &SquaresToEdge,
) -> Vec<(i16, ChessPieces, PieceColor)> {
    let start_piece = &board[start_square];
    let direction_offsets: [i16; 8] = [
        8, -8, -1, 1, // Up, Down, Left, Right
        7, -7, 9, -9, // Diagonals
    ];
    let (start_direction_index, end_direction_index) = match start_piece.piece_type {
        ChessPieces::Queens => (0, 8),
        ChessPieces::Bishops => (4, 8),
        ChessPieces::Rooks => (0, 4),
        _ => (0, 8),
    };
    // (piece direction, ..., ...)
    //
    // embed direction_offsets into this.
    let mut pinned_pieces: Vec<(i16, ChessPieces, PieceColor)> = Vec::new();

    for direction_index in start_direction_index..end_direction_index {
        // 8
        let mut path: Vec<(i16, ChessPieces, PieceColor)> = Vec::with_capacity(16);
        for n in 0..sqs_to_edge[start_square][direction_index] {
            // 8
            let target_square = start_square as i16 + direction_offsets[direction_index] * (n + 1);
            if let Some(target_piece) = board.get(target_square as usize) {
                if target_piece.piece_type != ChessPieces::Empty
                    && target_piece.piece_type != ChessPieces::Kings
                    && target_piece.piece_color != start_piece.piece_color
                {
                    path.push((
                        direction_offsets[direction_index],
                        target_piece.piece_type,
                        target_piece.piece_color,
                    ));
                    if path.len() > 1 {
                        break;
                    }
                }
                if target_piece.piece_type == ChessPieces::Kings
                    && target_piece.piece_color != start_piece.piece_color
                {
                    pinned_pieces.extend(path.drain(..));
                    break;
                }
            }
        }
    }

    pinned_pieces
}
