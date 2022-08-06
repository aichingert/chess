use crate::piece::*;

impl Piece {
    pub fn is_move_valid(&mut self, pos: (u8, u8), pieces: &Vec<Piece>) -> bool {
        self.get_moves(pieces).contains(&pos)
    }

    pub fn get_moves(&self, pieces: &Vec<Piece>) -> Vec<(u8, u8)> {
        let mut possible_moves: Vec<(u8, u8)> = Vec::new();

        match self.kind {
            Kind::Queen => {
                self.check_horizontal(pieces).iter().for_each( | pos | possible_moves.push(*pos));
                self.check_vertical(pieces).iter().for_each( | pos | possible_moves.push(*pos));
                self.check_diagonal_right(pieces).iter().for_each( | pos | possible_moves.push(*pos));
                self.check_diagonal_left(pieces).iter().for_each( | pos | possible_moves.push(*pos));
            },
            Kind::King => {
                self.check_king_moves(pieces).iter().for_each( | pos | possible_moves.push(*pos));
            },
            Kind::Rook => {
                self.check_horizontal(pieces).iter().for_each( | pos | possible_moves.push(*pos));
                self.check_vertical(pieces).iter().for_each( | pos | possible_moves.push(*pos));
            },
            Kind::Bishop => {
                self.check_diagonal_right(pieces).iter().for_each( | pos | possible_moves.push(*pos));
                self.check_diagonal_left(pieces).iter().for_each( | pos | possible_moves.push(*pos));
            },
            Kind::Knight => {
                self.check_horse_moves(pieces).iter().for_each( | pos | possible_moves.push(*pos));
            },
            Kind::Pawn => {
                self.check_pawn_moves(pieces).iter().for_each( | pos | possible_moves.push(*pos));
            }
        }

        possible_moves
    }

    fn check_king_moves(&self, pieces: &Vec<Piece>) -> Vec<(u8, u8)> {
        let mut possible_moves: Vec<(u8, u8)> = Vec::new();

        for i in -1..2 {
            for j in -1..2 {
                if self.pos.0 as i8 + i > -1 
                && self.pos.1 as i8 + j > -1
                && self.pos.0 + (i as u8) < 8
                && self.pos.1 + (j as u8) < 8 
                {
                    let mut encounter: (bool, PieceColor) = (false, PieceColor::White);

                    for piece in pieces {
                        if self.pos.0 as i8 + i == piece.pos.0 as i8 && self.pos.1 as i8 + j == piece.pos.1 as i8 {
                            encounter = (true, piece.color);
                        }
                    }

                    if !encounter.0 {
                        possible_moves.push(((self.pos.0 as i8 + i) as u8, (self.pos.1 as i8 + j) as u8))
                    } else {
                        if encounter.1 != self.color {
                            possible_moves.push(((self.pos.0 as i8 + i) as u8, (self.pos.1 as i8 + j) as u8))
                        }
                    }
                }
            }
        }

        for i in 0..possible_moves.len() {
            if possible_moves[i] == self.pos {
                possible_moves.remove(i);
                break;
            }
        }

        possible_moves
    }

