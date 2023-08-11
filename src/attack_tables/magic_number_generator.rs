use crate::utils::{
    board_slice::BoardSlice,
    enums::{Piece, Square},
    errors::MagicNumberError,
};

use super::{
    bishop_attack_generators::{generate_bishop_attack_mask, generate_bishop_attacks_on_the_fly},
    magic_number_constants::{BISHOP_MASK_BIT_COUNT, ROOK_MASK_BIT_COUNT},
    occupancy::get_occupancy,
    rook_attack_generators::{generate_rook_attack_mask, generate_rook_attacks_on_the_fly},
};

// MODULE NOT USED DURING PROGRAM EXECUTION, ONLY HERE FOR DOCUMENTATION. ONLY USED TO CREATE MAGIC NUMBER CONSTANTS.

#[allow(dead_code)]
pub fn generate_magic_number(square: Square, piece: Piece) -> Result<u64, MagicNumberError> {
    let attack_mask = match piece {
        Piece::Bishop => generate_bishop_attack_mask(square),
        Piece::Rook => generate_rook_attack_mask(square),
        _ => return Err(MagicNumberError::IncorrectPiece),
    };

    let relevant_bits = if piece == Piece::Bishop {
        BISHOP_MASK_BIT_COUNT[square as usize]
    } else {
        ROOK_MASK_BIT_COUNT[square as usize]
    };

    let num_occupancies: usize = 1 << relevant_bits;

    let occupancies: Vec<BoardSlice> = (0..num_occupancies)
        .map(|i| get_occupancy(i, relevant_bits, attack_mask))
        .collect();
    let attack_table: Vec<BoardSlice> = (0..num_occupancies)
        .map(|i| {
            if piece == Piece::Bishop {
                generate_bishop_attacks_on_the_fly(square, occupancies[i])
            } else {
                generate_rook_attacks_on_the_fly(square, occupancies[i])
            }
        })
        .collect();

    for _ in 0..100_000_000 {
        let candidate = generate_magic_number_candidate();

        if u64::count_ones((attack_mask.0.wrapping_mul(candidate)) & 0xFF00_0000_0000_0000) < 6 {
            continue;
        }

        let mut used_indices = vec![BoardSlice(0); num_occupancies];
        let mut fail = false;

        for j in 0..num_occupancies {
            let magic_index =
                ((occupancies[j].0.wrapping_mul(candidate)) >> (64 - relevant_bits)) as usize;

            if used_indices[magic_index].0 == 0 {
                used_indices[magic_index] = attack_table[j];
            } else if used_indices[magic_index] != attack_table[j] {
                fail = true;
                break;
            }
        }

        if !fail {
            return Ok(candidate);
        }
    }
    Err(MagicNumberError::MagicNumberNotFound)
}

#[allow(dead_code)]
pub fn init_magic_number_generator() {
    fastrand::seed(2);
}

#[allow(dead_code)]
fn generate_magic_number_candidate() -> u64 {
    fastrand::u64(..) & fastrand::u64(..)
}
