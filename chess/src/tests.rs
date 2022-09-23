#[cfg(test)]
mod test {
    use crate::board::*;
    use crate::piece::*;
    
    #[test]
    fn pawn_starting_moves() {
        let pieces: Vec<Piece> = vec![Piece::white(Kind::Pawn, (0, 1))];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![(0,3), (0,2)]);
    }

    #[test]
    fn pawn_unable_to_move_when_blocked() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Pawn, (0, 1)), 
            Piece::black(Kind::Pawn, (0, 2))
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![]);
    }

    #[test]
    fn pawn_able_to_take_while_blocked() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Pawn, (0, 1)), 
            Piece::black(Kind::Pawn, (0, 2)), 
            Piece::black(Kind::Pawn, (1, 2))
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![(1,2)]);
    }

    #[test]
    fn pawn_en_passnt_check() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Pawn, (1, 4)), 
            Piece::black(Kind::Pawn, (0, 4))
        ];
        let moves: Vec<Move> = vec![
            Move::new(Piece::black(Kind::Pawn, (0, 4)), (0, 6), (0, 4))
        ];

        assert!(pieces[0].get_moves(&pieces, &moves, false) == vec![(0, 5), (1, 5)]);
    }

    #[test]
    fn pawn_en_passant_not_possible_check() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Pawn, (1, 4)), 
            Piece::black(Kind::Pawn, (0, 4))
        ];
        let moves: Vec<Move> = vec![];

        assert!(pieces[0].get_moves(&pieces, &moves, false) == vec![(1, 5)]);
    }

    #[test]
    fn pawn_take_another_piece() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Pawn, (1, 4)), 
            Piece::black(Kind::Pawn, (0, 5))
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![(0, 5), (1, 5)]);
    }

    #[test]
    fn pawn_promotion_check() {
        // Not implemented yet

        let pieces: Vec<Piece> = vec![Piece::white(Kind::Pawn, (1, 7))];

        assert!(false);
    }

    #[test]
    fn rook_standard_moves() {
        let pieces: Vec<Piece> = vec![Piece::white(Kind::Rook, (3, 3))];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (4, 3), (5, 3), (6, 3), (7, 3), 
            (2, 3), (1, 3), (0, 3),
            (3, 4), (3, 5), (3, 6), (3, 7), 
            (3, 2), (3, 1), (3, 0)]);
    }

    #[test]
    fn rook_corner_moves() {
        let pieces: Vec<Piece> = vec![Piece::white(Kind::Rook, (0, 0))];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0),
            (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7)
        ]);
    }

    #[test]
    fn rook_unable_to_move_when_fully_blocked() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Rook, (3, 3)),
            Piece::white(Kind::Pawn, (4, 3)),
            Piece::white(Kind::Pawn, (2, 3)),
            Piece::white(Kind::Pawn, (3, 2)),
            Piece::white(Kind::Pawn, (3, 4))
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![]);
    }

    #[test]
    fn rook_blocked_from_three_sides_right_open() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Rook, (3, 3)),
            Piece::white(Kind::Pawn, (2, 3)),
            Piece::white(Kind::Pawn, (3, 2)),
            Piece::white(Kind::Pawn, (3, 4))
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (4, 3), (5, 3), (6, 3), (7, 3)
        ]);
    }

    #[test]
    fn rook_blocked_from_three_sides_left_open() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Rook, (3, 3)),
            Piece::white(Kind::Pawn, (4, 3)),
            Piece::white(Kind::Pawn, (3, 2)),
            Piece::white(Kind::Pawn, (3, 4))
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (2, 3), (1, 3), (0, 3)
        ]);
    }

    #[test]
    fn rook_blocked_from_three_sides_top_open() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Rook, (3, 3)),
            Piece::white(Kind::Pawn, (4, 3)),
            Piece::white(Kind::Pawn, (2, 3)),
            Piece::white(Kind::Pawn, (3, 2)),
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (3, 4), (3, 5), (3, 6), (3, 7)
        ]);
    }

    #[test]
    fn rook_on_top_blocked() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Rook, (2, 2)),
            Piece::white(Kind::Pawn, (2, 3))
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (3, 2), (4, 2), (5, 2), (6, 2), (7, 2), 
            (1, 2), (0, 2), 
            (2, 1), (2, 0)
        ]);
    }

    #[test]
    fn rook_on_bottom_blocked() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Rook, (2, 2)),
            Piece::white(Kind::Pawn, (2, 1))
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (3, 2), (4, 2), (5, 2), (6, 2), (7, 2),
            (1, 2), (0, 2),
            (2, 3), (2, 4), (2, 5), (2, 6), (2, 7)
        ])
    }
}