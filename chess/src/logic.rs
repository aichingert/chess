use crate::piece::*;

impl Piece {
    pub fn is_move_valid(&mut self, pos: (u8, u8), pieces: &Vec<Piece>) -> bool {
        true
    }

    pub fn get_moves(&self, pieces: &Vec<&Piece>) -> Vec<(u8, u8)> {
        let mut possible_moves: Vec<(u8, u8)> = Vec::new();

        self.check_horizontal(pieces).iter().for_each( | pos |{
            possible_moves.push(*pos)
        });

        self.check_vertical(pieces).iter().for_each( | pos | {
            possible_moves.push(*pos);
        });

        possible_moves
    }

    fn check_vertical(&self, pieces: &Vec<&Piece>) -> Vec<(u8, u8)> {
        let mut possible_positions: Vec<(u8, u8)> = Vec::new();

        'finished_up: for i in self.pos.1+1..8 {
            let mut encounter: (bool, PieceColor) = (false, PieceColor::Black);

            for piece in pieces {
                if i == piece.pos.1 && self.pos.0 == piece.pos.0 {
                    encounter = (true, piece.color);
                    break;
                }
            }

            if encounter.0 {
                if self.color != encounter.1 {
                    possible_positions.push((self.pos.0, i));
                }

                break 'finished_up;
            } else {
                possible_positions.push((self.pos.0, i));
            }
        }

        'finished_down: for i in 1..self.pos.1 {
            let mut encounter: (bool, PieceColor) = (false, PieceColor::Black);

            for piece in pieces {
                if (self.pos.1 - i) == piece.pos.1 && self.pos.0 == piece.pos.0 {
                    encounter = (true, piece.color);
                    break;
                }
            }

            if encounter.0 {
                if self.color != encounter.1 {
                    possible_positions.push((self.pos.0, (self.pos.1 - i)));
                }

                break 'finished_down;
            } else {
                possible_positions.push((self.pos.0, (self.pos.1 - i)));
            }
        }

        possible_positions
    }

    fn check_horizontal(&self, pieces: &Vec<&Piece>) -> Vec<(u8, u8)> {
        let mut possible_positions: Vec<(u8, u8)> = Vec::new();

        'finished_right: for i in self.pos.0+1..8 {
            let mut encounter: (bool, PieceColor) = (false, PieceColor::Black);

            for piece in pieces {
                if piece.pos.0 == i && piece.pos.1 == self.pos.1 {
                    encounter = (true, piece.color);
                }
            }

            if encounter.0 {
                if encounter.1 != self.color {
                    possible_positions.push((i, self.pos.1));
                }

                break 'finished_right;
            } else {
                possible_positions.push((i, self.pos.1));
            }
        }

        'finished_left: for i in 1..=self.pos.0 {
            let mut encounter: (bool, PieceColor) = (false, PieceColor::Black);

            for piece in pieces {
                if piece.pos.0 == (self.pos.0 - i) && piece.pos.1 == self.pos.1 {
                    encounter = (true, piece.color);
                }
            }

            if encounter.0 {
                if encounter.1 != self.color {
                    possible_positions.push(((self.pos.0 - i), self.pos.1));
                }

                break 'finished_left;
            } else {
                possible_positions.push(((self.pos.0 - i), self.pos.1));
            }
        }

        possible_positions
    }
}