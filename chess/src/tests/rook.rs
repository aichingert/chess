#[cfg(test)]
mod test {
    use crate::piece::*;

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
    fn rook_blocked_from_three_sides_bottom_open() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Rook, (3, 3)),
            Piece::white(Kind::Pawn, (4, 3)),
            Piece::white(Kind::Pawn, (2, 3)),
            Piece::white(Kind::Pawn, (3, 4))
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (3, 2), (3, 1), (3, 0)
        ]);
    }

    #[test]
    fn rook_blocked_from_two_sides_top_bottom_open() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Rook, (3, 3)),
            Piece::white(Kind::Pawn, (4, 3)),
            Piece::white(Kind::Pawn, (2, 3)),
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (3, 4), (3, 5), (3, 6), (3, 7), (3, 2), (3, 1), (3, 0)
        ]);
    }

    #[test]
    fn rook_blocked_from_two_sides_right_left_open() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Rook, (3, 3)),
            Piece::white(Kind::Pawn, (3, 2)),
            Piece::white(Kind::Pawn, (3, 4)),
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (4, 3), (5, 3), (6, 3), (7, 3), (2, 3), (1, 3), (0, 3)
        ]);
    }
    
    #[test]
    fn rook_blocked_from_two_sides_top_right_open() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Rook, (3, 3)),
            Piece::white(Kind::Pawn, (3, 2)),
            Piece::white(Kind::Pawn, (2, 3)),
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (4, 3), (5, 3), (6, 3), (7, 3),
            (3, 4), (3, 5), (3, 6), (3, 7)
        ]);
    }
     
    #[test]
    fn rook_blocked_from_two_sides_top_left_open() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Rook, (3, 3)),
            Piece::white(Kind::Pawn, (4, 3)),
            Piece::white(Kind::Pawn, (3, 2)),
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (2, 3), (1, 3), (0, 3),
            (3, 4), (3, 5), (3, 6), (3, 7)
        ]);
    }
     
    #[test]
    fn rook_blocked_from_two_sides_bottom_right_open() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Rook, (3, 3)),
            Piece::white(Kind::Pawn, (2, 3)),
            Piece::white(Kind::Pawn, (3, 4)),
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (4, 3), (5, 3), (6, 3), (7, 3),
            (3, 2), (3, 1), (3, 0)
        ]);
    }
     
    #[test]
    fn rook_blocked_from_two_sides_bottom_left_open() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Rook, (3, 3)),
            Piece::white(Kind::Pawn, (4, 3)),
            Piece::white(Kind::Pawn, (3, 4)),
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (2, 3), (1, 3), (0, 3),
            (3, 2), (3, 1), (3, 0)
        ]);
    }

    #[test]
    fn rook_on_right_blocked() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Rook, (2, 2)),
            Piece::white(Kind::Pawn, (3, 2))
        ];
        
        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (1, 2), (0, 2),
            (2, 3), (2, 4), (2, 5), (2, 6), (2, 7),
            (2, 1), (2, 0)
       ]);
    }

    #[test]
    fn rook_on_left_blocked() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Rook, (2, 2)),
            Piece::white(Kind::Pawn, (1, 2))
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (3, 2), (4, 2), (5, 2), (6, 2), (7, 2),
            (2, 3), (2, 4), (2, 5), (2, 6), (2, 7),
            (2, 1), (2, 0)
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

    #[test]
    fn rook_can_not_take_through_enemy_piece() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::Rook, (0, 0)),
            Piece::white(Kind::Knight, (1, 0)),
            Piece::black(Kind::Rook, (0, 7)),
            Piece::black(Kind::Pawn, (0, 6))
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6)
        ])
    }
}