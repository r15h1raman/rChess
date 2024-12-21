use std::fmt;

use int_enum::IntEnum;

use super::enums::Square;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoardSlice(pub u64);

impl fmt::Display for BoardSlice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let binary_string = format!("{:064b}", self.0);
        let mut grid: [[char; 8]; 8] = [['0'; 8]; 8];
        for (i, char) in binary_string.chars().rev().enumerate() {
            grid[i / 8][i % 8] = char;
        }

        for (i, row) in grid.iter().enumerate().rev() {
            write!(f, "{}  ", i + 1)?;

            for &char in row {
                write!(f, " {}", char)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "\n    a b c d e f g h")?;
        write!(f, "Number: 0x{:0x}", self.0)?;
        Ok(())
    }
}

impl BoardSlice {
    fn iter_square(&self) -> impl Iterator<Item = Square> {
        let mut curr_board = self.0;
        std::iter::from_fn(move || {
            if curr_board == 0 {
                None
            } else {
                let curr_square = curr_board.trailing_zeros();
                curr_board &= curr_board - 1;
                Some(Square::from_int(curr_square as u8).unwrap())
            }
        })
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_board_slice_iter() {
        let board_slice = BoardSlice(0b1010101);
        let squares: Vec<Square> = board_slice.iter_square().collect();
        let expected_vec = vec![Square::A1, Square::C1, Square::E1, Square::G1];
        assert_eq!(squares, expected_vec);
    }
}
