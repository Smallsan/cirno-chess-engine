use crate::chess_state::ChessState;
use crate::types::{BoardPiece, Castle, ChessPieces, PieceColor};

/**
 * Doesn't support the entire FEN string yet.
 * Halfmoves (essential for tracking stalemates) don't work yet.
 */
pub fn load_fen_state(fen: String) -> Result<ChessState, &'static str> {
    let fen: Vec<&str> = fen.trim().split_whitespace().collect();
    if fen.len() < 2 {
        return Err("Invalid FEN string");
    }

    let board = if let Some(fen_board) = fen.get(0) {
        load_position_from_fen(fen_board.to_string())?
    } else {
        [Default::default(); 64]
    };

    let mut state = ChessState {
        board,
        ..Default::default()
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
    let mut board: [BoardPiece; 64] = [Default::default(); 64];
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
                let piece_type = match letter {
                    'P' => Ok(BoardPiece { piece_type: ChessPieces::Pawns, piece_color: PieceColor::White }),
                    'K' => Ok(BoardPiece { piece_type: ChessPieces::Kings, piece_color: PieceColor::White }),
                    'Q' => Ok(BoardPiece { piece_type: ChessPieces::Queens, piece_color: PieceColor::White }),
                    'B' => Ok(BoardPiece { piece_type: ChessPieces::Bishops, piece_color: PieceColor::White }),
                    'N' => Ok(BoardPiece { piece_type: ChessPieces::Knights, piece_color: PieceColor::White }),
                    'R' => Ok(BoardPiece { piece_type: ChessPieces::Rooks, piece_color: PieceColor::White }),

                    'p' => Ok(BoardPiece { piece_type: ChessPieces::Pawns, piece_color: PieceColor::Black }),
                    'k' => Ok(BoardPiece { piece_type: ChessPieces::Kings, piece_color: PieceColor::Black }),
                    'q' => Ok(BoardPiece { piece_type: ChessPieces::Queens, piece_color: PieceColor::Black }),
                    'b' => Ok(BoardPiece { piece_type: ChessPieces::Bishops, piece_color: PieceColor::Black }),
                    'n' => Ok(BoardPiece { piece_type: ChessPieces::Knights, piece_color: PieceColor::Black }),
                    'r' => Ok(BoardPiece { piece_type: ChessPieces::Rooks, piece_color: PieceColor::Black }),
                    _ => Err("You probably got a wrong letter in your fen string."),
                };
                let index = usize::try_from(rank * 8 + file).unwrap();
                match piece_type {
                    Ok(piece) => board[index] = piece.to_owned(),
                    Err(err) => return Err(err),
                }
                
                file += 1;
            }
        }
    }
    Ok(board)
}
