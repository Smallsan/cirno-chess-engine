use crate::chess_state::ChessState;
use crate::types::{BoardPiece, ChessPieces, Move, MoveType, PieceColor};

// https://crates.io/crates/colored
use colored::*;

pub fn display_chess_tui(state: &ChessState, movement: &Vec<Move>) {
    let turn_color = match state.color_to_move {
        PieceColor::White => "White",
        PieceColor::Black => "Black",
        PieceColor::None => "Something errored out.",
    };

    let mut print_index = 1;
    let mut position = 0;
    print!("\n{turn_color}'s turn\n");

    for i in 0..8 {
        let letter = match i {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => unreachable!(),
        };
        print!(" {} ", letter.truecolor(105, 105, 105))
    }
    println!("");
    for square in state.board {
        let newline = if print_index % 8 == 0 {
            format!(
                " {}\n",
                (print_index / 8).to_string().truecolor(105, 105, 105)
            )
        } else {
            String::from("")
        };

        let mut castling_moves = find_castling_moves(movement);
        castling_moves.extend(movement); // dirty hack by Small <3
                                         // Queens don't get displayed for no reason without this.

        let (r, g, b) =
            if let Some(mo) = castling_moves.iter().find(|x| x.target_square == position) {
                match mo.move_type {
                    MoveType::NoCapture => (146, 254, 144),
                    MoveType::Normal => (146, 254, 144),
                    MoveType::Castle => (255, 80, 80),
                    MoveType::EnPassant => (30, 0, 77),
                    MoveType::Promotion => (255, 255, 255),
                    MoveType::Piercing => (255, 255, 255),
                }
            } else {
                (100, 100, 100)
            };

        let piece = format_piece(square);
        let sq = format!(
            "{}{}{}",
            "[".truecolor(r, g, b),
            piece,
            "]".truecolor(r, g, b)
        );

        print!("{}{}", sq, newline);
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

pub fn format_piece(square: BoardPiece) -> ColoredString {
    return match square.piece_color {
        PieceColor::White => match square.piece_type {
            ChessPieces::Kings => format!("K"),
            ChessPieces::Queens => format!("Q"),
            ChessPieces::Rooks => format!("R"),
            ChessPieces::Bishops => format!("B"),
            ChessPieces::Knights => format!("N"),
            ChessPieces::Pawns => format!("P"),
            ChessPieces::Empty => format!(" "),
        }.red(),
        PieceColor::Black => match square.piece_type {
            ChessPieces::Kings => format!("k"),
            ChessPieces::Queens => format!("q"),
            ChessPieces::Rooks => format!("r"),
            ChessPieces::Bishops => format!("b"),
            ChessPieces::Knights => format!("n"),
            ChessPieces::Pawns => format!("p"),
            ChessPieces::Empty => format!(" "),
        }.blue(),
        PieceColor::None => ColoredString::from(format!(" ")),
    };
}
