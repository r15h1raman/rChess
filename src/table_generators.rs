use crate::utils::board_slice::BoardSlice;

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

#[cfg(test)]
pub mod tests {
    use crate::utils::enums::{File, Rank};

    use super::*;

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
}
