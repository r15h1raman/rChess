use crate::utils::{board_slice::BoardSlice, enums::Square};

pub fn generate_rook_attack_mask(square: Square) -> BoardSlice {
    let mut attack_mask = BoardSlice(0);

    let rank = square as usize / 8;
    let file = square as usize % 8;

    for i in (rank + 1)..7 {
        attack_mask.0 |= 1 << (i * 8 + file);
    }
    for j in (file + 1)..7 {
        attack_mask.0 |= 1 << (rank * 8 + j);
    }
    for i in (1..rank).rev() {
        attack_mask.0 |= 1 << (i * 8 + file);
    }
    for j in (1..file).rev() {
        attack_mask.0 |= 1 << (rank * 8 + j);
    }

    attack_mask
}

pub fn generate_rook_attacks_on_the_fly(square: Square, blockers: BoardSlice) -> BoardSlice {
    let mut attacks = BoardSlice(0);

    let rank = square as usize / 8;
    let file = square as usize % 8;

    for i in (rank + 1)..8 {
        attacks.0 |= 1 << (i * 8 + file);
        if (blockers.0 & (1 << (i * 8 + file))) != 0 {
            break;
        }
    }
    for j in (file + 1)..8 {
        attacks.0 |= 1 << (rank * 8 + j);
        if (blockers.0 & (1 << (rank * 8 + j))) != 0 {
            break;
        }
    }
    for i in (0..rank).rev() {
        attacks.0 |= 1 << (i * 8 + file);
        if (blockers.0 & (1 << (i * 8 + file))) != 0 {
            break;
        }
    }
    for j in (0..file).rev() {
        attacks.0 |= 1 << (rank * 8 + j);
        if (blockers.0 & (1 << (rank * 8 + j))) != 0 {
            break;
        }
    }

    attacks
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_rook_attack_mask() {
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

    #[test]
    fn test_rook_attacks_on_the_fly() {
        assert_eq!(
            generate_rook_attacks_on_the_fly(
                Square::D4,
                BoardSlice(
                    (1 << Square::A4 as usize)
                        | (1 << Square::D2 as usize)
                        | (1 << Square::G4 as usize)
                )
            ),
            BoardSlice(0x808080877080800)
        );

        assert_eq!(
            generate_rook_attacks_on_the_fly(
                Square::F3,
                BoardSlice(
                    (1 << Square::F2 as usize)
                        | (1 << Square::G3 as usize)
                        | (1 << Square::F4 as usize)
                )
            ),
            BoardSlice(0x205f2000)
        );
    }
}
