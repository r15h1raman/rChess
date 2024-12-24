use super::enums::{Piece, Square};
use std::fmt::{self};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move {
    pub orig: Square,
    pub dest: Square,

    pub promotion: Option<Piece>,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.orig.to_string().to_lowercase(),
            self.dest.to_string().to_lowercase()
        )?;
        match self.promotion {
            Some(Piece::Queen) => writeln!(f, "q")?,
            Some(Piece::Rook) => writeln!(f, "r")?,
            Some(Piece::Bishop) => writeln!(f, "b")?,
            Some(Piece::Knight) => writeln!(f, "n")?,
            _default => writeln!(f, "")?,
        }
        Ok(())
    }
}
