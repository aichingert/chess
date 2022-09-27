#[cfg(test)]
mod test {
    use crate::piece::*;

    #[test]
    fn king_in_the_corner() {
        let pieces: Vec<Piece> = vec![
            Piece::white(Kind::King, (0, 0))
        ];

        assert!(pieces[0].get_moves(&pieces, &vec![], false) == vec![
            (0, 1), (1, 0), (1, 1)
        ]);
    }
}