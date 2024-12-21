use crate::attack_tables::{
    get_bishop_attacks, get_king_attacks, get_knight_attacks, get_pawn_attacks, get_queen_attacks,
    get_rook_attacks,
};
use crate::utils::{board_slice::BoardSlice, enums::*, errors::FENParseError};
use int_enum::IntEnum;
use std::fmt;
use std::str::FromStr;
use strum::IntoEnumIterator;

#[macro_export]
macro_rules! bitboard_piece_index {
    ($c: expr, $p: expr) => {
        ($c as usize) * 6 + ($p as usize)
    };
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Bitboard {
    pieces: [BoardSlice; 12],

    pub to_move: Color,

    castling_rights: u8,

    pub en_passant_square: Option<Square>,

    pub half_move_clock: usize,
    pub full_move_clock: usize,
}

impl Bitboard {
    pub fn get_piece(&self, color: Color, piece: Piece) -> BoardSlice {
        self.pieces[bitboard_piece_index!(color, piece)]
    }

    pub fn get_all_pieces(&self) -> BoardSlice {
        self.pieces
            .iter()
            .fold(BoardSlice(0), |acc, &x| BoardSlice(acc.0 | x.0))
    }

    pub fn get_color_pieces(&self, color: Color) -> BoardSlice {
        match color {
            Color::White => self
                .pieces
                .iter()
                .take(6)
                .fold(BoardSlice(0), |acc, &x| BoardSlice(acc.0 | x.0)),
            Color::Black => self
                .pieces
                .iter()
                .skip(6)
                .fold(BoardSlice(0), |acc, &x| BoardSlice(acc.0 | x.0)),
        }
    }

    pub fn is_square_attacked(&self, color: Color, square: Square) -> bool {
        ((get_pawn_attacks(color.opposite(), square).0 & self.get_piece(color, Piece::Pawn).0)
            | (get_knight_attacks(square).0 & self.get_piece(color, Piece::Knight).0)
            | (get_bishop_attacks(square, self.get_all_pieces()).0
                & self.get_piece(color, Piece::Bishop).0)
            | (get_rook_attacks(square, self.get_all_pieces()).0
                & self.get_piece(color, Piece::Rook).0)
            | (get_queen_attacks(square, self.get_all_pieces()).0
                & self.get_piece(color, Piece::Queen).0)
            | (get_king_attacks(square).0 & self.get_piece(color, Piece::King).0))
            != 0
    }

    pub fn get_king_square(&self, color: Color) -> Square {
        Square::from_int(self.get_piece(color, Piece::King).0.trailing_zeros() as u8).unwrap()
    }

    pub fn is_king_in_check(&self, color: Color) -> bool {
        self.is_square_attacked(color.opposite(), self.get_king_square(color))
    }

    pub fn has_castling_right(&self, cm: CastleMoves) -> bool {
        self.castling_rights & (cm as u8) != 0
    }

    pub fn add_piece(&mut self, color: Color, piece: Piece, square: Square) {
        self.pieces[bitboard_piece_index!(color, piece)].0 |= 1 << square as usize;
    }

    pub fn remove_piece(&mut self, color: Color, piece: Piece, square: Square) {
        self.pieces[bitboard_piece_index!(color, piece)].0 &= !(1 << square as usize);
    }

    pub fn move_piece(&mut self, color: Color, piece: Piece, orig: Square, dest: Square) {
        self.pieces[bitboard_piece_index!(color, piece)].0 &= !(1 << orig as usize);
        self.pieces[bitboard_piece_index!(color, piece)].0 |= 1 << dest as usize;
    }

    pub fn toggle_move(&mut self) {
        self.to_move = self.to_move.opposite();
    }

    pub fn remove_castling_right(&mut self, cm: CastleMoves) {
        self.castling_rights &= !(cm as u8);
    }

    pub fn to_str(&self) -> String {
        let mut fen = String::new();

        for i in (0..8).rev() {
            let mut blank_spaces = 0;
            for j in 0..8 {
                let mut found_piece = false;
                for color in Color::iter() {
                    for piece in Piece::iter() {
                        if self.get_piece(color, piece).0 & (1 << (i * 8 + j)) != 0 {
                            let piece_char = match piece {
                                Piece::Pawn => 'p',
                                Piece::Knight => 'n',
                                Piece::Bishop => 'b',
                                Piece::Rook => 'r',
                                Piece::Queen => 'q',
                                Piece::King => 'k',
                            };
                            if blank_spaces != 0 {
                                fen.push_str(&format!("{}", blank_spaces));
                                blank_spaces = 0;
                            }
                            fen.push(match color {
                                Color::White => piece_char.to_ascii_uppercase(),
                                Color::Black => piece_char,
                            });
                            found_piece = true;
                        }
                    }
                }
                if !found_piece {
                    blank_spaces += 1;
                }
            }
            if blank_spaces != 0 {
                fen.push_str(&format!("{}", blank_spaces));
            }
            fen.push('/');
        }
        fen.pop();

        fen.push(' ');

        fen.push(match self.to_move {
            Color::White => 'w',
            Color::Black => 'b',
        });

        fen.push(' ');

        if self.castling_rights == 0 {
            fen.push('-');
        } else {
            if self.castling_rights & CastleMoves::WhiteKingsideCastle as u8 != 0 {
                fen.push('K');
            }
            if self.castling_rights & CastleMoves::WhiteQueensideCastle as u8 != 0 {
                fen.push('Q');
            }
            if self.castling_rights & CastleMoves::BlackKingsideCastle as u8 != 0 {
                fen.push('k');
            }
            if self.castling_rights & CastleMoves::BlackQueensideCastle as u8 != 0 {
                fen.push('q');
            }
        }

        fen.push(' ');

        let en_passant_str = match self.en_passant_square {
            Some(square) => square.to_string().to_lowercase(),
            None => String::from("-"),
        };
        fen.push_str(&en_passant_str);

        fen.push(' ');

        fen.push_str(&format!("{} ", self.half_move_clock));
        fen.push_str(&format!("{}", self.full_move_clock));

        fen
    }
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

        let castling_rights = if fen_parts[2].chars().next() == Some('-') {
            0
        } else {
            fen_parts[2].chars().try_fold(0, |acc, c| match c {
                'K' => Ok(acc | CastleMoves::WhiteKingsideCastle as u8),
                'Q' => Ok(acc | CastleMoves::WhiteQueensideCastle as u8),
                'k' => Ok(acc | CastleMoves::BlackKingsideCastle as u8),
                'q' => Ok(acc | CastleMoves::BlackQueensideCastle as u8),
                _ => Err(FENParseError::IncorrectCastlingRights),
            })?
        };

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
    fn test_get_piece() {
        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let bitboard = position_fen.parse::<Bitboard>().unwrap();
        assert_eq!(
            bitboard.get_piece(Color::White, Piece::Rook),
            BoardSlice(0x0000000000000081)
        );

        assert_eq!(
            bitboard.get_piece(Color::Black, Piece::Pawn),
            BoardSlice(0x00FF000000000000)
        );
    }

    #[test]
    fn test_get_all_pieces() {
        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let bitboard = position_fen.parse::<Bitboard>().unwrap();

        assert_eq!(bitboard.get_all_pieces(), BoardSlice(0xFFFF00000000FFFF));
    }

    #[test]
    fn test_get_color_pieces() {
        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let bitboard = position_fen.parse::<Bitboard>().unwrap();

        assert_eq!(
            bitboard.get_color_pieces(Color::White),
            BoardSlice(0x000000000000FFFF)
        );
        assert_eq!(
            bitboard.get_color_pieces(Color::Black),
            BoardSlice(0xFFFF000000000000)
        );
    }

    #[test]
    fn test_get_king_square() {
        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let bitboard = position_fen.parse::<Bitboard>().unwrap();

        assert_eq!(bitboard.get_king_square(Color::White), Square::E1);
        assert_eq!(bitboard.get_king_square(Color::Black), Square::E8);
    }

    #[test]
    fn test_is_square_attacked() {
        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let bitboard = position_fen.parse::<Bitboard>().unwrap();

        assert_eq!(bitboard.is_square_attacked(Color::White, Square::E3), true);
        assert_eq!(bitboard.is_square_attacked(Color::White, Square::E4), false);

        assert_eq!(bitboard.is_square_attacked(Color::Black, Square::E6), true);
        assert_eq!(bitboard.is_square_attacked(Color::Black, Square::E5), false);

        assert_eq!(bitboard.is_square_attacked(Color::White, Square::E1), true);

        let position_fen = "k6q/8/8/8/7R/8/8/K6B w KQkq - 0 1";
        let bitboard = position_fen.parse::<Bitboard>().unwrap();

        assert_eq!(bitboard.is_square_attacked(Color::White, Square::D5), true);
        assert_eq!(bitboard.is_square_attacked(Color::White, Square::A3), false);

        assert_eq!(bitboard.is_square_attacked(Color::Black, Square::A7), true);
        assert_eq!(bitboard.is_square_attacked(Color::Black, Square::H1), false);
    }

    #[test]
    fn test_is_king_in_check() {
        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let bitboard = position_fen.parse::<Bitboard>().unwrap();

        assert_eq!(bitboard.is_king_in_check(Color::White), false);

        let position_fen = "k6q/8/8/8/7R/8/8/K6B w KQkq - 0 1";
        let bitboard = position_fen.parse::<Bitboard>().unwrap();

        assert_eq!(bitboard.is_king_in_check(Color::White), true);
        assert_eq!(bitboard.is_king_in_check(Color::Black), true);
    }

    #[test]
    fn test_move_piece() {
        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut bitboard = position_fen.parse::<Bitboard>().unwrap();

        bitboard.move_piece(Color::White, Piece::Queen, Square::D1, Square::E1);
        assert_eq!(
            bitboard.get_piece(Color::White, Piece::Queen),
            BoardSlice(0x10)
        );
    }

    #[test]
    fn test_toggle_move() {
        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut bitboard = position_fen.parse::<Bitboard>().unwrap();

        bitboard.toggle_move();
        assert_eq!(bitboard.to_move, Color::Black);

        bitboard.toggle_move();
        assert_eq!(bitboard.to_move, Color::White);
    }

    #[test]
    fn test_remove_castling_right() {
        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut bitboard = position_fen.parse::<Bitboard>().unwrap();

        bitboard.remove_castling_right(CastleMoves::WhiteKingsideCastle);
        assert_eq!(bitboard.castling_rights, 0b1110);
    }

    #[test]
    fn test_to_str_valid_1() {
        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let bitboard = position_fen.parse::<Bitboard>().unwrap();
        assert_eq!(bitboard.to_str(), position_fen);
    }

    #[test]
    fn test_to_str_valid_2() {
        let position_fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 1 1";
        let bitboard = position_fen.parse::<Bitboard>().unwrap();
        assert_eq!(bitboard.to_str(), position_fen);
    }

    #[test]
    fn test_from_str_valid_1() {
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
    fn test_from_str_valid_2() {
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
    fn test_from_str_errors() {
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
