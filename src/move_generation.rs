use crate::utils::enums::{CastleMoves, Piece, Square};

pub struct Move {
    orig: Square,
    dest: Square,
    piece: Piece,
    promoted_piece: Option<Piece>,
    capture: bool,
    double_push: bool,
    en_passant: bool,
    castle: CastleMoves,
}
