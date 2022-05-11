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

                    },
                    Kind::Bishop => {},
                    Kind::Rook => {},
                    Kind::Queen => {},
                    Kind::King => {},
                    _ => {}
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
                    Kind::Knight => {},
                    Kind::Bishop => {},
                    Kind::Rook => {},
                    Kind::Queen => {},
                    Kind::King => {},
                    _ => {}
                }
            },
        }
    
        if in_check(pieces.clone(), self.color) {

        }
    }
}