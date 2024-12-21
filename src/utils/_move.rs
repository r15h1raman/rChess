use super::enums::{Piece, Square};

pub struct Move {
    pub orig: Square,
    pub dest: Square,

    pub promotion: Option<Piece>,
}
