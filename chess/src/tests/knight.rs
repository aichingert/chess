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

    #[test]
    fn knight_on_top_side() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Knight, (4, 7))
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (2, 6), (6, 6),
            (3, 5), (5, 5)
        ]);
    }

    #[test]
    fn knight_on_bottom_side() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Knight, (4, 0))
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (2, 1), (6, 1),
            (3, 2), (5, 2)
        ]);
    }
    
    #[test]
    fn knight_blocked_from_team_able_to_take_enemy() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Knight, (4, 0)),
            Piece::black(Kind::Queen, (2, 1)),
            Piece::white(Kind::Queen, (6, 1))
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (2, 1),
            (3, 2), (5, 2)
        ]);
    }
}