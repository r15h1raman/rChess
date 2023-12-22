use crate::utils::board_slice::BoardSlice;

pub fn get_occupancy(index: usize, bit_count: usize, mut attack_mask: BoardSlice) -> BoardSlice {
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
    fn test_get_occupancy() {
        assert_eq!(
            get_occupancy(3, 4, BoardSlice(0b110001100)),
            BoardSlice(0b1100)
        );
        assert_eq!(
            get_occupancy(8, 5, BoardSlice(0b110001110)),
            BoardSlice(0b10000000)
        )
    }
}
