#[cfg(test)]
mod test {
    use crate::piece::*;

    #[test]
    fn bishop_middle_of_the_board() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Bishop, (3, 3))
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (4, 4), (5, 5), (6, 6), (7, 7),
            (4, 2), (5, 1), (6, 0),
            (2, 4), (1, 5), (0, 6),
            (2, 2), (1, 1), (0, 0), 
        ]);
    }
}