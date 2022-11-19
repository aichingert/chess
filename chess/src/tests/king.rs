#[cfg(test)]
mod test {
    use crate::piece::*;

    #[test]
    fn king_in_the_corner() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::King, (0, 0))
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![]) == vec![
            (0, 1), (1, 0), (1, 1)
        ]);
    }

    #[test]
    fn king_in_the_middle() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::King, (3, 3))
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![]) == vec![
            (2, 2), (2, 3), (2, 4),
            (3, 2)        , (3, 4),
            (4, 2), (4, 3), (4, 4)
        ]);
    }

    #[test]
    fn king_in_check_from_queen() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::King, (3, 3)),
            Piece::black(Kind::King, (7, 7)), // Need a second king because the get moves function checks if there are 2 kings
            Piece::black(Kind::Queen, (3, 4))
        ];   

        assert!(pieces[0].get_moves(&pieces, &vec![]) == vec![
            (4, 2), (4, 4)
        ])
    }
}