use crate::utils::{
    board_slice::BoardSlice,
    enums::{Color, Square},
};

#[allow(clippy::needless_range_loop)]
pub fn generate_pawn_attack_table(color: Color) -> [BoardSlice; 64] {
    let mut attack_table = [BoardSlice(0); 64];
    match color {
        Color::White => {
            for i in 1..7 {
                for j in 0..8 {
                    attack_table[i * 8 + j].0 |= 1 << (i * 8 + j + 8);
                }
            }
            for j in 0..8 {
                attack_table[8 + j].0 |= 1 << (8 + j + 16);
            }
        }
        Color::Black => {
            for i in 1..7 {
                for j in 0..8 {
                    attack_table[i * 8 + j].0 |= 1 << (i * 8 + j - 8);
                }
            }
            for j in 0..8 {
                attack_table[48 + j].0 |= 1 << (48 + j - 16);
            }
        }
    }

    attack_table
}

#[allow(clippy::needless_range_loop)]
pub fn generate_knight_attack_table() -> [BoardSlice; 64] {
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
pub fn generate_king_attack_table() -> [BoardSlice; 64] {
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

pub fn generate_bishop_attack_mask(square: Square) -> BoardSlice {
    let mut attack_mask = BoardSlice(0);

    let rank = square as i64 / 8;
    let file = square as i64 % 8;

    for (i, j) in ((rank + 1)..7).zip((file + 1)..7) {
        attack_mask.0 |= 1 << (i * 8 + j);
    }
    for (i, j) in (1..rank).rev().zip((file + 1)..7) {
        attack_mask.0 |= 1 << (i * 8 + j);
    }
    for (i, j) in (1..rank).rev().zip((1..file).rev()) {
        attack_mask.0 |= 1 << (i * 8 + j);
    }
    for (i, j) in ((rank + 1)..7).zip((1..file).rev()) {
        attack_mask.0 |= 1 << (i * 8 + j);
    }

    attack_mask
}

pub fn generate_rook_attack_mask(square: Square) -> BoardSlice {
    let mut attack_mask = BoardSlice(0);

    let rank = square as i64 / 8;
    let file = square as i64 % 8;

    for i in (rank + 1)..7 {
        attack_mask.0 |= 1 << (i * 8 + file);
    }
    for j in (file + 1)..7 {
        attack_mask.0 |= 1 << (rank * 8 + j);
    }
    for i in 1..rank {
        attack_mask.0 |= 1 << (i * 8 + file);
    }
    for j in 1..file {
        attack_mask.0 |= 1 << (rank * 8 + j);
    }

    attack_mask
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::utils::enums::Square;

    #[test]
    fn pawn_attack_table_valid() {
        let white_attack_table = generate_pawn_attack_table(Color::White);
        let black_attack_table = generate_pawn_attack_table(Color::Black);
        assert_eq!(white_attack_table[Square::A1 as usize], BoardSlice(0));
        assert_eq!(
            white_attack_table[Square::D2 as usize],
            BoardSlice(0x8080000)
        );
        assert_eq!(
            white_attack_table[Square::E6 as usize],
            BoardSlice(0x10000000000000)
        );
        assert_eq!(white_attack_table[Square::H8 as usize], BoardSlice(0));
        assert_eq!(black_attack_table[Square::A8 as usize], BoardSlice(0));
        assert_eq!(
            black_attack_table[Square::D7 as usize],
            BoardSlice(0x80800000000)
        );
        assert_eq!(black_attack_table[Square::E2 as usize], BoardSlice(0x10));
        assert_eq!(black_attack_table[Square::H1 as usize], BoardSlice(0));
    }

    #[test]
    fn knight_attack_table_valid() {
        let attack_table = generate_knight_attack_table();
        assert_eq!(attack_table[Square::A1 as usize], BoardSlice(0x20400));
        assert_eq!(
            attack_table[Square::H8 as usize],
            BoardSlice(0x20400000000000)
        );
        assert_eq!(
            attack_table[Square::E4 as usize],
            BoardSlice(0x284400442800)
        );
        assert_eq!(attack_table[Square::B2 as usize], BoardSlice(0x5080008));
        assert_eq!(
            attack_table[Square::G7 as usize],
            BoardSlice(0x100010a000000000)
        );
    }

    #[test]
    fn king_attack_table_valid() {
        let attack_table = generate_king_attack_table();
        assert_eq!(attack_table[Square::A1 as usize], BoardSlice(0x302));
        assert_eq!(
            attack_table[Square::H8 as usize],
            BoardSlice(0x40c0000000000000)
        );
        assert_eq!(attack_table[Square::E4 as usize], BoardSlice(0x3828380000));
        assert_eq!(attack_table[Square::B2 as usize], BoardSlice(0x70507));
        assert_eq!(
            attack_table[Square::G7 as usize],
            BoardSlice(0xe0a0e00000000000)
        );
    }

    #[test]
    fn bishop_attack_mask_valid() {
        assert_eq!(
            generate_bishop_attack_mask(Square::A1),
            BoardSlice(0x40201008040200)
        );
        assert_eq!(
            generate_bishop_attack_mask(Square::D4),
            BoardSlice(0x40221400142200)
        );
        assert_eq!(
            generate_bishop_attack_mask(Square::F4),
            BoardSlice(0x4085000500800)
        );
        assert_eq!(
            generate_bishop_attack_mask(Square::E2),
            BoardSlice(0x244280000)
        );
    }

    #[test]
    fn rook_attack_mask_valid() {
        assert_eq!(
            generate_rook_attack_mask(Square::A1),
            BoardSlice(0x101010101017e)
        );
        assert_eq!(
            generate_rook_attack_mask(Square::D4),
            BoardSlice(0x8080876080800)
        );
        assert_eq!(
            generate_rook_attack_mask(Square::F4),
            BoardSlice(0x2020205e202000)
        );
        assert_eq!(
            generate_rook_attack_mask(Square::E2),
            BoardSlice(0x10101010106e00)
        );
    }
}
