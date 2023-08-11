use crate::utils::{board_slice::BoardSlice, enums::*, errors::FENParseError};
use std::fmt;
use std::str::FromStr;
use strum::IntoEnumIterator;

macro_rules! bitboard_piece_index {
    ($c: expr, $p: expr) => {
        ($c as usize) * 6 + ($p as usize)
    };
}

#[derive(Debug, PartialEq, Eq)]
pub struct Bitboard {
    pieces: [BoardSlice; 12],

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

        let fen_board: Vec<&str> = fen_parts[0].split('/').collect();
        if fen_board.len() != 8 {
            return Err(FENParseError::IncorrectBoardLength(fen_board.len()));
        }

        let mut pieces = [BoardSlice(0); 12];
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
                        'P' => {
                            pieces[bitboard_piece_index!(Color::White, Piece::Pawn)].0 |= 1 << acc
                        }
                        'N' => {
                            pieces[bitboard_piece_index!(Color::White, Piece::Knight)].0 |= 1 << acc
                        }
                        'B' => {
                            pieces[bitboard_piece_index!(Color::White, Piece::Bishop)].0 |= 1 << acc
                        }
                        'R' => {
                            pieces[bitboard_piece_index!(Color::White, Piece::Rook)].0 |= 1 << acc
                        }
                        'Q' => {
                            pieces[bitboard_piece_index!(Color::White, Piece::Queen)].0 |= 1 << acc
                        }
                        'K' => {
                            pieces[bitboard_piece_index!(Color::White, Piece::King)].0 |= 1 << acc
                        }
                        'p' => {
                            pieces[bitboard_piece_index!(Color::Black, Piece::Pawn)].0 |= 1 << acc
                        }
                        'n' => {
                            pieces[bitboard_piece_index!(Color::Black, Piece::Knight)].0 |= 1 << acc
                        }
                        'b' => {
                            pieces[bitboard_piece_index!(Color::Black, Piece::Bishop)].0 |= 1 << acc
                        }
                        'r' => {
                            pieces[bitboard_piece_index!(Color::Black, Piece::Rook)].0 |= 1 << acc
                        }
                        'q' => {
                            pieces[bitboard_piece_index!(Color::Black, Piece::Queen)].0 |= 1 << acc
                        }
                        'k' => {
                            pieces[bitboard_piece_index!(Color::Black, Piece::King)].0 |= 1 << acc
                        }
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

        let castling_rights = fen_parts[2].chars().try_fold(0, |acc, c| match c {
            'K' => Ok(acc | CastleMoves::WhiteKingsideCastle as u8),
            'Q' => Ok(acc | CastleMoves::WhiteQueensideCastle as u8),
            'k' => Ok(acc | CastleMoves::BlackKingsideCastle as u8),
            'q' => Ok(acc | CastleMoves::BlackQueensideCastle as u8),
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

        Ok(Bitboard {
            pieces,
            to_move,
            castling_rights,
            en_passant_square,
            half_move_clock,
            full_move_clock,
        })
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in (0..8).rev() {
            write!(f, "{}  ", i + 1)?;
            for j in 0..8 {
                let mut empty = true;
                for (board_i, &slice) in self.pieces.iter().enumerate() {
                    if (slice.0 & (1 << (i * 8 + j))) != 0 {
                        empty = false;
                        match board_i {
                            0 => write!(f, " ♙")?,
                            1 => write!(f, " ♘")?,
                            2 => write!(f, " ♗")?,
                            3 => write!(f, " ♖")?,
                            4 => write!(f, " ♕")?,
                            5 => write!(f, " ♔")?,
                            6 => write!(f, " ♟")?,
                            7 => write!(f, " ♞")?,
                            8 => write!(f, " ♝")?,
                            9 => write!(f, " ♜")?,
                            10 => write!(f, " ♛")?,
                            11 => write!(f, " ♚")?,
                            _ => (),
                        }
                    }
                }
                if empty {
                    write!(f, " ·")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "\n    a b c d e f g h\n")?;

        writeln!(f, "To Move: {:?}", self.to_move)?;
        writeln!(
            f,
            "CR: {}",
            CastleMoves::iter()
                .map(|cr| {
                    if self.castling_rights & cr as u8 != 0 {
                        match cr {
                            CastleMoves::WhiteKingsideCastle => 'K',
                            CastleMoves::WhiteQueensideCastle => 'Q',
                            CastleMoves::BlackKingsideCastle => 'k',
                            CastleMoves::BlackQueensideCastle => 'q',
                        }
                    } else {
                        '\0'
                    }
                })
                .collect::<String>()
        )?;

        match self.en_passant_square {
            Some(square) => writeln!(f, "EP Square: {:?}", square),
            None => writeln!(f, "EP Square: None"),
        }?;

        writeln!(
            f,
            "FMC: {}, HMC: {}",
            self.full_move_clock, self.half_move_clock
        )?;
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::utils::errors::FENParseError;

    #[test]
    fn from_str_valid_1() {
        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(
            position_fen.parse::<Bitboard>(),
            Ok(Bitboard {
                pieces: [
                    BoardSlice(0x000000000000FF00),
                    BoardSlice(0x0000000000000042),
                    BoardSlice(0x0000000000000024),
                    BoardSlice(0x0000000000000081),
                    BoardSlice(0x0000000000000008),
                    BoardSlice(0x0000000000000010),
                    BoardSlice(0x00FF000000000000),
                    BoardSlice(0x4200000000000000),
                    BoardSlice(0x2400000000000000),
                    BoardSlice(0x8100000000000000),
                    BoardSlice(0x0800000000000000),
                    BoardSlice(0x1000000000000000),
                ],
                to_move: Color::White,
                castling_rights: (CastleMoves::WhiteKingsideCastle as u8
                    | CastleMoves::WhiteQueensideCastle as u8
                    | CastleMoves::BlackKingsideCastle as u8
                    | CastleMoves::BlackQueensideCastle as u8),
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
                    BoardSlice(0x000000001000EF00),
                    BoardSlice(0x0000000000000042),
                    BoardSlice(0x0000000000000024),
                    BoardSlice(0x0000000000000081),
                    BoardSlice(0x0000000000000008),
                    BoardSlice(0x0000000000000010),
                    BoardSlice(0x00FF000000000000),
                    BoardSlice(0x4200000000000000),
                    BoardSlice(0x2400000000000000),
                    BoardSlice(0x8100000000000000),
                    BoardSlice(0x0800000000000000),
                    BoardSlice(0x1000000000000000),
                ],
                to_move: Color::Black,
                castling_rights: (CastleMoves::WhiteKingsideCastle as u8
                    | CastleMoves::WhiteQueensideCastle as u8
                    | CastleMoves::BlackKingsideCastle as u8
                    | CastleMoves::BlackQueensideCastle as u8),
                en_passant_square: Some(Square::E3),
                half_move_clock: 1,
                full_move_clock: 1
            })
        )
    }

    #[test]
    fn from_str_errors() {
        let position_fen = "1 2 3 4 5";
        assert_eq!(
            position_fen.parse::<Bitboard>(),
            Err(FENParseError::IncorrectPartsCount(5))
        );

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
