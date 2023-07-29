use crate::utils::{board_slice::BoardSlice, enums::Color};

#[allow(clippy::needless_range_loop)]
pub fn generate_pawn_attack_table(color: Color) -> [[BoardSlice; 8]; 8] {
    let mut attack_table = [[BoardSlice(0); 8]; 8];
    match color {
        Color::White => {
            for i in 1..7 {
                for j in 0..8 {
                    attack_table[i][j].0 |= 1 << (i * 8 + j + 8);
                }
            }
            for j in 0..8 {
                attack_table[1][j].0 |= 1 << (8 + j + 16);
            }
        }
        Color::Black => {
            for i in 1..7 {
                for j in 0..8 {
                    attack_table[i][j].0 |= 1 << (i * 8 + j - 8);
                }
            }
            for j in 0..8 {
                attack_table[6][j].0 |= 1 << (48 + j - 16);
            }
        }
    }

    attack_table
}

#[allow(clippy::needless_range_loop)]
pub fn generate_knight_attack_table() -> [[BoardSlice; 8]; 8] {
    let mut attack_table = [[BoardSlice(0); 8]; 8];

    // NNE
    for i in 0..6 {
        for j in 0..7 {
            attack_table[i][j].0 |= 1 << (i * 8 + j + 17);
        }
    }
    // NEE
    for i in 0..7 {
        for j in 0..6 {
            attack_table[i][j].0 |= 1 << (i * 8 + j + 10);
        }
    }
    // SEE
    for i in 1..8 {
        for j in 0..6 {
            attack_table[i][j].0 |= 1 << (i * 8 + j - 6);
        }
    }
    // SSE
    for i in 2..8 {
        for j in 0..7 {
            attack_table[i][j].0 |= 1 << (i * 8 + j - 15);
        }
    }
    // SSW
    for i in 2..8 {
        for j in 1..8 {
            attack_table[i][j].0 |= 1 << (i * 8 + j - 17);
        }
    }
    // SWW
    for i in 1..8 {
        for j in 2..8 {
            attack_table[i][j].0 |= 1 << (i * 8 + j - 10);
        }
    }
    // NWW
    for i in 0..7 {
        for j in 2..8 {
            attack_table[i][j].0 |= 1 << (i * 8 + j + 6);
        }
    }
    // NNW
    for i in 0..6 {
        for j in 1..8 {
            attack_table[i][j].0 |= 1 << (i * 8 + j + 15);
        }
    }
    attack_table
}

#[allow(clippy::needless_range_loop)]
pub fn generate_king_attack_table() -> [[BoardSlice; 8]; 8] {
    let mut attack_table = [[BoardSlice(0); 8]; 8];

    // N
    for i in 0..7 {
        for j in 0..8 {
            attack_table[i][j].0 |= 1 << (i * 8 + j + 8);
        }
    }
    // NE
    for i in 0..7 {
        for j in 0..7 {
            attack_table[i][j].0 |= 1 << (i * 8 + j + 9);
        }
    }
    // E
    for i in 0..8 {
        for j in 0..7 {
            attack_table[i][j].0 |= 1 << (i * 8 + j + 1);
        }
    }
    // SE
    for i in 1..8 {
        for j in 0..7 {
            attack_table[i][j].0 |= 1 << (i * 8 + j - 7);
        }
    }
    // S
    for i in 1..8 {
        for j in 0..8 {
            attack_table[i][j].0 |= 1 << (i * 8 + j - 8);
        }
    }
    // SW
    for i in 1..8 {
        for j in 1..8 {
            attack_table[i][j].0 |= 1 << (i * 8 + j - 9);
        }
    }
    // W
    for i in 0..8 {
        for j in 1..8 {
            attack_table[i][j].0 |= 1 << (i * 8 + j - 1);
        }
    }
    // NW
    for i in 0..7 {
        for j in 1..8 {
            attack_table[i][j].0 |= 1 << (i * 8 + j + 7);
        }
    }

    attack_table
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::utils::enums::{File, Rank};

    #[test]
    fn pawn_attack_table_valid() {
        let white_attack_table = generate_pawn_attack_table(Color::White);
        let black_attack_table = generate_pawn_attack_table(Color::Black);
        assert_eq!(
            white_attack_table[Rank::Rank1 as usize][File::AFile as usize],
            BoardSlice(0)
        );
        assert_eq!(
            white_attack_table[Rank::Rank2 as usize][File::DFile as usize],
            BoardSlice(0x8080000)
        );
        assert_eq!(
            white_attack_table[Rank::Rank6 as usize][File::EFile as usize],
            BoardSlice(0x10000000000000)
        );
        assert_eq!(
            white_attack_table[Rank::Rank8 as usize][File::HFile as usize],
            BoardSlice(0)
        );
        assert_eq!(
            black_attack_table[Rank::Rank8 as usize][File::AFile as usize],
            BoardSlice(0)
        );
        assert_eq!(
            black_attack_table[Rank::Rank7 as usize][File::DFile as usize],
            BoardSlice(0x80800000000)
        );
        assert_eq!(
            black_attack_table[Rank::Rank2 as usize][File::EFile as usize],
            BoardSlice(0x10)
        );
        assert_eq!(
            black_attack_table[Rank::Rank1 as usize][File::HFile as usize],
            BoardSlice(0)
        );
    }

    #[test]
    fn knight_attack_table_valid() {
        let attack_table = generate_knight_attack_table();
        assert_eq!(
            attack_table[Rank::Rank1 as usize][File::AFile as usize],
            BoardSlice(0x20400)
        );
        assert_eq!(
            attack_table[Rank::Rank8 as usize][File::HFile as usize],
            BoardSlice(0x20400000000000)
        );
        assert_eq!(
            attack_table[Rank::Rank4 as usize][File::EFile as usize],
            BoardSlice(0x284400442800)
        );
        assert_eq!(
            attack_table[Rank::Rank2 as usize][File::BFile as usize],
            BoardSlice(0x5080008)
        );
        assert_eq!(
            attack_table[Rank::Rank7 as usize][File::GFile as usize],
            BoardSlice(0x100010a000000000)
        );
    }

    #[test]
    fn king_attack_table_valid() {
        let attack_table = generate_king_attack_table();
        assert_eq!(
            attack_table[Rank::Rank1 as usize][File::AFile as usize],
            BoardSlice(0x302)
        );
        assert_eq!(
            attack_table[Rank::Rank8 as usize][File::HFile as usize],
            BoardSlice(0x40c0000000000000)
        );
        assert_eq!(
            attack_table[Rank::Rank4 as usize][File::EFile as usize],
            BoardSlice(0x3828380000)
        );
        assert_eq!(
            attack_table[Rank::Rank2 as usize][File::BFile as usize],
            BoardSlice(0x70507)
        );
        assert_eq!(
            attack_table[Rank::Rank7 as usize][File::GFile as usize],
            BoardSlice(0xe0a0e00000000000)
        );
    }
}
