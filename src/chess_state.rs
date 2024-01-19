use crate::{
    types::{ChessPieces, MoveType},
    BoardPiece, Castle, Move, PieceColor,
};

#[derive(Debug)]
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
    notation: &str,
) -> Result<(Move, BoardPiece, BoardPiece), &'static str> {
    // separate this from make_move
    let (start_square_index, end_square_index) = algebraic_notation_decoder(notation)?;
    let moves = friendly_movements.iter().find(|moves| {
        (moves.start_square as u32, moves.target_square as u32)
            == (start_square_index, end_square_index)
    });

    match moves {
        Some(moves) => {
            let piece = (
                moves.clone(),
                board[start_square_index as usize].clone(),
                board[end_square_index as usize].clone(),
            );

            if piece.0.move_type == MoveType::NoCapture && piece.2.piece_type != ChessPieces::Empty
            {
                return Err("Move not allowed due to NoCapture pawn behaviour.");
            }

            board[end_square_index as usize] = board[start_square_index as usize].clone();
            board[start_square_index as usize] = BoardPiece {
                ..Default::default() // Empty.
            };
            Ok(piece)
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

fn algebraic_notation_decoder(notation: &str) -> Result<(u32, u32), &'static str> {
    if notation.is_empty() {
        return Err("Empty notation!");
    }
    let (notation_start, notation_end) = notation.split_at(2);
    let start_square_index = convert_algebraic_snippet(notation_start)?;
    let end_square_index = convert_algebraic_snippet(notation_end)?;
    Ok((start_square_index, end_square_index))
}

fn convert_algebraic_snippet(notation: &str) -> Result<u32, &'static str> {
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
