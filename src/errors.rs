use quick_error::quick_error;
quick_error! {
    #[derive(Debug, PartialEq)]
    pub enum FENParseError {
        IncorrectPartsCount(value: usize) {
            display("FEN does not have 6 parts; instead has {} parts.", value)
        }
        IncorrectBoardLength(value: usize) {
            display("Board does not have 8 parts; instead has {} parts.", value)
        }
        IncorrectBoardRowLength(value: usize) {
            display("Row {} is too long.", value)
        }
        IncorrectBoard(value: char) {
            display("Incorrect symbol found in board string: {}.", value)
        }
        IncorrectToMove {
            display("Incorrect symbol found in color to move.")
        }
        IncorrectCastlingRights {
            display("Incorrect symbol found in castling rights.")
        }
        IncorrectEnPassantSquare {
            display("Incorrect symbol found in color to move.")
        }
        IncorrectHalfMoveClock {
            display("Incorrect symbol found in half move clock.")
        }
        IncorrectFullMoveClock {
            display("Incorrect symbol found in full move clock.")
        }
    }
}
