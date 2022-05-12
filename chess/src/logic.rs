use bevy::prelude::*;
use crate::piece::*;

impl Piece {
    pub fn calculate_pseudo_legal_moves(&mut self, pieces: Vec<Piece>) {
        self.moves.clear();
        let mut possible_moves: Vec<(i32, i32)> = Vec::new(); 
    
        let x_position = self.position.0;
        let y_position = self.position.1;
    
        match self.color {
            PieceColor::White => {
                match self.kind {
                    Kind::Pawn => {
                        if y_position == 7 {
                            self.promotion(Kind::Queen);
                            return;
                        }
                        let mut pawn_blocked = false;

                        if y_position == 1 {
                            for p in &pieces {
                                for i in 1..3 {
                                    if (y_position + i) == p.position.1 && x_position == p.position.0 {
                                        pawn_blocked = true;
                                        break;
                                    }
                                }
                            }

                            if !pawn_blocked {
                                possible_moves.push((x_position, y_position + 2));
                            }
                        }
                    
                        pawn_blocked = false;

                        for p in &pieces {
                            if (y_position + 1) == p.position.1 && x_position == p.position.0 || (y_position + 1) > 7 {
                                pawn_blocked = true;
                            }
                            
                            match p.color {
                                PieceColor::Black => {
                                    match p.kind {
                                        Kind::Pawn => {
                                            match p.en_passant {
                                                EnPassantStates::Ready => {
                                                    // En passant check, possbile issue is that it can take 2 pieces at once
                                                    if y_position == p.position.1 && x_position - 1 == p.position.0 && (x_position - 1) >= 0 {
                                                        possible_moves.push((x_position - 1, y_position + 1));
                                                    }

                                                    if y_position == p.position.1 && x_position + 1 == p.position.0 && (x_position + 1) <= 7 {
                                                        possible_moves.push((x_position + 1, y_position + 1));
                                                    }

                                                    info!("{:?}", p);
                                                },
                                                _ => {}
                                            }
                                        },
                                        _ => {}
                                    }

                                    if (y_position + 1) == p.position.1 && (x_position + 1) == p.position.0 
                                    && ( (y_position + 1) < 8 || (x_position + 1) < 8 ) {
                                        possible_moves.push((x_position + 1, y_position + 1))
                                    }
        
                                    if (y_position + 1) == p.position.1 && (x_position - 1) == p.position.0 
                                        && ( (y_position + 1) < 8 || (x_position - 1) > -1 ) {
                                        possible_moves.push((x_position - 1, y_position + 1))
                                    }
                                },
                                _ => {}
                            }
                        }

                        if !pawn_blocked {
                            possible_moves.push((x_position, y_position + 1));
                        }

                        self.moves = possible_moves.clone();
                    },
                    Kind::Knight => {
                        possible_moves.push((x_position + 2, y_position + 1));
                        possible_moves.push((x_position + 2, y_position - 1));
                        possible_moves.push((x_position - 2, y_position + 1));
                        possible_moves.push((x_position - 2, y_position - 1));
                        possible_moves.push((x_position + 1, y_position + 2));
                        possible_moves.push((x_position - 1, y_position + 2));
                        possible_moves.push((x_position + 1, y_position - 2));
                        possible_moves.push((x_position - 1, y_position - 2));

                        let mut idx: usize;
                        let mut bad_moves: bool = true;

                        while bad_moves {
                            bad_moves = false;
                            idx = 0;

                            for i in 0..possible_moves.len() {
                                if possible_moves[i].0 < 0 || possible_moves[i].0 > 7 
                                || possible_moves[i].1 < 0 || possible_moves[i].1 > 7
                                || is_blocked_friendly(pieces.clone(), possible_moves[i], PieceColor::White) {
                                    bad_moves = true;
                                    idx = i;
                                    break;
                                }
                            }

                            if bad_moves {
                                possible_moves.remove(idx);
                            }
                        }

                        self.moves = possible_moves.clone();
                    },
                    Kind::Bishop => {
                        let mut x_pos: i32 = x_position;

                        for i in y_position+1..8 {
                            x_pos += 1;

                            if !is_blocked_friendly(pieces.clone(), (x_pos, i) ,PieceColor::White) 
                            && x_pos < 8 {
                                possible_moves.push((x_pos, i));
                            }
                            else {
                                break;
                            }
                        }

                        x_pos = x_position;

                        for i in y_position+1..8 {
                            x_pos -= 1;

                            if !is_blocked_friendly(pieces.clone(), (x_pos, i) ,PieceColor::White) 
                            && x_pos > -1 {
                                possible_moves.push((x_pos, i));
                            }
                            else {
                                break;
                            }
                        }

                        x_pos = x_position;

                        for i in 1..y_position {
                            x_pos += 1;

                            if !is_blocked_friendly(pieces.clone(), (x_pos, y_position + i) ,PieceColor::White) 
                            && x_pos < 8 {
                                possible_moves.push((x_pos, i));
                            }
                            else {
                                break;
                            }
                        }

                        x_pos = x_position;

                        for i in 1..y_position {
                            x_pos -= 1;

                            if !is_blocked_friendly(pieces.clone(), (x_pos, y_position - i) ,PieceColor::White) 
                            && x_pos < 8 {
                                possible_moves.push((x_pos, i));
                            }
                            else {
                                break;
                            }
                        }

                        self.moves = possible_moves.clone();
                    },
                    Kind::Rook => {},
                    Kind::Queen => {},
                    Kind::King => {},
                }
            },
            PieceColor::Black => {
                match self.kind {
                    Kind::Pawn => {
                        if y_position == 0 {
                            self.promotion(Kind::Queen);
                            return;
                        }

                        let mut pawn_blocked = false;

                        if y_position == 6 {
                            for p in &pieces {
                                for i in 1..3 {
                                    if (y_position - i) == p.position.1 && x_position == p.position.0 {
                                        pawn_blocked = true;
                                        break;
                                    }
                                }
                            }

                            if !pawn_blocked {
                                possible_moves.push((x_position, y_position - 2));
                            }
                        }

                        pawn_blocked = false;

                        for p in &pieces {
                            if (y_position - 1) == p.position.1 && x_position == p.position.0 || (y_position - 1) < 0 {
                                pawn_blocked = true;
                            }

                            match p.color {
                                PieceColor::White => {
                                    match p.kind {
                                        Kind::Pawn => {
                                            match p.en_passant {
                                                EnPassantStates::Ready => {
                                                    if y_position == p.position.1 && x_position - 1 == p.position.0 && (x_position - 1) >= 0 {
                                                        possible_moves.push((x_position - 1, y_position - 1));
                                                    }
                
                                                    if y_position == p.position.1 && x_position + 1 == p.position.0 && (x_position + 1) <= 7 {
                                                        possible_moves.push((x_position + 1, y_position - 1));
                                                    }
                                                },
                                                _ => {}
                                            }
                                        },
                                        _ => {}
                                    }

                                    if (y_position - 1) == p.position.1 && (x_position + 1) == p.position.0 
                                    && ( (y_position - 1) < -1 || (x_position + 1) < 8 ) {
                                        possible_moves.push((x_position + 1, y_position - 1))
                                    }
        
                                    if (y_position - 1) == p.position.1 && (x_position - 1) == p.position.0 
                                        && ( (y_position - 1) > -1 || (x_position - 1) > -1 ) {
                                        possible_moves.push((x_position - 1, y_position - 1))
                                    }
                                },
                                _ => {}
                            }
                        }

                        if !pawn_blocked {
                            possible_moves.push((x_position, y_position - 1));
                        }

                        self.moves = possible_moves.clone();
                    },
                    Kind::Knight => {
                        possible_moves.push((x_position + 2, y_position + 1));
                        possible_moves.push((x_position + 2, y_position - 1));
                        possible_moves.push((x_position - 2, y_position + 1));
                        possible_moves.push((x_position - 2, y_position - 1));
                        possible_moves.push((x_position + 1, y_position + 2));
                        possible_moves.push((x_position - 1, y_position + 2));
                        possible_moves.push((x_position + 1, y_position - 2));
                        possible_moves.push((x_position - 1, y_position - 2));

                        let mut idx: usize;
                        let mut bad_moves: bool = true;

                        while bad_moves {
                            bad_moves = false;
                            idx = 0;

                            for i in 0..possible_moves.len() {
                                if possible_moves[i].0 < 0 || possible_moves[i].0 > 7 
                                || possible_moves[i].1 < 0 || possible_moves[i].1 > 7
                                || is_blocked_friendly(pieces.clone(), possible_moves[i], PieceColor::Black) {
                                    bad_moves = true;
                                    idx = i;
                                    break;
                                }
                            }

                            if bad_moves {
                                possible_moves.remove(idx);
                            }
                        }

                        self.moves = possible_moves.clone();
                    },
                    Kind::Bishop => {

                    },
                    Kind::Rook => {},
                    Kind::Queen => {},
                    Kind::King => {},
                }
            },
        }
    
        if in_check(pieces.clone(), self.color) {

        }
    }
}

fn is_blocked_friendly(
    pieces: Vec<Piece>,
    pseudo_move: (i32, i32),
    friendly_color: PieceColor,
) -> bool {
    for piece in &pieces {
        if piece.color == friendly_color {
            if piece.position == pseudo_move {
                info!("Cock blocked by {:?}, {:?}", piece, pseudo_move);
                return true;
            }
        }
    }

    false
}