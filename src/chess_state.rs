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
    pub en_passant_target: Option<i16>,
}

/**
 * Clone chess state and return the modified chess state.
 */
pub fn make_move(
    state: &ChessState,
    friendly_movements: &Vec<Move>,
    start_square_index: u32,
    end_square_index: u32,
) -> Result<ChessState, &'static str> {
    let moves = friendly_movements.iter().find(|moves| {
        (moves.start_square as u32, moves.target_square as u32)
            == (start_square_index, end_square_index)
    });

    let mut state = state.clone();

    match moves {
        Some(moves) => {
            let start_piece = state.board[start_square_index as usize];
            let end_piece = state.board[end_square_index as usize];

            if moves.move_type == MoveType::NoCapture && end_piece.piece_type != ChessPieces::Empty
            {
                return Err("Move not allowed due to NoCapture pawn behaviour.");
            }
            // do unmake move later dear fucking god.
            if moves.move_type == MoveType::Castle {
                match start_piece.piece_color {
                    PieceColor::White => {
                        if end_square_index == 2 { // queenside
                            state.board[2] = start_piece.clone();
                            state.board[3] = state.board[0];
                            state.board[2] = BoardPiece { ..Default::default() };
                            state.board[0] = BoardPiece { ..Default::default() };
                        }
                        if end_square_index == 6 { // kingside
                            state.board[6] = start_piece.clone();
                            state.board[5] = state.board[7];
                            state.board[6] = BoardPiece { ..Default::default() };
                            state.board[7] = BoardPiece { ..Default::default() };
                        }
                    },
                    PieceColor::Black => {
                        // assuming rook positions to figure out queenside and kingside.
                        if end_square_index == 58 { // queenside
                            state.board[58] = start_piece.clone();
                            state.board[59] = state.board[56].clone();
                            state.board[58] = BoardPiece { ..Default::default() };
                            state.board[56] = BoardPiece { ..Default::default() };
                        }
                        if end_square_index == 62 { // kingside
                            state.board[62] = start_piece.clone();
                            state.board[61] = state.board[63].clone();
                            state.board[62] = BoardPiece { ..Default::default() };
                            state.board[63] = BoardPiece { ..Default::default() };
                        }
                    },
                    PieceColor::None => {}
                }
            }

            state.board[end_square_index as usize] = start_piece.clone();
            state.board[start_square_index as usize] = BoardPiece {
                ..Default::default() // Empty.
            };
            // just clone fen_state to keep track of the previous state
            Ok(state)
        }
        None => Err("Move not allowed."),
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
            en_passant_target: None,
        }
    }
}
