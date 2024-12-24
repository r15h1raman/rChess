use std::usize;

use self::{
    bishop_attack_generators::{generate_bishop_attack_mask, generate_bishop_attacks_on_the_fly},
    magic_number_constants::{
        BISHOP_ATTACK_MASKS, BISHOP_MAGIC_NUMBERS, BISHOP_MASK_BIT_COUNT, ROOK_ATTACK_MASKS,
        ROOK_MAGIC_NUMBERS, ROOK_MASK_BIT_COUNT,
    },
    occupancy::get_occupancy,
    rook_attack_generators::{generate_rook_attack_mask, generate_rook_attacks_on_the_fly},
};
use crate::utils::{
    board_slice::BoardSlice,
    enums::{Color, Square},
};

use lazy_static::lazy_static;
use strum::IntoEnumIterator;

mod bishop_attack_generators;
mod magic_number_constants;
mod magic_number_generator;
mod occupancy;
mod rook_attack_generators;

lazy_static! {
    static ref WHITE_PAWN_MOVE_TABLE: [BoardSlice; 64] = generate_pawn_move_table(Color::White);
    static ref BLACK_PAWN_MOVE_TABLE: [BoardSlice; 64] = generate_pawn_move_table(Color::Black);
    static ref WHITE_PAWN_ATTACK_TABLE: [BoardSlice; 64] = generate_pawn_attack_table(Color::White);
    static ref BLACK_PAWN_ATTACK_TABLE: [BoardSlice; 64] = generate_pawn_attack_table(Color::Black);
    static ref WHITE_DOUBLE_PAWN_MOVE_TABLE: [BoardSlice; 64] =
        generate_double_pawn_move_table(Color::White);
    static ref BLACK_DOUBLE_PAWN_MOVE_TABLE: [BoardSlice; 64] =
        generate_double_pawn_move_table(Color::Black);
    static ref WHITE_INVERSE_DOUBLE_PAWN_MOVE_TABLE: [BoardSlice; 64] =
        generate_inverse_double_pawn_move_table(Color::White);
    static ref BLACK_INVERSE_DOUBLE_PAWN_MOVE_TABLE: [BoardSlice; 64] =
        generate_inverse_double_pawn_move_table(Color::Black);
    static ref KNIGHT_ATTACK_TABLE: [BoardSlice; 64] = generate_knight_attack_table();
    static ref KING_ATTACK_TABLE: [BoardSlice; 64] = generate_king_attack_table();
    static ref BISHOP_ATTACK_TABLE: Vec<[BoardSlice; 512]> = generate_bishop_attack_table();
    static ref ROOK_ATTACK_TABLE: Vec<[BoardSlice; 4096]> = generate_rook_attack_table();
}

#[allow(clippy::needless_range_loop)]
fn generate_pawn_move_table(color: Color) -> [BoardSlice; 64] {
    let mut attack_table = [BoardSlice(0); 64];
    match color {
        Color::White => {
            for i in 1..7 {
                for j in 0..8 {
                    attack_table[i * 8 + j].0 |= 1 << (i * 8 + j + 8);
                }
            }
        }
        Color::Black => {
            for i in 1..7 {
                for j in 0..8 {
                    attack_table[i * 8 + j].0 |= 1 << (i * 8 + j - 8);
                }
            }
        }
    }

    attack_table
}

#[allow(clippy::needless_range_loop)]
fn generate_double_pawn_move_table(color: Color) -> [BoardSlice; 64] {
    let mut attack_table = [BoardSlice(0); 64];
    match color {
        Color::White => {
            for j in 0..8 {
                attack_table[8 + j].0 |= 1 << (8 + j + 16);
            }
        }
        Color::Black => {
            for j in 0..8 {
                attack_table[48 + j].0 |= 1 << (48 + j - 16);
            }
        }
    }
    attack_table
}

