use crate::{
    types::{ChessPieces, MoveType},
    BoardPiece, Castle, Move, PieceColor,
};

#[derive(Debug, Clone)]
pub struct ChessState {
    pub board: [BoardPiece; 64],
    pub color_to_move: PieceColor,
    pub is_able_to_castle: Castle,
    pub is_check: bool,
    pub pinned_pieces: Vec<BoardPiece>,
}

/**
 * Castling no work yet!
 * Castling checks no work yet!
 *
 */
pub fn make_move(
    board: &mut [BoardPiece; 64],
    friendly_movements: &Vec<Move>,
    start_square_index: u32,
    end_square_index: u32,
) -> Result<(Move, BoardPiece, BoardPiece), &'static str> {
    let moves = friendly_movements.iter().find(|moves| {
        (moves.start_square as u32, moves.target_square as u32)
            == (start_square_index, end_square_index)
    });

    match moves {
        Some(moves) => {
            let start_piece = board[start_square_index as usize];
            let end_piece = board[end_square_index as usize];

            if moves.move_type == MoveType::NoCapture && end_piece.piece_type != ChessPieces::Empty
            {
                return Err("Move not allowed due to NoCapture pawn behaviour.");
            }
            // do unmake move later dear fucking god.
            if moves.move_type == MoveType::Castle {
                match start_piece.piece_color {
                    PieceColor::White => {
                        if end_square_index == 2 { // queenside
                            board[2] = start_piece.clone();
                            board[3] = board[0];
                            board[2] = BoardPiece { ..Default::default() };
                            board[0] = BoardPiece { ..Default::default() };
                        }
                        if end_square_index == 6 { // kingside
                            board[6] = start_piece.clone();
                            board[5] = board[7];
                            board[6] = BoardPiece { ..Default::default() };
                            board[7] = BoardPiece { ..Default::default() };
                        }
                    },
                    PieceColor::Black => {
                        // assuming rook positions to figure out queenside and kingside.
                        if end_square_index == 58 { // queenside
                            board[58] = start_piece.clone();
                            board[59] = board[56].clone();
                            board[58] = BoardPiece { ..Default::default() };
                            board[56] = BoardPiece { ..Default::default() };
                        }
                        if end_square_index == 62 { // kingside
                            board[62] = start_piece.clone();
                            board[61] = board[63].clone();
                            board[62] = BoardPiece { ..Default::default() };
                            board[63] = BoardPiece { ..Default::default() };
                        }
                    },
                    PieceColor::None => {}
                }
            }

            board[end_square_index as usize] = start_piece.clone();
            board[start_square_index as usize] = BoardPiece {
                ..Default::default() // Empty.
            };
            Ok((moves.clone(), start_piece.clone(), end_piece.clone()))
        }
        None => Err("Move not allowed."),
    }
}

pub fn unmake_move(
    board: &mut [BoardPiece; 64],
    piece_and_move: (Move, BoardPiece, BoardPiece),
) -> Result<(), &'static str> {
    let (move_, starting_piece, eaten_piece) = piece_and_move;

    if move_.start_square >= 64 || move_.target_square >= 64 {
        Err("Out of bounds!")
    } else {
        board[move_.start_square as usize] = starting_piece;
        board[move_.target_square as usize] = eaten_piece;
        Ok(())
    }
}

pub fn algebraic_notation_decoder(notation: &str) -> Result<(u32, u32), &'static str> {
    if notation.is_empty() {
        return Err("Empty notation!");
    }
    let (notation_start, notation_end) = notation.split_at(2);
    let start_square_index = convert_algebraic_snippet(notation_start)?;
    let end_square_index = convert_algebraic_snippet(notation_end)?;
    Ok((start_square_index, end_square_index))
}

pub fn convert_algebraic_snippet(notation: &str) -> Result<u32, &'static str> {
    let mut file = 0;
    let mut rank = 0;
    for ch in notation.chars() {
        match ch {
            '1'..='8' => rank = ch.to_digit(10).unwrap() - 1, // 0 indexed
            'A'..='H' | 'a'..='h' => file = map_char_to_number(ch).unwrap() - 1, // 0 indexed
            '\r' | '\n' => {}
            _ => return Err("Invalid notation!"),
        }
    }
    Ok(rank * 8 + file)
}

fn map_char_to_number(c: char) -> Option<u32> {
    match c {
        'A' | 'a' => Some(1),
        'B' | 'b' => Some(2),
        'C' | 'c' => Some(3),
        'D' | 'd' => Some(4),
        'E' | 'e' => Some(5),
        'F' | 'f' => Some(6),
        'G' | 'g' => Some(7),
        'H' | 'h' => Some(8),
        _ => None, // Handle other characters if needed
    }
}

impl Default for ChessState {
    fn default() -> ChessState {
        ChessState {
            board: [BoardPiece {
                ..Default::default()
            }; 64],
            color_to_move: PieceColor::Black,
            is_able_to_castle: Default::default(),
            pinned_pieces: vec![],
            is_check: false,
        }
    }
}
