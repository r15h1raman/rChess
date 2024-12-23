use std::usize;

use int_enum::IntEnum;
use strum_macros::{Display, EnumIter, EnumString, ToString};

use super::board_slice::BoardSlice;

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, EnumIter)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
pub enum CastleMoves {
    WhiteKingsideCastle = 0b1,
    WhiteQueensideCastle = 0b10,
    BlackKingsideCastle = 0b100,
    BlackQueensideCastle = 0b1000,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter, EnumString, IntEnum, ToString)]
#[repr(u8)]
pub enum Square {
    A1 = 0,
    B1 = 1,
    C1 = 2,
    D1 = 3,
    E1 = 4,
    F1 = 5,
    G1 = 6,
    H1 = 7,
    A2 = 8,
    B2 = 9,
    C2 = 10,
    D2 = 11,
    E2 = 12,
    F2 = 13,
    G2 = 14,
    H2 = 15,
    A3 = 16,
    B3 = 17,
    C3 = 18,
    D3 = 19,
    E3 = 20,
    F3 = 21,
    G3 = 22,
    H3 = 23,
    A4 = 24,
    B4 = 25,
    C4 = 26,
    D4 = 27,
    E4 = 28,
    F4 = 29,
    G4 = 30,
    H4 = 31,
    A5 = 32,
    B5 = 33,
    C5 = 34,
    D5 = 35,
    E5 = 36,
    F5 = 37,
    G5 = 38,
    H5 = 39,
    A6 = 40,
    B6 = 41,
    C6 = 42,
    D6 = 43,
    E6 = 44,
    F6 = 45,
    G6 = 46,
    H6 = 47,
    A7 = 48,
    B7 = 49,
    C7 = 50,
    D7 = 51,
    E7 = 52,
    F7 = 53,
    G7 = 54,
    H7 = 55,
    A8 = 56,
    B8 = 57,
    C8 = 58,
    D8 = 59,
    E8 = 60,
    F8 = 61,
    G8 = 62,
    H8 = 63,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, IntEnum)]
#[repr(u8)]
pub enum File {
    AFile = 0,
    BFile = 1,
    CFile = 2,
    DFile = 3,
    EFile = 4,
    FFile = 5,
    GFile = 6,
    HFile = 7,
}

pub fn file_mask(file: File) -> BoardSlice {
    BoardSlice(0x0101010101010101 << file as usize)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, IntEnum)]
#[repr(u8)]
pub enum Rank {
    Rank1 = 0,
    Rank2 = 1,
    Rank3 = 2,
    Rank4 = 3,
    Rank5 = 4,
    Rank6 = 5,
    Rank7 = 6,
    Rank8 = 7,
}

pub fn rank_mask(rank: Rank) -> BoardSlice {
    BoardSlice(0xFF << 8 * rank as usize)
}