    fn check_pawn_moves(&self, pieces: &Vec<Piece>) -> Vec<(u8, u8)> {
        let mut possible_position: Vec<(u8, u8)> = Vec::new();

        match self.color {
            PieceColor::White => {
                let mut encounter: bool = false;

                if self.pos.1 == 1 {
                    for i in 1..=2 {
                        for piece in pieces {
                            if self.pos.0 == piece.pos.0 && self.pos.1 + i == piece.pos.1 {
                                encounter = true;
                            }
                        }
                    }

                    if !encounter {
                        possible_position.push((self.pos.0, self.pos.1 + 2));
                    }
                }

                if self.pos.1 + 1 < 8 {
                    encounter = false;

                    for piece in pieces {
                        if self.pos.0 as i8 > -1 && piece.pos.0 == self.pos.0 - 1 && piece.pos.1 == self.pos.1 + 1 && self.color != piece.color {
                            possible_position.push((self.pos.0 - 1, self.pos.1 + 1))
                        }

                        if self.pos.0 + 1 < 8 && piece.pos.0 == self.pos.0 + 1 && piece.pos.1 == self.pos.1 + 1 && self.color != piece.color {
                            possible_position.push((self.pos.0 + 1, self.pos.1 + 1))
                        }

                        if self.pos.0 == piece.pos.0 && self.pos.1 + 1 == piece.pos.1 {
                            encounter = true;
                        }
                    }

                    if !encounter {
                        possible_position.push((self.pos.0, self.pos.1 + 1));
                    }
                }
            },
            PieceColor::Black => {
                let mut encounter: bool = false;

                if self.pos.1 == 6 {
                    for i in 1..=2 {
                        for piece in pieces {
                            if self.pos.0 == piece.pos.0 && self.pos.1 - i == piece.pos.1 {
                                encounter = true;
                            }
                        }
                    }

                    if !encounter {
                        possible_position.push((self.pos.0, self.pos.1 - 2));
                    }
                }

                if self.pos.1 as i8 - 1 > -1 {
                    encounter = false;

                    for piece in pieces {
                        if self.pos.0 as i8 > -1 && piece.pos.0 == self.pos.0 - 1 && piece.pos.1 == self.pos.1 - 1 && self.color != piece.color {
                            possible_position.push((self.pos.0 - 1, self.pos.1 - 1))
                        }

                        if self.pos.0 + 1 < 8 && piece.pos.0 == self.pos.0 + 1 && piece.pos.1 == self.pos.1 - 1 && self.color != piece.color {
                            possible_position.push((self.pos.0 + 1, self.pos.1 - 1))
                        }

                        if self.pos.0 == piece.pos.0 && self.pos.1 - 1 == piece.pos.1 {
                            encounter = true;
                        }
                    }

                    if !encounter {
                        possible_position.push((self.pos.0, self.pos.1 - 1));
                    }
                }
            }
        }

        possible_position
    }

    fn check_horse_moves(&self, pieces: &Vec<Piece>) -> Vec<(u8, u8)> {
        let mut possible_positions: Vec<(u8, u8)> = Vec::new();

        let moves_to_check: Vec<(i8, i8)> = vec![
            (self.pos.0 as i8 - 2, self.pos.1 as i8 + 1),
            (self.pos.0 as i8 - 2, self.pos.1 as i8 - 1),
            (self.pos.0 as i8 + 2, self.pos.1 as i8 + 1),
            (self.pos.0 as i8 + 2, self.pos.1 as i8 - 1),
            (self.pos.0 as i8 - 1, self.pos.1 as i8 + 2),
            (self.pos.0 as i8 - 1, self.pos.1 as i8 - 2),
            (self.pos.0 as i8 + 1, self.pos.1 as i8 + 2),
            (self.pos.0 as i8 + 1, self.pos.1 as i8 - 2),
        ];

        for piece_move in moves_to_check {
            if piece_move.0 > -1 && piece_move.0 < 8 
            && piece_move.1 > -1 && piece_move.1 < 8 {
                let mut encounter: (bool, PieceColor) = (false, PieceColor::White);

                for piece in pieces {
                    if piece.pos.0 == piece_move.0 as u8 && piece.pos.1 == piece_move.1 as u8 {
                        encounter = (true, piece.color);
                        break;
                    }
                }

                if encounter.0 {
                    if self.color != encounter.1 {
                        possible_positions.push((piece_move.0 as u8, piece_move. 1 as u8));
                    }
                } else {
                    possible_positions.push((piece_move.0 as u8, piece_move. 1 as u8));
                }
            }
        }

        possible_positions
    }