#[allow(clippy::needless_range_loop)]
fn generate_pawn_attack_table(color: Color) -> [BoardSlice; 64] {
    let mut attack_table = [BoardSlice(0); 64];
    match color {
        Color::White => {
            for i in 0..7 {
                for j in 0..7 {
                    attack_table[i * 8 + j].0 |= 1 << (i * 8 + j + 9);
                }
                for j in 1..8 {
                    attack_table[i * 8 + j].0 |= 1 << (i * 8 + j + 7);
                }
            }
        }
        Color::Black => {
            for i in 1..8 {
                for j in 0..7 {
                    attack_table[i * 8 + j].0 |= 1 << (i * 8 + j - 7);
                }
                for j in 1..8 {
                    attack_table[i * 8 + j].0 |= 1 << (i * 8 + j - 9);
                }
            }
        }
    }

    attack_table
}

#[allow(clippy::needless_range_loop)]
fn generate_inverse_double_pawn_move_table(color: Color) -> [BoardSlice; 64] {
    let mut attack_table = [BoardSlice(0); 64];
    match color {
        Color::White => {
            for j in 0..8 {
                attack_table[3 * 8 + j].0 |= 1 << (8 + j);
            }
        }
        Color::Black => {
            for j in 0..8 {
                attack_table[4 * 8 + j].0 |= 1 << (6 * 8 + j);
            }
        }
    }

    attack_table
}

#[allow(clippy::needless_range_loop)]
fn generate_knight_attack_table() -> [BoardSlice; 64] {
    let mut attack_table = [BoardSlice(0); 64];

    // NNE
    for i in 0..6 {
        for j in 0..7 {
            attack_table[i * 8 + j].0 |= 1 << (i * 8 + j + 17);
        }
    }
    // NEE
    for i in 0..7 {
        for j in 0..6 {
            attack_table[i * 8 + j].0 |= 1 << (i * 8 + j + 10);
        }
    }
    // SEE
    for i in 1..8 {
        for j in 0..6 {
            attack_table[i * 8 + j].0 |= 1 << (i * 8 + j - 6);
        }
    }
    // SSE
    for i in 2..8 {
        for j in 0..7 {
            attack_table[i * 8 + j].0 |= 1 << (i * 8 + j - 15);
        }
    }
    // SSW
    for i in 2..8 {
        for j in 1..8 {
            attack_table[i * 8 + j].0 |= 1 << (i * 8 + j - 17);
        }
    }
    // SWW
    for i in 1..8 {
        for j in 2..8 {
            attack_table[i * 8 + j].0 |= 1 << (i * 8 + j - 10);
        }
    }
    // NWW
    for i in 0..7 {
        for j in 2..8 {
            attack_table[i * 8 + j].0 |= 1 << (i * 8 + j + 6);
        }
    }
    // NNW
    for i in 0..6 {
        for j in 1..8 {
            attack_table[i * 8 + j].0 |= 1 << (i * 8 + j + 15);
        }
    }
    attack_table
}

#[allow(clippy::needless_range_loop)]
fn generate_king_attack_table() -> [BoardSlice; 64] {
    let mut attack_table = [BoardSlice(0); 64];

    // N
    for i in 0..7 {
        for j in 0..8 {
            attack_table[i * 8 + j].0 |= 1 << (i * 8 + j + 8);
        }
    }
    // NE
    for i in 0..7 {
        for j in 0..7 {
            attack_table[i * 8 + j].0 |= 1 << (i * 8 + j + 9);
        }
    }
    // E
    for i in 0..8 {
        for j in 0..7 {
            attack_table[i * 8 + j].0 |= 1 << (i * 8 + j + 1);
        }
    }
    // SE
    for i in 1..8 {
        for j in 0..7 {
            attack_table[i * 8 + j].0 |= 1 << (i * 8 + j - 7);
        }
    }
    // S
    for i in 1..8 {
        for j in 0..8 {
            attack_table[i * 8 + j].0 |= 1 << (i * 8 + j - 8);
        }
    }
    // SW
    for i in 1..8 {
        for j in 1..8 {
            attack_table[i * 8 + j].0 |= 1 << (i * 8 + j - 9);
        }
    }
    // W
    for i in 0..8 {
        for j in 1..8 {
            attack_table[i * 8 + j].0 |= 1 << (i * 8 + j - 1);
        }
    }
    // NW
    for i in 0..7 {
        for j in 1..8 {
            attack_table[i * 8 + j].0 |= 1 << (i * 8 + j + 7);
        }
    }

    attack_table
}

