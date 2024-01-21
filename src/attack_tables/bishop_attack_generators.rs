use crate::utils::{board_slice::BoardSlice, enums::Square};

pub fn generate_bishop_attack_mask(square: Square) -> BoardSlice {
    let mut attack_mask = BoardSlice(0);

    let rank = square as usize / 8;
    let file = square as usize % 8;

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

pub fn generate_bishop_attacks_on_the_fly(square: Square, blockers: BoardSlice) -> BoardSlice {
    let mut attacks = BoardSlice(0);

    let rank = square as usize / 8;
    let file = square as usize % 8;

    for (i, j) in ((rank + 1)..8).zip((file + 1)..8) {
        attacks.0 |= 1 << (i * 8 + j);
        if (blockers.0 & (1 << (i * 8 + j))) != 0 {
            break;
        }
    }
    for (i, j) in (0..rank).rev().zip((file + 1)..8) {
        attacks.0 |= 1 << (i * 8 + j);
        if blockers.0 & (1 << (i * 8 + j)) != 0 {
            break;
        }
    }
    for (i, j) in (0..rank).rev().zip((0..file).rev()) {
        attacks.0 |= 1 << (i * 8 + j);
        if blockers.0 & (1 << (i * 8 + j)) != 0 {
            break;
        }
    }
    for (i, j) in ((rank + 1)..8).zip((0..file).rev()) {
        attacks.0 |= 1 << (i * 8 + j);
        if blockers.0 & (1 << (i * 8 + j)) != 0 {
            break;
        }
    }

    attacks
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_bishop_attack_mask() {
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
    fn test_bishop_attacks_on_the_fly() {
        assert_eq!(
            generate_bishop_attacks_on_the_fly(
                Square::D4,
                BoardSlice(
                    (1 << Square::B2 as usize)
                        | (1 << Square::E3 as usize)
                        | (1 << Square::F6 as usize)
                )
            ),
            BoardSlice(0x1221400140200)
        );

        assert_eq!(
            generate_bishop_attacks_on_the_fly(
                Square::F3,
                BoardSlice(
                    (1 << Square::E2 as usize)
                        | (1 << Square::G2 as usize)
                        | (1 << Square::G4 as usize)
                )
            ),
            BoardSlice(0x102040850005000)
        );
    }
}
