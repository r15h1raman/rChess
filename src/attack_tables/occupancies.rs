use crate::utils::board_slice::BoardSlice;

const BISHOP_MASK_BIT_COUNT: [u32; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5, 5, 5, 7, 7, 7, 7, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 6, 5, 5, 5, 5, 5, 5, 6,
];

const ROOK_MASK_BIT_COUNT: [u32; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11, 11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 12, 11, 11, 11, 11, 11, 11, 12,
];

fn set_occupancy(index: u32, bit_count: u32, mut attack_mask: BoardSlice) -> BoardSlice {
    let mut occupancy = BoardSlice(0);

    for i in 0..bit_count {
        let square = attack_mask.0.trailing_zeros();

        attack_mask.0 &= !(1 << square);

        if (1 << i) & index != 0 {
            occupancy.0 |= 1 << square;
        }
    }
    occupancy
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn set_occupancy_valid() {
        assert_eq!(
            set_occupancy(3, 4, BoardSlice(0b110001100)),
            BoardSlice(0b1100)
        );
        assert_eq!(
            set_occupancy(8, 5, BoardSlice(0b110001110)),
            BoardSlice(0b10000000)
        )
    }
}