fn generate_bishop_attack_table() -> Vec<[BoardSlice; 512]> {
    let mut attack_table = vec![[BoardSlice(0); 512]; 64];

    for square in Square::iter() {
        let attack_mask = generate_bishop_attack_mask(square);
        let relevant_bits = BISHOP_MASK_BIT_COUNT[square as usize];

        for i in 0..(1 << relevant_bits) {
            let occupancy = get_occupancy(i, relevant_bits, attack_mask);
            let magic_index = ((occupancy
                .0
                .wrapping_mul(BISHOP_MAGIC_NUMBERS[square as usize]))
                >> (64 - relevant_bits)) as usize;

            attack_table[square as usize][magic_index] =
                generate_bishop_attacks_on_the_fly(square, occupancy);
        }
    }
    attack_table
}

fn generate_rook_attack_table() -> Vec<[BoardSlice; 4096]> {
    let mut attack_table = vec![[BoardSlice(0); 4096]; 64];

    for square in Square::iter() {
        let attack_mask = generate_rook_attack_mask(square);
        let relevant_bits = ROOK_MASK_BIT_COUNT[square as usize];

        for i in 0..(1 << relevant_bits) {
            let occupancy = get_occupancy(i, relevant_bits, attack_mask);
            let magic_index = ((occupancy
                .0
                .wrapping_mul(ROOK_MAGIC_NUMBERS[square as usize]))
                >> (64 - relevant_bits)) as usize;

            attack_table[square as usize][magic_index] =
                generate_rook_attacks_on_the_fly(square, occupancy);
        }
    }
    attack_table
}

pub fn get_pawn_moves(color: Color, square: Square) -> BoardSlice {
    match color {
        Color::White => WHITE_PAWN_MOVE_TABLE[square as usize],
        Color::Black => BLACK_PAWN_MOVE_TABLE[square as usize],
    }
}

pub fn get_pawn_attacks(color: Color, square: Square) -> BoardSlice {
    match color {
        Color::White => WHITE_PAWN_ATTACK_TABLE[square as usize],
        Color::Black => BLACK_PAWN_ATTACK_TABLE[square as usize],
    }
}

pub fn get_double_pawn_moves(color: Color, square: Square) -> BoardSlice {
    match color {
        Color::White => WHITE_DOUBLE_PAWN_MOVE_TABLE[square as usize],
        Color::Black => BLACK_DOUBLE_PAWN_MOVE_TABLE[square as usize],
    }
}

pub fn get_inverse_double_pawn_moves(color: Color, square: Square) -> BoardSlice {
    match color {
        Color::White => WHITE_INVERSE_DOUBLE_PAWN_MOVE_TABLE[square as usize],
        Color::Black => BLACK_INVERSE_DOUBLE_PAWN_MOVE_TABLE[square as usize],
    }
}

pub fn get_knight_attacks(square: Square) -> BoardSlice {
    KNIGHT_ATTACK_TABLE[square as usize]
}

pub fn get_king_attacks(square: Square) -> BoardSlice {
    KING_ATTACK_TABLE[square as usize]
}

pub fn get_bishop_attacks(square: Square, blockers: BoardSlice) -> BoardSlice {
    let occupancy = blockers.0 & BISHOP_ATTACK_MASKS[square as usize].0;
    let magic_index = ((occupancy.wrapping_mul(BISHOP_MAGIC_NUMBERS[square as usize]))
        >> (64 - BISHOP_MASK_BIT_COUNT[square as usize])) as usize;
    BISHOP_ATTACK_TABLE[square as usize][magic_index]
}

