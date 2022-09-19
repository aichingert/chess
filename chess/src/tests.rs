#[cfg(test)]
mod test {
    use super::*;
    use crate::board::*;
    use crate::piece::*;
    
    #[test]
    fn standard_pawn_moves() {
        let pieces: Vec<Piece> = vec![Piece::white(Kind::Pawn, (0, 1))];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![(0,3), (0,2)]);
    }

    #[test]
    fn no_possible_moves_when_pawn_blocked() {
        let pieces: Vec<Piece> = vec![Piece::white(Kind::Pawn, (0, 1)), Piece::black(Kind::Pawn, (0, 2))];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![]);
    }

    #[test]
    fn en_passant() {
        let pieces: Vec<Piece> = vec![Piece::white(Kind::Pawn, (1, 4)), Piece::black(Kind::Pawn, (0, 4))];
        let moves: Vec<Move> = vec![Move::new(Piece::black(Kind::Pawn, (0, 4)), (0, 6), (0, 4))];

        assert!(pieces[0].get_moves(&pieces, &moves, false) == vec![(0, 5), (1, 5)]);
    }

    #[test]
    fn take_another_piece_with_pawn() {
        let pieces: Vec<Piece> = vec![Piece::white(Kind::Pawn, (1, 4)), Piece::black(Kind::Pawn, (0, 5))];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![(0, 5), (1, 5)]);
    }
}