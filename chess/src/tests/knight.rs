#[cfg(test)]
mod test {
    use crate::piece::*;

    #[test]
    fn kinght_on_left_side() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Knight, (0, 4))
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (2, 5), (2, 3),
            (1, 6), (1, 2)
        ]);
    }

    #[test]
    fn knight_on_right_side() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Knight, (7, 4))
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (5, 5), (5, 3),
            (6, 6), (6, 2)
        ]);
    }


}