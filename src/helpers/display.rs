use crate::chess_state::ChessState;
use crate::types::{BoardPiece, ChessPieces, Move, MoveType, PieceColor};

pub fn display_chess_tui(state: &ChessState, movement: &Vec<Move>) {
    let turn_color = match state.color_to_move {
        PieceColor::White => "White",
        PieceColor::Black => "Black",
        PieceColor::None => "Something errored out.",
    };

    let mut print_index = 1;
    let mut position = 0;
    print!("\n{turn_color}'s turn\n");

    for square in state.board {
        let newline = if print_index % 8 == 0 {
            print_index = 0;
            "\n"
        } else {
            ""
        };

        let mut castling_moves = find_castling_moves(movement);
        castling_moves.extend(movement); // dirty hack by Small <3
                                         // Queens don't get displayed for no reason without this.

        let move_str = if let Some(mo) = castling_moves.iter().find(|x| x.target_square == position)
        {
            match mo.move_type {
                MoveType::NoCapture => "%",
                MoveType::Normal => "*",
                MoveType::Castle => "&",
                MoveType::EnPassant => "x",
                MoveType::Promotion => "!",
                MoveType::Piercing => "?",
            }
        } else {
            " "
        };

        let mut piece = format_piece(square);
        piece.insert_str(0, move_str);
        print!("[{}]{}", piece, newline);
        print_index += 1;
        position += 1;
    }
    print!("\n");
}

fn find_castling_moves(moves: &Vec<Move>) -> Vec<&Move> {
    moves
        .iter()
        .filter(|&&mov| mov.move_type == MoveType::Castle)
        .collect()
}

pub fn format_piece(square: BoardPiece) -> String {
    return match square.piece_color {
        PieceColor::White => match square.piece_type {
            ChessPieces::Kings => format!("K"),
            ChessPieces::Queens => format!("Q"),
            ChessPieces::Rooks => format!("R"),
            ChessPieces::Bishops => format!("B"),
            ChessPieces::Knights => format!("N"),
            ChessPieces::Pawns => format!("P"),
            ChessPieces::Empty => format!(" "),
        },
        PieceColor::Black => match square.piece_type {
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
    };
}
