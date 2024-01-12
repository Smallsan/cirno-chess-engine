use crate::types::{BoardPiece, ChessPieces, PieceColor, ChessState, Castle};
use std::collections::HashMap;

/**
 * Doesn't support the entire FEN string yet. Castling doesn't work yet.
 */
pub fn load_fen_state(fen: String) -> Result<ChessState, &'static str> {
    let fen: Vec<&str> = fen
        .trim()
        .split_whitespace()
        .collect();
    if fen.len() < 2 {
        return Err("Invalid FEN string");
    }
    
    let board = if let Some(fen_board) = fen.get(0) {
        load_position_from_fen(fen_board.to_string()) 
    } else {
        [(ChessPieces::Empty, PieceColor::None); 64]
    };

    let mut state = ChessState {
        board,
        color_to_move: PieceColor::None,
        is_able_to_castle: Castle { queenside: false, kingside: false },
    };

    for (i, &part) in fen.iter().skip(1).enumerate() {
        match i {
            0 => state.color_to_move = parse_turn(part),
            1 => state.is_able_to_castle = parse_castle(part).unwrap(),
            _ => return Err("Invalid/Unimplemented FEN string")
        }
    }
    Ok(state)
}

fn parse_turn(part: &str) -> PieceColor {
    match part.chars().nth(0).unwrap() {
        'w' => PieceColor::White,
        'b' => PieceColor::Black,
        _ => PieceColor::None,
    }
}

fn parse_castle(part: &str) -> Result<Castle, &'static str> {
    let mut castle = Castle {
        queenside: false,
        kingside: false,
    };

    for char in part.chars() {
        match char {
            'Q' | 'q' => {
                castle.queenside = true;
            }
            'K' | 'k' => {
                castle.kingside = true;
            }
            _ => return Err("Wrong input for Castles."),
        }
    }
    
    Ok(castle)
}

fn load_position_from_fen(fen: String) -> [BoardPiece; 64] {
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
    board
}
