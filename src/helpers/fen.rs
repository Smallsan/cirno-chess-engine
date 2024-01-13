use crate::types::{BoardPiece, Castle, ChessPieces, ChessState, PieceColor, SquaresToEdge};
use std::collections::HashMap;

/**
 * Doesn't support the entire FEN string yet. Castling doesn't work yet.
 */
pub fn load_fen_state(
    fen: String,
) -> Result<ChessState, &'static str> {
    let fen: Vec<&str> = fen.trim().split_whitespace().collect();
    if fen.len() < 2 {
        return Err("Invalid FEN string");
    }

    let board = if let Some(fen_board) = fen.get(0) {
        load_position_from_fen(fen_board.to_string())?
    } else {
        [(ChessPieces::Empty, PieceColor::None); 64]
    };

    let mut state = ChessState {
        board,
        color_to_move: PieceColor::None,
        is_able_to_castle: Castle {
            queenside: false,
            kingside: false,
        },
        pinned_pieces: vec![],
    };

    for (i, &part) in fen.iter().skip(1).enumerate() {
        match i {
            0 => state.color_to_move = parse_turn(part)?,
            1 => state.is_able_to_castle = parse_castle(part, state.color_to_move)?,
            _ => {}
        }
    }
    Ok(state)
}

fn parse_turn(part: &str) -> Result<PieceColor, &'static str> {
    match part.chars().nth(0).unwrap_or_else(|| 'm') {
        'w' => Ok(PieceColor::White),
        'b' => Ok(PieceColor::Black),
        _ => Err("Wrong input for turns."),
    }
}

fn parse_castle(part: &str, color: PieceColor) -> Result<Castle, &'static str> {
    let mut castle = Castle {
        queenside: false,
        kingside: false,
    };

    for char in part.chars() {
        if char == 'Q' && color == PieceColor::White || char == 'q' && color == PieceColor::Black {
            castle.queenside = true;
        }
        if char == 'K' && color == PieceColor::White || char == 'k' && color == PieceColor::Black {
            castle.kingside = true;
        }
    }
    Ok(castle)
}

fn load_position_from_fen(fen: String) -> Result<[BoardPiece; 64], &'static str> {
    let mut board = [(ChessPieces::Empty, PieceColor::None); 64];
    let piece_type_from_symbol: HashMap<char, BoardPiece> = HashMap::from([
        ('r', (ChessPieces::Rooks, PieceColor::Black)),
        ('n', (ChessPieces::Knights, PieceColor::Black)),
        ('b', (ChessPieces::Bishops, PieceColor::Black)),
        ('q', (ChessPieces::Queens, PieceColor::Black)),
        ('k', (ChessPieces::Kings, PieceColor::Black)),
        ('p', (ChessPieces::Pawns, PieceColor::Black)),
        ('R', (ChessPieces::Rooks, PieceColor::White)),
        ('N', (ChessPieces::Knights, PieceColor::White)),
        ('B', (ChessPieces::Bishops, PieceColor::White)),
        ('Q', (ChessPieces::Queens, PieceColor::White)),
        ('K', (ChessPieces::Kings, PieceColor::White)),
        ('P', (ChessPieces::Pawns, PieceColor::White)),
    ]);

    let mut file = 0;
    let mut rank = 7;

    for letter in fen.chars() {
        if letter == '/' {
            file = 0;
            rank -= 1;
        } else {
            if letter.is_digit(10) {
                // movement | 8
                file += letter.to_digit(10).unwrap();
            } else {
                let piece_type = piece_type_from_symbol.get(&letter).unwrap();
                let index = usize::try_from(rank * 8 + file).unwrap();
                board[index] = piece_type.to_owned();
                file += 1;
            }
        }
    }
    Ok(board)
}