pub fn get_rook_attacks(square: Square, blockers: BoardSlice) -> BoardSlice {
    let occupancy = blockers.0 & ROOK_ATTACK_MASKS[square as usize].0;
    let magic_index = ((occupancy.wrapping_mul(ROOK_MAGIC_NUMBERS[square as usize]))
        >> (64 - ROOK_MASK_BIT_COUNT[square as usize])) as usize;
    ROOK_ATTACK_TABLE[square as usize][magic_index]
}

pub fn get_queen_attacks(square: Square, blockers: BoardSlice) -> BoardSlice {
    BoardSlice(get_bishop_attacks(square, blockers).0 | get_rook_attacks(square, blockers).0)
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::utils::enums::Square;

    #[test]
    fn test_pawn_move_table() {
        assert_eq!(get_pawn_moves(Color::White, Square::A1), BoardSlice(0));
        assert_eq!(
            get_pawn_moves(Color::White, Square::D2),
            BoardSlice(1 << Square::D3 as usize)
        );
        assert_eq!(
            get_pawn_moves(Color::White, Square::E6),
            BoardSlice(0x10000000000000)
        );
        assert_eq!(get_pawn_moves(Color::White, Square::H8), BoardSlice(0));

        assert_eq!(get_pawn_moves(Color::Black, Square::A8), BoardSlice(0));
        assert_eq!(
            get_pawn_moves(Color::Black, Square::D7),
            BoardSlice(1 << Square::D6 as usize)
        );
        assert_eq!(get_pawn_moves(Color::Black, Square::E3), BoardSlice(0x1000));
        assert_eq!(get_pawn_moves(Color::Black, Square::H1), BoardSlice(0));
    }

    #[test]
    fn test_pawn_attack_table() {
        assert_eq!(
            get_pawn_attacks(Color::White, Square::A1),
            BoardSlice(0x200)
        );
        assert_eq!(
            get_pawn_attacks(Color::White, Square::A2),
            BoardSlice(0x20000)
        );
        assert_eq!(
            get_pawn_attacks(Color::White, Square::D4),
            BoardSlice(0x1400000000)
        );
        assert_eq!(
            get_pawn_attacks(Color::White, Square::H6),
            BoardSlice(0x40000000000000)
        );
        assert_eq!(get_pawn_attacks(Color::White, Square::H8), BoardSlice(0));

        assert_eq!(
            get_pawn_attacks(Color::Black, Square::A8),
            BoardSlice(0x2000000000000)
        );
        assert_eq!(
            get_pawn_attacks(Color::Black, Square::A7),
            BoardSlice(0x20000000000)
        );
        assert_eq!(
            get_pawn_attacks(Color::Black, Square::D5),
            BoardSlice(0x14000000)
        );
        assert_eq!(
            get_pawn_attacks(Color::Black, Square::H3),
            BoardSlice(0x4000)
        );
        assert_eq!(get_pawn_attacks(Color::Black, Square::H1), BoardSlice(0));
    }

    #[test]
    fn test_double_pawn_move_table() {
        assert_eq!(
            get_double_pawn_moves(Color::White, Square::D2),
            BoardSlice(1 << Square::D4 as usize)
        );
        assert_eq!(
            get_double_pawn_moves(Color::White, Square::D1),
            BoardSlice(0)
        );
        assert_eq!(
            get_double_pawn_moves(Color::Black, Square::D7),
            BoardSlice(1 << Square::D5 as usize)
        );
        assert_eq!(
            get_double_pawn_moves(Color::White, Square::D8),
            BoardSlice(0)
        );
    }

    #[test]
    fn test_inverse_double_pawn_move_table() {
        assert_eq!(
            get_inverse_double_pawn_moves(Color::White, Square::B4),
            BoardSlice(1 << Square::B2 as usize)
        );
        assert_eq!(
            get_inverse_double_pawn_moves(Color::White, Square::G4),
            BoardSlice(1 << Square::G2 as usize)
        );
        assert_eq!(
            get_inverse_double_pawn_moves(Color::Black, Square::B5),
            BoardSlice(1 << Square::B7 as usize)
        );
        assert_eq!(
            get_inverse_double_pawn_moves(Color::Black, Square::G5),
            BoardSlice(1 << Square::G7 as usize)
        );
        assert_eq!(
            get_inverse_double_pawn_moves(Color::White, Square::B3),
            BoardSlice(0)
        );
    }

    #[test]
    fn test_knight_attack_table() {
        assert_eq!(get_knight_attacks(Square::A1), BoardSlice(0x20400));
        assert_eq!(get_knight_attacks(Square::H8), BoardSlice(0x20400000000000));
        assert_eq!(get_knight_attacks(Square::E4), BoardSlice(0x284400442800));
        assert_eq!(get_knight_attacks(Square::B2), BoardSlice(0x5080008));
        assert_eq!(
            get_knight_attacks(Square::G7),
            BoardSlice(0x100010a000000000)
        );
    }

    #[test]
    fn test_king_attack_table() {
        assert_eq!(get_king_attacks(Square::A1), BoardSlice(0x302));
        assert_eq!(get_king_attacks(Square::H8), BoardSlice(0x40c0000000000000));
        assert_eq!(get_king_attacks(Square::E4), BoardSlice(0x3828380000));
        assert_eq!(get_king_attacks(Square::B2), BoardSlice(0x70507));
        assert_eq!(get_king_attacks(Square::G7), BoardSlice(0xe0a0e00000000000));
    }

    #[test]
    fn test_get_bishop_attacks() {
        let square = Square::D3;
        let blockers =
            BoardSlice(1 << Square::C2 as u64 | 1 << Square::E2 as u64 | 1 << Square::B5 as u64);

        assert_eq!(
            get_bishop_attacks(square, blockers),
            BoardSlice(0x80402214001400)
        );

        let square = Square::E4;
        let blockers = BoardSlice(
            1 << Square::E3 as u64
                | 1 << Square::D4 as u64
                | 1 << Square::E5 as u64
                | 1 << Square::F4 as u64,
        );
        assert_eq!(
            get_bishop_attacks(square, blockers),
            BoardSlice(0x182442800284482)
        );

        let square = Square::C6;
        let blockers = BoardSlice(
            1 << Square::E3 as u64
                | 1 << Square::G2 as u64
                | 1 << Square::E4 as u64
                | 1 << Square::B7 as u64,
        );
        assert_eq!(
            get_bishop_attacks(square, blockers),
            BoardSlice(0x100a000a11000000)
        );
    }

    #[test]
    fn test_get_rook_attacks() {
        let square = Square::E4;
        let blockers = BoardSlice(
            1 << Square::E3 as u64
                | 1 << Square::D4 as u64
                | 1 << Square::F4 as u64
                | 1 << Square::E7 as u64,
        );
        assert_eq!(
            get_rook_attacks(square, blockers),
            BoardSlice(0x10101028100000)
        );

        let square = Square::C3;
        let blockers = BoardSlice(
            1 << Square::C5 as u64
                | 1 << Square::C7 as u64
                | 1 << Square::G7 as u64
                | 1 << Square::G3 as u64,
        );
        assert_eq!(get_rook_attacks(square, blockers), BoardSlice(0x4047b0404));

        let square = Square::D4;
        let blockers = BoardSlice(
            1 << Square::C3 as u64
                | 1 << Square::C5 as u64
                | 1 << Square::E5 as u64
                | 1 << Square::E3 as u64,
        );
        assert_eq!(
            get_rook_attacks(square, blockers),
            BoardSlice(0x8080808f7080808)
        );
    }

    #[test]
    fn test_get_queen_attacks() {
        let square = Square::E4;
        let blockers = BoardSlice(
            1 << Square::E2 as u64
                | 1 << Square::G2 as u64
                | 1 << Square::G4 as u64
                | 1 << Square::G6 as u64
                | 1 << Square::E6 as u64,
        );

        assert_eq!(
            get_queen_attacks(square, blockers),
            BoardSlice(0x10254386f385402)
        );
    }
}
