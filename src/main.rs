use r_chess::bitboard::Bitboard;

fn main() {
    println!("Hello world");
    let position_fen = "rnbqkbnr/pp2pppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let bitboard = position_fen.parse::<Bitboard>().unwrap();
    let fen = bitboard.to_str();
    println!("{}", fen);
}
