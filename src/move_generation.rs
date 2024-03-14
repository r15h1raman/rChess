use crate::{
    bitboard::Bitboard,
    utils::{
        enums::{CastleMoves, Color, Piece, Square},
        errors::PerformMoveError,
    },
};

pub struct Move {
    orig: Square,
    dest: Square,

    color: Color,
    piece: Piece,

    capture: Option<Piece>,
    castle: Option<CastleMoves>,
    double_push: bool,
    en_passant: bool,
    promotion: Option<Piece>,
}

fn perform_move(bitboard: &Bitboard, _move: &Move) -> Result<Bitboard, PerformMoveError> {
    let mut new_bitboard = bitboard.clone();

    // Castling
    match _move.castle {
        Some(castle) => {
            match castle {
                CastleMoves::WhiteKingsideCastle => {
                    new_bitboard.move_piece(Color::White, Piece::King, Square::E1, Square::G1);
                    new_bitboard.move_piece(Color::White, Piece::Rook, Square::H1, Square::F1);
                    new_bitboard.remove_castling_right(CastleMoves::WhiteKingsideCastle);
                    new_bitboard.remove_castling_right(CastleMoves::WhiteQueensideCastle);
                }
                CastleMoves::BlackKingsideCastle => {
                    new_bitboard.move_piece(Color::Black, Piece::King, Square::E8, Square::G8);
                    new_bitboard.move_piece(Color::Black, Piece::Rook, Square::H8, Square::F8);
                    new_bitboard.remove_castling_right(CastleMoves::WhiteKingsideCastle);
                    new_bitboard.remove_castling_right(CastleMoves::WhiteQueensideCastle);
                }
                CastleMoves::WhiteQueensideCastle => {
                    new_bitboard.move_piece(Color::White, Piece::King, Square::E1, Square::C1);
                    new_bitboard.move_piece(Color::White, Piece::Rook, Square::A1, Square::D1);
                    new_bitboard.remove_castling_right(CastleMoves::BlackKingsideCastle);
                    new_bitboard.remove_castling_right(CastleMoves::BlackQueensideCastle);
                }
                CastleMoves::BlackQueensideCastle => {
                    new_bitboard.move_piece(Color::Black, Piece::King, Square::E8, Square::C8);
                    new_bitboard.move_piece(Color::Black, Piece::Rook, Square::A8, Square::D8);
                    new_bitboard.remove_castling_right(CastleMoves::BlackKingsideCastle);
                    new_bitboard.remove_castling_right(CastleMoves::BlackQueensideCastle);
                }
            }
            new_bitboard.toggle_move();
            new_bitboard.en_passant_square = None;
            new_bitboard.half_move_clock += 1;
            new_bitboard.full_move_clock += if new_bitboard.to_move == Color::White {
                1
            } else {
                0
            };
            return Ok(new_bitboard);
        }
        None => {}
    };

    // Double push
    if _move.double_push {
        new_bitboard.move_piece(_move.color, Piece::Pawn, _move.orig, _move.dest);

        new_bitboard.toggle_move();
        new_bitboard.en_passant_square = Some(_move.dest);
        new_bitboard.half_move_clock = 0;
        new_bitboard.full_move_clock += if new_bitboard.to_move == Color::White {
            1
        } else {
            0
        };
        return Ok(new_bitboard);
    };

    // En passant
    if _move.en_passant {
        new_bitboard.move_piece(_move.color, Piece::Pawn, _move.orig, _move.dest);
        new_bitboard.remove_piece(
            _move.color.opposite(),
            Piece::Pawn,
            match bitboard.en_passant_square {
                Some(square) => square,
                None => return Err(PerformMoveError::EnPassantImpossible),
            },
        );

        new_bitboard.toggle_move();
        new_bitboard.en_passant_square = None;
        new_bitboard.half_move_clock = 0;
        new_bitboard.full_move_clock += if new_bitboard.to_move == Color::White {
            1
        } else {
            0
        };
        return Ok(new_bitboard);
    };

    // Promotion
    match _move.promotion {
        Some(piece) => {
            new_bitboard.remove_piece(_move.color, Piece::Pawn, _move.dest);
            new_bitboard.add_piece(_move.color, piece, _move.dest);

            new_bitboard.toggle_move();
            new_bitboard.en_passant_square = None;
            new_bitboard.half_move_clock += 1;
            new_bitboard.full_move_clock += if new_bitboard.to_move == Color::White {
                1
            } else {
                0
            };
            return Ok(new_bitboard);
        }
        None => {}
    }

    // Normal and capture
    new_bitboard.move_piece(_move.color, _move.piece, _move.orig, _move.dest);
    match _move.capture {
        Some(piece) => new_bitboard.remove_piece(_move.color.opposite(), piece, _move.dest),
        None => {}
    };

    match _move.color {
        Color::White => {
            if _move.piece == Piece::King || _move.orig == Square::A1 {
                new_bitboard.remove_castling_right(CastleMoves::WhiteQueensideCastle);
            };
            if _move.piece == Piece::King || _move.orig == Square::H1 {
                new_bitboard.remove_castling_right(CastleMoves::WhiteKingsideCastle);
            };
        }
        Color::Black => {
            if _move.piece == Piece::King || _move.orig == Square::A8 {
                new_bitboard.remove_castling_right(CastleMoves::BlackQueensideCastle);
            };
            if _move.piece == Piece::King || _move.orig == Square::H8 {
                new_bitboard.remove_castling_right(CastleMoves::BlackKingsideCastle);
            };
        }
    };

    new_bitboard.toggle_move();
    new_bitboard.en_passant_square = None;
    new_bitboard.half_move_clock = if _move.capture != None || _move.piece == Piece::Pawn {
        new_bitboard.half_move_clock + 1
    } else {
        0
    };
    new_bitboard.full_move_clock += if new_bitboard.to_move == Color::White {
        1
    } else {
        0
    };
    Ok(new_bitboard)
}

fn generate_moves(buffer: &mut Vec<Move>, color: Color, bitboard: &Bitboard) {
    let is_check = bitboard.is_king_in_check(color);
}

#[cfg(test)]
pub mod tests {
    use crate::{
        bitboard::Bitboard,
        utils::enums::{Color, Piece, Square},
    };

    use super::{perform_move, Move};

    #[test]
    fn test_perform_move_opera() {
        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let bitboard = position_fen.parse::<Bitboard>().unwrap();

        let move1 = Move {
            orig: Square::E2,
            dest: Square::E4,

            color: Color::White,
            piece: Piece::Pawn,

            capture: None,
            castle: None,
            double_push: true,
            en_passant: false,
            promotion: None,
        };
        let move1_bb = perform_move(&bitboard, &move1);
    }
}
