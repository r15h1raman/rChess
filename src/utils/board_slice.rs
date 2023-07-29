use std::fmt;

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
        write!(f, "Number: {:0x}", self.0)?;
        Ok(())
    }
}
