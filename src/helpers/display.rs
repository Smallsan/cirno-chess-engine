
use crate::types::{Chess, Move, ChessPieces, PieceColor};

pub fn display_chess_tui(board: &[Chess; 64], movement: &Vec<Move>) {
    let mut print_index = 1;
    let mut position = 0;

    for square in board {
        let newline = if print_index % 8 == 0 {
            print_index = 0;
            "\n"
        } else {
            ""
        };

        let attack = if let Some(_) = movement.iter().find(|x| x.target_square == position) {
            "*"
        } else {
            " "
        };

        let piece = match square.1 {
            PieceColor::White => match square.0 {
                ChessPieces::Kings => format!("{attack}K"),
                ChessPieces::Queens => format!("{attack}Q"),
                ChessPieces::Rooks => format!("{attack}R"),
                ChessPieces::Bishops => format!("{attack}B"),
                ChessPieces::Knights => format!("{attack}N"),
                ChessPieces::Pawns => format!("{attack}P"),
                ChessPieces::Empty => format!("{attack} "),
            },
            PieceColor::Black => match square.0 {
                ChessPieces::Kings => format!("{attack}k"),
                ChessPieces::Queens => format!("{attack}q"),
                ChessPieces::Rooks => format!("{attack}r"),
                ChessPieces::Bishops => format!("{attack}b"),
                ChessPieces::Knights => format!("{attack}n"),
                ChessPieces::Pawns => format!("{attack}p"),
                ChessPieces::Empty => format!("{attack} "),
            },
            PieceColor::None => {
                format!("{attack} ")
            }
        };
        print!("[{}]{}", piece, newline);
        print_index += 1;
        position += 1;
    }
}
