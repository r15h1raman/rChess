use std::{i8, usize};

use int_enum::IntEnum;
use strum::IntoEnumIterator;

use crate::{
    bitboard::Bitboard,
    utils::{
        _move::Move,
        enums::{CastleMoves, Color, Piece, Square},
        errors::PerformMoveError,
    },
};

/// Perform move on bitboard and return correct new bitboard or error if encountered
/// Function does NOT check for legality of move. Illegal moves may result in the function throwing
/// an error or returning correctly (undefined behavior).
fn perform_move(bitboard: &Bitboard, _move: &Move) -> Result<Bitboard, PerformMoveError> {
    let mut new_bitboard = bitboard.clone();

    let move_color = if bitboard.get_color_pieces(Color::White).0 & (1 << _move.orig as usize) != 0
    {
        Color::White
    } else {
        Color::Black
    };

    let move_piece = Piece::iter()
        .filter(|&piece| bitboard.get_piece(move_color, piece).0 & (1 << _move.orig as usize) != 0)
        .next()
        .unwrap();

    // Castling
    if move_piece == Piece::King && ((_move.orig as i8 - _move.dest as i8).abs() == 2) {
        if _move.dest == Square::G1 {
            new_bitboard.move_piece(Color::White, Piece::King, Square::E1, Square::G1);
            new_bitboard.move_piece(Color::White, Piece::Rook, Square::H1, Square::F1);
            new_bitboard.remove_castling_right(CastleMoves::WhiteKingsideCastle);
            new_bitboard.remove_castling_right(CastleMoves::WhiteQueensideCastle);
        } else if _move.dest == Square::G8 {
            new_bitboard.move_piece(Color::Black, Piece::King, Square::E8, Square::G8);
            new_bitboard.move_piece(Color::Black, Piece::Rook, Square::H8, Square::F8);
            new_bitboard.remove_castling_right(CastleMoves::BlackKingsideCastle);
            new_bitboard.remove_castling_right(CastleMoves::BlackQueensideCastle);
        } else if _move.dest == Square::C1 {
            new_bitboard.move_piece(Color::White, Piece::King, Square::E1, Square::C1);
            new_bitboard.move_piece(Color::White, Piece::Rook, Square::A1, Square::D1);
            new_bitboard.remove_castling_right(CastleMoves::WhiteKingsideCastle);
            new_bitboard.remove_castling_right(CastleMoves::WhiteQueensideCastle);
        } else if _move.dest == Square::C8 {
            new_bitboard.move_piece(Color::Black, Piece::King, Square::E8, Square::C8);
            new_bitboard.move_piece(Color::Black, Piece::Rook, Square::A8, Square::D8);
            new_bitboard.remove_castling_right(CastleMoves::BlackKingsideCastle);
            new_bitboard.remove_castling_right(CastleMoves::BlackQueensideCastle);
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

    // Double push
    if move_piece == Piece::Pawn && ((_move.orig as i8 - _move.dest as i8).abs() == 16) {
        new_bitboard.move_piece(move_color, Piece::Pawn, _move.orig, _move.dest);

        new_bitboard.toggle_move();
        new_bitboard.en_passant_square = Some(match move_color {
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
    if bitboard.en_passant_square == Some(_move.dest) && move_piece == Piece::Pawn {
        new_bitboard.move_piece(move_color, Piece::Pawn, _move.orig, _move.dest);
        new_bitboard.remove_piece(
            move_color.opposite(),
            Piece::Pawn,
            match bitboard.en_passant_square {
                Some(square) => match move_color {
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
            new_bitboard.remove_piece(move_color, Piece::Pawn, _move.orig);
            new_bitboard.add_piece(move_color, piece, _move.dest);

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
    let capture_piece = Piece::iter()
        .filter(|&piece| {
            bitboard.get_piece(move_color.opposite(), piece).0 & (1 << _move.dest as usize) != 0
        })
        .next();

    new_bitboard.move_piece(move_color, move_piece, _move.orig, _move.dest);

    match capture_piece {
        Some(piece) => new_bitboard.remove_piece(move_color.opposite(), piece, _move.dest),
        None => {}
    };

    match move_color {
        Color::White => {
            if move_piece == Piece::Rook && _move.orig == Square::A1 {
                new_bitboard.remove_castling_right(CastleMoves::WhiteQueensideCastle);
            } else if move_piece == Piece::Rook && _move.orig == Square::H1 {
                new_bitboard.remove_castling_right(CastleMoves::WhiteKingsideCastle);
            } else if move_piece == Piece::King {
                new_bitboard.remove_castling_right(CastleMoves::WhiteKingsideCastle);
                new_bitboard.remove_castling_right(CastleMoves::WhiteQueensideCastle);
            }
        }
        Color::Black => {
            if move_piece == Piece::Rook && _move.orig == Square::A8 {
                new_bitboard.remove_castling_right(CastleMoves::BlackQueensideCastle);
            } else if move_piece == Piece::Rook && _move.orig == Square::H8 {
                new_bitboard.remove_castling_right(CastleMoves::BlackKingsideCastle);
            } else if move_piece == Piece::King {
                new_bitboard.remove_castling_right(CastleMoves::BlackKingsideCastle);
                new_bitboard.remove_castling_right(CastleMoves::BlackQueensideCastle);
            }
        }
    };

    new_bitboard.toggle_move();
    new_bitboard.en_passant_square = None;
    new_bitboard.half_move_clock = if capture_piece == None && move_piece != Piece::Pawn {
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

#[cfg(test)]
pub mod tests {

    use crate::{
        bitboard::Bitboard,
        utils::enums::{Piece, Square},
    };

    use super::{perform_move, Move};

    #[test]
    fn test_perform_move_opera_game() {
        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let bitboard = position_fen.parse::<Bitboard>().unwrap();

        let move1 = Move {
            orig: Square::E2,
            dest: Square::E4,

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
            orig: Square::E1,
            dest: Square::C1,

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

            promotion: None,
        };
        let bitboard = perform_move(&bitboard, &move1).unwrap();
        assert_eq!(
            bitboard.to_str(),
            "rnbqkbnr/pp1p2pp/2P5/4pp2/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 4"
        )
    }

    #[test]
    fn test_remove_castle_rights_on_king_move() {
        let position_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1";
        let bitboard = position_fen.parse::<Bitboard>().unwrap();
        let move1 = Move {
            orig: Square::E1,
            dest: Square::D1,

            promotion: None,
        };

        let bitboard = perform_move(&bitboard, &move1).unwrap();
        assert_eq!(
            bitboard.to_str(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R2K3R b kq - 1 1"
        )
    }

    #[test]
    fn test_remove_castle_rights_on_rook_move() {
        let position_fen = "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R b KQkq - 1 1";
        let bitboard = position_fen.parse::<Bitboard>().unwrap();
        let move1 = Move {
            orig: Square::A8,
            dest: Square::B8,

            promotion: None,
        };

        let bitboard = perform_move(&bitboard, &move1).unwrap();
        assert_eq!(
            bitboard.to_str(),
            "1r2k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQk - 2 2"
        )
    }
}
