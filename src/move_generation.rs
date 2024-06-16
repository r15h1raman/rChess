use int_enum::IntEnum;

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
                    new_bitboard.remove_castling_right(CastleMoves::BlackKingsideCastle);
                    new_bitboard.remove_castling_right(CastleMoves::BlackQueensideCastle);
                }
                CastleMoves::WhiteQueensideCastle => {
                    new_bitboard.move_piece(Color::White, Piece::King, Square::E1, Square::C1);
                    new_bitboard.move_piece(Color::White, Piece::Rook, Square::A1, Square::D1);
                    new_bitboard.remove_castling_right(CastleMoves::WhiteKingsideCastle);
                    new_bitboard.remove_castling_right(CastleMoves::WhiteQueensideCastle);
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
        new_bitboard.en_passant_square = Some(match _move.color {
            Color::White => Square::from_int(_move.dest as u8 - 8)
                .map_err(|_| PerformMoveError::ImpossibleDoublePush),
            Color::Black => Square::from_int(_move.dest as u8 + 8)
                .map_err(|_| PerformMoveError::ImpossibleDoublePush),
        }?);
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
                Some(square) => match _move.color {
                    Color::White => Square::from_int(square as u8 - 8)
                        .map_err(|_| PerformMoveError::EnPassantImpossible),
                    Color::Black => Square::from_int(square as u8 + 8)
                        .map_err(|_| PerformMoveError::EnPassantImpossible),
                }?,
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
            new_bitboard.remove_piece(_move.color, Piece::Pawn, _move.orig);
            new_bitboard.add_piece(_move.color, piece, _move.dest);

            new_bitboard.toggle_move();
            new_bitboard.en_passant_square = None;
            new_bitboard.half_move_clock = 0;
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
            if _move.piece == Piece::Rook || _move.orig == Square::A1 {
                new_bitboard.remove_castling_right(CastleMoves::WhiteQueensideCastle);
            };
            if _move.piece == Piece::Rook || _move.orig == Square::H1 {
                new_bitboard.remove_castling_right(CastleMoves::WhiteKingsideCastle);
            };
        }
        Color::Black => {
            if _move.piece == Piece::Rook || _move.orig == Square::A8 {
                new_bitboard.remove_castling_right(CastleMoves::BlackQueensideCastle);
            };
            if _move.piece == Piece::Rook || _move.orig == Square::H8 {
                new_bitboard.remove_castling_right(CastleMoves::BlackKingsideCastle);
            };
        }
    };

    new_bitboard.toggle_move();
    new_bitboard.en_passant_square = None;
    new_bitboard.half_move_clock = if _move.capture == None && _move.piece != Piece::Pawn {
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
        bitboard::{self, Bitboard},
        utils::enums::{CastleMoves, Color, Piece, Square},
    };

    use super::{perform_move, Move};

    #[test]
    fn test_perform_move_opera_game() {
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
        let bitboard = perform_move(&bitboard, &move1).unwrap();
        assert_eq!(
            bitboard.to_str(),
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"
        );

        let move2 = Move {
            orig: Square::E7,
            dest: Square::E5,

            color: Color::Black,
            piece: Piece::Pawn,

            capture: None,
            castle: None,
            double_push: true,
            en_passant: false,
            promotion: None,
        };
        let bitboard = perform_move(&bitboard, &move2).unwrap();
        assert_eq!(
            bitboard.to_str(),
            "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2"
        );

        let move3 = Move {
            orig: Square::G1,
            dest: Square::F3,

            color: Color::White,
            piece: Piece::Knight,

            capture: None,
            castle: None,
            double_push: false,
            en_passant: false,
            promotion: None,
        };
        let bitboard = perform_move(&bitboard, &move3).unwrap();
        assert_eq!(
            bitboard.to_str(),
            "rnbqkbnr/pppp1ppp/8/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2"
        );

        let move4 = Move {
            orig: Square::D7,
            dest: Square::D6,

            color: Color::Black,
            piece: Piece::Pawn,

            capture: None,
            castle: None,
            double_push: false,
            en_passant: false,
            promotion: None,
        };
        let bitboard = perform_move(&bitboard, &move4).unwrap();
        assert_eq!(
            bitboard.to_str(),
            "rnbqkbnr/ppp2ppp/3p4/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 3"
        );

        let move5 = Move {
            orig: Square::D2,
            dest: Square::D4,

            color: Color::White,
            piece: Piece::Pawn,

            capture: None,
            castle: None,
            double_push: true,
            en_passant: false,
            promotion: None,
        };
        let bitboard = perform_move(&bitboard, &move5).unwrap();
        assert_eq!(
            bitboard.to_str(),
            "rnbqkbnr/ppp2ppp/3p4/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq d3 0 3"
        );

        let move6 = Move {
            orig: Square::C8,
            dest: Square::G4,

            color: Color::Black,
            piece: Piece::Bishop,

            capture: None,
            castle: None,
            double_push: false,
            en_passant: false,
            promotion: None,
        };
        let bitboard = perform_move(&bitboard, &move6).unwrap();
        assert_eq!(
            bitboard.to_str(),
            "rn1qkbnr/ppp2ppp/3p4/4p3/3PP1b1/5N2/PPP2PPP/RNBQKB1R w KQkq - 1 4"
        );

        let move7 = Move {
            orig: Square::D4,
            dest: Square::E5,

            color: Color::White,
            piece: Piece::Pawn,

            capture: Some(Piece::Pawn),
            castle: None,
            double_push: false,
            en_passant: false,
            promotion: None,
        };
        let bitboard = perform_move(&bitboard, &move7).unwrap();
        assert_eq!(
            bitboard.to_str(),
            "rn1qkbnr/ppp2ppp/3p4/4P3/4P1b1/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 4"
        );

        let move8 = Move {
            orig: Square::G4,
            dest: Square::F3,

            color: Color::Black,
            piece: Piece::Bishop,

            capture: Some(Piece::Knight),
            castle: None,
            double_push: false,
            en_passant: false,
            promotion: None,
        };
        let bitboard = perform_move(&bitboard, &move8).unwrap();
        assert_eq!(
            bitboard.to_str(),
            "rn1qkbnr/ppp2ppp/3p4/4P3/4P3/5b2/PPP2PPP/RNBQKB1R w KQkq - 0 5"
        );

        let move9 = Move {
            orig: Square::D1,
            dest: Square::F3,

            color: Color::White,
            piece: Piece::Queen,

            capture: Some(Piece::Bishop),
            castle: None,
            double_push: false,
            en_passant: false,
            promotion: None,
        };
        let bitboard = perform_move(&bitboard, &move9).unwrap();
        assert_eq!(
            bitboard.to_str(),
            "rn1qkbnr/ppp2ppp/3p4/4P3/4P3/5Q2/PPP2PPP/RNB1KB1R b KQkq - 0 5"
        );

        let move10 = Move {
            orig: Square::D6,
            dest: Square::E5,

            color: Color::Black,
            piece: Piece::Pawn,

            capture: Some(Piece::Pawn),
            castle: None,
            double_push: false,
            en_passant: false,
            promotion: None,
        };
        let bitboard = perform_move(&bitboard, &move10).unwrap();
        assert_eq!(
            bitboard.to_str(),
            "rn1qkbnr/ppp2ppp/8/4p3/4P3/5Q2/PPP2PPP/RNB1KB1R w KQkq - 0 6"
        );

        // Skip moves till castling
        let position_fen = "r3kb1r/p2nqppp/5n2/1B2p1B1/4P3/1Q6/PPP2PPP/R3K2R w KQkq - 1 12";
        let bitboard = position_fen.parse::<Bitboard>().unwrap();
        let move23 = Move {
            orig: Square::A1,
            dest: Square::A1,

            color: Color::White,
            piece: Piece::King,

            capture: None,
            castle: Some(CastleMoves::WhiteQueensideCastle),
            double_push: false,
            en_passant: false,
            promotion: None,
        };
        let bitboard = perform_move(&bitboard, &move23).unwrap();
        assert_eq!(
            bitboard.to_str(),
            "r3kb1r/p2nqppp/5n2/1B2p1B1/4P3/1Q6/PPP2PPP/2KR3R b kq - 2 12"
        )
    }

    #[test]
    fn test_perform_move_promotion() {
        let position_fen = "8/4P3/k3K3/8/8/8/8/8 w - - 0 1";
        let bitboard = position_fen.parse::<Bitboard>().unwrap();
        let move1 = Move {
            orig: Square::E7,
            dest: Square::E8,

            color: Color::White,
            piece: Piece::Pawn,

            capture: None,
            castle: None,
            double_push: false,
            en_passant: false,
            promotion: Some(Piece::Rook),
        };
        let bitboard = perform_move(&bitboard, &move1).unwrap();
        assert_eq!(bitboard.to_str(), "4R3/8/k3K3/8/8/8/8/8 b - - 0 1")
    }

    #[test]
    fn test_perform_move_en_passant() {
        let position_fen = "rnbqkbnr/pp1p2pp/8/2pPpp2/4P3/8/PPP2PPP/RNBQKBNR w KQkq c6 0 4";
        let bitboard = position_fen.parse::<Bitboard>().unwrap();
        let move1 = Move {
            orig: Square::D5,
            dest: Square::C6,

            color: Color::White,
            piece: Piece::Pawn,

            capture: None,
            castle: None,
            double_push: false,
            en_passant: true,
            promotion: None,
        };
        let bitboard = perform_move(&bitboard, &move1).unwrap();
        assert_eq!(
            bitboard.to_str(),
            "rnbqkbnr/pp1p2pp/2P5/4pp2/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 4"
        )
    }
}
