use crate::{BoardPiece, Castle, Move, PieceColor};

#[derive(Debug)]
pub struct ChessState {
    pub board: [BoardPiece; 64],
    pub color_to_move: PieceColor,
    pub is_able_to_castle: Castle,
    pub is_check: bool,
    pub pinned_pieces: Vec<BoardPiece>,
}

fn make_move(
    board: &mut [BoardPiece; 64],
    friendly_movements: &Vec<Move>,
    notation: &str,
) -> Result<(Move, BoardPiece), &'static str> {
    let (start_square_index, end_square_index) = algebraic_notation_decoder(notation);
    let moves = friendly_movements
        .iter()
        .find(|moves| (moves.start_square as u32, moves.target_square as u32) == (start_square_index, end_square_index));
    match moves {
        Some(moves) => {
            let piece = (moves.clone(), board[start_square_index as usize].clone());
            board[end_square_index as usize] = board[start_square_index as usize].clone();
            board[start_square_index as usize] = BoardPiece {
                ..Default::default() // Empty.
            };
            Ok(piece)
        }
        None => Err("Move not allowed."),
    }
}
// i gotta get info from the notation on the piece type, color, and position
//
// i could also return the previously made move from the make_move function.
fn unmake_move(board: &mut [BoardPiece; 64], piece_and_move: (Move, BoardPiece)) {
    let (move_, board_piece) = piece_and_move;
    board[move_.start_square as usize] = board_piece;
    board[move_.target_square as usize] = BoardPiece {
        ..Default::default() // Empty.
    };
}

fn algebraic_notation_decoder(notation: &str) -> (u32, u32) {
    let (notation_start, notation_end) = notation.split_at(2);
    let start_square_index = convert_algebraic_snippet(notation_start);
    let end_square_index = convert_algebraic_snippet(notation_end);
    (start_square_index, end_square_index)
}
fn convert_algebraic_snippet(notation: &str) -> u32 {
    let mut start_square_index = 0;
    for ch in notation.chars() {
        match ch {
            '1'..='8' => start_square_index += ch.to_digit(10).unwrap(), // yeah idk if this is
            // even right
            //
            // 1 indexed!! make sure the indexing is right.
            'A'..='H' => start_square_index *= map_char_to_number(ch).unwrap(),
            _ => {}
        }
    }
    start_square_index
}
fn map_char_to_number(c: char) -> Option<u32> {
    match c {
        'A' => Some(1),
        'B' => Some(2),
        'C' => Some(3),
        'D' => Some(4),
        'E' => Some(5),
        'F' => Some(6),
        'G' => Some(7),
        'H' => Some(8),
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
