use crate::errors::FENParseError;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

macro_rules! piece_index {
    ($c: expr, $p: expr) => {
        ($c as usize) * 6 + ($p as usize)
    };
}

#[derive(PartialEq, Eq)]
#[repr(u8)]
pub enum CastlingRights {
    WhiteKingsideCastle = 0b1,
    WhiteQueensideCastle = 0b10,
    BlackKingsideCastle = 0b100,
    BlackQueensideCastle = 0b1000,
}

#[derive(Debug, EnumString, PartialEq, Eq)]
#[repr(u8)]
pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Bitboard {
    pieces: [u64; 12],

    to_move: Color,

    castling_rights: u8,

    en_passant_square: Option<Square>,

    half_move_clock: usize,
    full_move_clock: usize,
}

impl FromStr for Bitboard {
    type Err = FENParseError;

    fn from_str(fen: &str) -> Result<Bitboard, FENParseError> {
        let fen_parts: Vec<&str> = fen.split_whitespace().collect();
        if fen_parts.len() != 6 {
            return Err(FENParseError::IncorrectPartsCount(fen_parts.len()));
        }

        let fen_board: Vec<&str> = fen_parts[0].split("/").collect();
        if fen_board.len() != 8 {
            return Err(FENParseError::IncorrectBoardLength(fen_board.len()));
        }

        let mut pieces = [0; 12];
        fen_board
            .iter()
            .rev()
            .enumerate()
            .try_for_each::<_, Result<(), FENParseError>>(|(row_index, &row)| {
                match row.chars().try_fold(row_index * 8, |acc, c| {
                    if acc >= (row_index + 1) * 8 {
                        return Err(FENParseError::IncorrectBoardRowLength(row_index + 1));
                    }
                    match c {
                        'P' => pieces[piece_index!(Color::White, Piece::Pawn)] |= 1 << acc,
                        'N' => pieces[piece_index!(Color::White, Piece::Knight)] |= 1 << acc,
                        'B' => pieces[piece_index!(Color::White, Piece::Bishop)] |= 1 << acc,
                        'R' => pieces[piece_index!(Color::White, Piece::Rook)] |= 1 << acc,
                        'Q' => pieces[piece_index!(Color::White, Piece::Queen)] |= 1 << acc,
                        'K' => pieces[piece_index!(Color::White, Piece::King)] |= 1 << acc,
                        'p' => pieces[piece_index!(Color::Black, Piece::Pawn)] |= 1 << acc,
                        'n' => pieces[piece_index!(Color::Black, Piece::Knight)] |= 1 << acc,
                        'b' => pieces[piece_index!(Color::Black, Piece::Bishop)] |= 1 << acc,
                        'r' => pieces[piece_index!(Color::Black, Piece::Rook)] |= 1 << acc,
                        'q' => pieces[piece_index!(Color::Black, Piece::Queen)] |= 1 << acc,
                        'k' => pieces[piece_index!(Color::Black, Piece::King)] |= 1 << acc,
                        '1'..='9' => {
                            if (c.to_digit(10).unwrap() as usize + acc) > (row_index + 1) * 8 {
                                return Err(FENParseError::IncorrectBoardRowLength(row_index + 1));
                            }
                            return Ok(acc + c.to_digit(10).unwrap() as usize);
                        }
                        other => return Err(FENParseError::IncorrectBoard(other)),
                    };
                    Ok(acc + 1)
                }) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err),
                }
            })?;

        let to_move = match fen_parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(FENParseError::IncorrectToMove),
        };

        let castling_rights = fen_parts[2].chars().try_fold(0 as u8, |acc, c| match c {
            'K' => Ok(acc | CastlingRights::WhiteKingsideCastle as u8),
            'Q' => Ok(acc | CastlingRights::WhiteQueensideCastle as u8),
            'k' => Ok(acc | CastlingRights::BlackKingsideCastle as u8),
            'q' => Ok(acc | CastlingRights::BlackQueensideCastle as u8),
            _ => Err(FENParseError::IncorrectCastlingRights),
        })?;

        let en_passant_square = match fen_parts[3].to_ascii_uppercase().as_str() {
            "-" => None,
            other => Some(
                other
                    .parse::<Square>()
                    .map_err(|_| FENParseError::IncorrectEnPassantSquare)?,
            ),
        };

        let half_move_clock = fen_parts[4]
            .parse::<usize>()
            .map_err(|_| FENParseError::IncorrectHalfMoveClock)?;

        let full_move_clock = fen_parts[5]
            .parse::<usize>()
            .map_err(|_| FENParseError::IncorrectFullMoveClock)?;

        return Ok(Bitboard {
            pieces,
            to_move,
            castling_rights,
            en_passant_square,
            half_move_clock,
            full_move_clock,
        });
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn from_str_valid_1() {
        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(
            position_fen.parse::<Bitboard>(),
            Ok(Bitboard {
                pieces: [
                    0x000000000000FF00,
                    0x0000000000000042,
                    0x0000000000000024,
                    0x0000000000000081,
                    0x0000000000000008,
                    0x0000000000000010,
                    0x00FF000000000000,
                    0x4200000000000000,
                    0x2400000000000000,
                    0x8100000000000000,
                    0x0800000000000000,
                    0x1000000000000000,
                ],
                to_move: Color::White,
                castling_rights: (CastlingRights::WhiteKingsideCastle as u8
                    | CastlingRights::WhiteQueensideCastle as u8
                    | CastlingRights::BlackKingsideCastle as u8
                    | CastlingRights::BlackQueensideCastle as u8),
                en_passant_square: None,
                half_move_clock: 0,
                full_move_clock: 1
            })
        );
    }

    #[test]
    fn from_str_valid_2() {
        let position_fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 1 1";
        assert_eq!(
            position_fen.parse::<Bitboard>(),
            Ok(Bitboard {
                pieces: [
                    0x000000001000EF00,
                    0x0000000000000042,
                    0x0000000000000024,
                    0x0000000000000081,
                    0x0000000000000008,
                    0x0000000000000010,
                    0x00FF000000000000,
                    0x4200000000000000,
                    0x2400000000000000,
                    0x8100000000000000,
                    0x0800000000000000,
                    0x1000000000000000,
                ],
                to_move: Color::Black,
                castling_rights: (CastlingRights::WhiteKingsideCastle as u8
                    | CastlingRights::WhiteQueensideCastle as u8
                    | CastlingRights::BlackKingsideCastle as u8
                    | CastlingRights::BlackQueensideCastle as u8),
                en_passant_square: Some(Square::E3),
                half_move_clock: 1,
                full_move_clock: 1
            })
        )
    }

    #[test]
    fn from_str_errors() {
        let position_fen = "rnbqkbnr/pppppppp/9/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(
            position_fen.parse::<Bitboard>(),
            Err(FENParseError::IncorrectBoardRowLength(6))
        );

        let position_fen = "rnbqkbnrr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(
            position_fen.parse::<Bitboard>(),
            Err(FENParseError::IncorrectBoardRowLength(8))
        );

        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNt w KQkq - 0 1";
        assert_eq!(
            position_fen.parse::<Bitboard>(),
            Err(FENParseError::IncorrectBoard('t'))
        );

        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1";
        assert_eq!(
            position_fen.parse::<Bitboard>(),
            Err(FENParseError::IncorrectToMove)
        );

        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w xKQkq - 0 1";
        assert_eq!(
            position_fen.parse::<Bitboard>(),
            Err(FENParseError::IncorrectCastlingRights)
        );

        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq e9 0 1";
        assert_eq!(
            position_fen.parse::<Bitboard>(),
            Err(FENParseError::IncorrectEnPassantSquare)
        );

        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - x 1";
        assert_eq!(
            position_fen.parse::<Bitboard>(),
            Err(FENParseError::IncorrectHalfMoveClock)
        );

        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 x";
        assert_eq!(
            position_fen.parse::<Bitboard>(),
            Err(FENParseError::IncorrectFullMoveClock)
        )
    }
}
