use crate::{
    attack_tables::get_king_attacks,
    bitboard::{self, Bitboard},
    utils::_move::Move,
};
fn generate_legal_moves(buffer: &mut Vec<Move>, bitboard: Bitboard) {
    if bitboard.is_king_in_check(bitboard.to_move) {
        let king_square = bitboard.get_king_square(bitboard.to_move);
        let possible_king_moves = get_king_attacks(king_square) & bitboard.get_empty_squares();
        let legal_king_moves = possible_king_moves
            .iter()
            .filter(|&square| !bitboard.is_square_attacked(bitboard.to_move.opposite(), square));
    }
}
