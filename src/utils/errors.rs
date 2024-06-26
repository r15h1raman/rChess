use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum FENParseError {
    #[error("FEN does not have 6 parts; instead has {0} parts.")]
    IncorrectPartsCount(usize),
    #[error("Board does not have 8 parts; instead has {0} parts.")]
    IncorrectBoardLength(usize),
    #[error("Row {0} is too long.")]
    IncorrectBoardRowLength(usize),
    #[error("Incorrect symbol found in board string: {0}.")]
    IncorrectBoard(char),
    #[error("Incorrect symbol found in color to move.")]
    IncorrectToMove,
    #[error("Incorrect symbol found in castling rights.")]
    IncorrectCastlingRights,
    #[error("Incorrect symbol found in color to move.")]
    IncorrectEnPassantSquare,
    #[error("Incorrect symbol found in half move clock.")]
    IncorrectHalfMoveClock,
    #[error("Incorrect symbol found in full move clock.")]
    IncorrectFullMoveClock,
}

#[derive(Debug, Error, PartialEq)]
pub enum MagicNumberError {
    #[error("Piece must be bishop or rook.")]
    IncorrectPiece,
    #[error("Magic number was not able to be found. Try changing the seed.")]
    MagicNumberNotFound,
}

#[derive(Debug, Error, PartialEq)]
pub enum PerformMoveError {
    #[error("En passant performed with no en passant allowed.")]
    EnPassantImpossible,
    #[error("Impossible double push move.")]
    ImpossibleDoublePush,
}