    fn check_diagonal_left(&self, pieces: &Vec<Piece>) -> Vec<(u8, u8)> {
        let mut possible_positions: Vec<(u8, u8)> = Vec::new();

        let mut j: u8 = 0;

        'finished_diagonal_left_up: for i in 1..=self.pos.0 {
            let mut encounter: (bool, PieceColor) = (false, PieceColor::White);
            j += 1;

            for piece in pieces {
                if self.pos.0 - i == piece.pos.0 && self.pos.1 + j == piece.pos.1 {
                    encounter = (true, piece.color);
                }
            }

            if encounter.0 {
                if encounter.1 != self.color {
                    possible_positions.push((self.pos.0 - i, self.pos.1 + j));
                }

                break 'finished_diagonal_left_up;
            } else {
                possible_positions.push((self.pos.0 - i, self.pos.1 + j));
            }
        }

        j = 0;

        
        'finished_diagonal_left_down: for i in self.pos.0+1..8 {
            let mut encounter: (bool, PieceColor) = (false, PieceColor::White);
            j += 1;

            for piece in pieces {
                if i == piece.pos.0 && self.pos.1 - j == piece.pos.1 {
                    encounter = (true, piece.color);
                }
            }

            if encounter.0 {
                if encounter.1 != self.color {
                    possible_positions.push((i, self.pos.1 - j));
                }

                break 'finished_diagonal_left_down;
            } else {
                possible_positions.push((i, self.pos.1 - j));
            }
        }

        possible_positions
    }

    fn check_diagonal_right(&self, pieces: &Vec<Piece>) -> Vec<(u8, u8)> {
        let mut possible_positions: Vec<(u8, u8)> = Vec::new();
        let mut j: u8 = 0;

        'finished_diagonal_right_up: for i in self.pos.0+1..8 {
            let mut encounter: (bool, PieceColor) = (false, PieceColor::White);
            j += 1;

            for piece in pieces {
                if i == piece.pos.0 && self.pos.1 + j == piece.pos.1 {
                    encounter = (true, piece.color);
                }
            }

            if encounter.0 {
                if encounter.1 != self.color {
                    possible_positions.push((i, self.pos.1 + j));
                }

                break 'finished_diagonal_right_up;
            } else {
                possible_positions.push((i, self.pos.1 + j));
            }
        }

        j = 0;

        'finished_diagonal_right_down: for i in 1..=self.pos.0 {
            let mut encounter: (bool, PieceColor) = (false, PieceColor::White);
            j += 1;

            for piece in pieces {
                if self.pos.0 - i == piece.pos.0 && self.pos.1 - j == piece.pos.1 {
                    encounter = (true, piece.color);
                }
            }

            if encounter.0 {
                if encounter.1 != self.color {
                    possible_positions.push((self.pos.0 - i, self.pos.1 - j));
                }

                break 'finished_diagonal_right_down;
            } else {
                possible_positions.push((self.pos.0 - i, self.pos.1 - j));
            }
        }

        possible_positions
    }

    fn check_vertical(&self, pieces: &Vec<Piece>) -> Vec<(u8, u8)> {
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

        'finished_down: for i in 1..=self.pos.1 {
            let mut encounter: (bool, PieceColor) = (false, PieceColor::Black);

            for piece in pieces {
                if (self.pos.1 - i) == piece.pos.1 && self.pos.0 == piece.pos.0 {
                    encounter = (true, piece.color);
                    break;
                }
            }

            if encounter.0 {
                if self.color != encounter.1 {
                    possible_positions.push((self.pos.0, self.pos.1 - i));
                }

                break 'finished_down;
            } else {
                possible_positions.push((self.pos.0, self.pos.1 - i));
            }
        }

        possible_positions
    }

    fn check_horizontal(&self, pieces: &Vec<Piece>) -> Vec<(u8, u8)> {
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
                    possible_positions.push((self.pos.0 - i, self.pos.1));
                }

                break 'finished_left;
            } else {
                possible_positions.push((self.pos.0 - i, self.pos.1));
            }
        }

        possible_positions
    }
}