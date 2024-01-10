
use crate::types::{Move, ChessPieces, PieceColor, ChessState, BoardPiece};

pub fn display_chess_tui(state: &ChessState, movement: &Vec<Move>) {
    let mut print_index = 1;
    let mut position = 0;

    let turn_color = match state.color_to_move {
        PieceColor::White => "White",
        PieceColor::Black => "Black",
        PieceColor::None => "Something errored out."
    };
    print!("\n{turn_color}'s turn\n");
    for square in state.board {
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

        let mut piece = format_piece(square);
        piece.insert_str(0, attack);
        print!("[{}]{}", piece, newline);
        print_index += 1;
        position += 1;
    }
    print!("\n");
}

pub fn format_piece(square: BoardPiece) -> String {
    return match square.1 {
        PieceColor::White => match square.0 {
            ChessPieces::Kings => format!("K"),
            ChessPieces::Queens => format!("Q"),
            ChessPieces::Rooks => format!("R"),
            ChessPieces::Bishops => format!("B"),
            ChessPieces::Knights => format!("N"),
            ChessPieces::Pawns => format!("P"),
            ChessPieces::Empty => format!(" "),
        },
        PieceColor::Black => match square.0 {
            ChessPieces::Kings => format!("k"),
            ChessPieces::Queens => format!("q"),
            ChessPieces::Rooks => format!("r"),
            ChessPieces::Bishops => format!("b"),
            ChessPieces::Knights => format!("n"),
            ChessPieces::Pawns => format!("p"),
            ChessPieces::Empty => format!(" "),
        },
        PieceColor::None => {
            format!(" ")
        }
    }
}
