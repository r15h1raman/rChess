use crate::{
    attack_tables::{
        get_bishop_attacks, get_king_attacks, get_knight_attacks, get_pawn_attacks,
        get_queen_attacks, get_rook_attacks,
    },
    bitboard::Bitboard,
    utils::enums::{CastleMoves, Color, Piece, Square},
};

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

fn is_square_attacked(color: Color, square: Square, bitboard: Bitboard) -> bool {
    ((get_pawn_attacks(
        match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        },
        square,
    )
    .0 & bitboard.get_board_slice(color, Piece::Pawn).0)
        | (get_knight_attacks(square).0 & bitboard.get_board_slice(color, Piece::Knight).0)
        | (get_bishop_attacks(square, bitboard.get_all_pieces()).0
            & bitboard.get_board_slice(color, Piece::Bishop).0)
        | (get_rook_attacks(square, bitboard.get_all_pieces()).0
            & bitboard.get_board_slice(color, Piece::Rook).0)
        | (get_queen_attacks(square, bitboard.get_all_pieces()).0
            & bitboard.get_board_slice(color, Piece::Queen).0)
        | (get_king_attacks(square).0 & bitboard.get_board_slice(color, Piece::King).0))
        != 0
}
