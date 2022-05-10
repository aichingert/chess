/*
    delete this shit now
*/

use bevy::prelude::*;

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_pieces)
            .add_system(detection_system);
    }
}

fn spawn_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    // Spawning white pawns
    for i in 0..8 {
        commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("white/pawn.png"),
            transform: Transform {
                translation: Vec3::new(super::OFFSET + i as f32* super::SQUARE_SIZE, super::OFFSET + super::SQUARE_SIZE, 0.0),
                scale: Vec3::new(0.4, 0.4, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Piece::white(Kind::Pawn, (i, 1), EnPassantStates::Waiting));
    }

    // spawn white king
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("white/king.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 4.0 * super::SQUARE_SIZE, super::OFFSET, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::white(Kind::King, (4, 0), EnPassantStates::Done));
    
    // spawn white queen
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("white/queen.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 3.0 * super::SQUARE_SIZE, super::OFFSET, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::white(Kind::Queen, (3, 0), EnPassantStates::Done));
        
    // spawn white bishop left
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("white/bishop.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 2.0 * super::SQUARE_SIZE, super::OFFSET, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::white(Kind::Bishop, (2, 0), EnPassantStates::Done));
     
    // spawn white bishop right
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("white/bishop.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 5.0 * super::SQUARE_SIZE, super::OFFSET, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::white(Kind::Bishop, (5, 0), EnPassantStates::Done));
 
    // spawn white knight left
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("white/knight.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 1.0 * super::SQUARE_SIZE, super::OFFSET, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::white(Kind::Knight, (1, 0), EnPassantStates::Done));
 
    // spawn white knight right
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("white/knight.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 6.0 * super::SQUARE_SIZE, super::OFFSET, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::white(Kind::Knight, (6, 0), EnPassantStates::Done));

    // spawn white rook left
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("white/rook.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET, super::OFFSET, 0.0),
            scale: Vec3::new(0.4, 0.4, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::white(Kind::Rook, (0, 0), EnPassantStates::Done));

    // spawn white rook right
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("white/rook.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 7.0 * super::SQUARE_SIZE, super::OFFSET, 0.0),
            scale: Vec3::new(0.4, 0.4, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::white(Kind::Rook, (7, 0), EnPassantStates::Done));
 
 
 
    // Spawning black pawns
    for i in 0..8 {
        commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("black/pawn.png"),
            transform: Transform {
                translation: Vec3::new(super::OFFSET + i as f32 * super::SQUARE_SIZE, super::OFFSET + super::SQUARE_SIZE * 6.0, 0.0),
                scale: Vec3::new(0.4, 0.4, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Piece::black(Kind::Pawn, (i, 6), EnPassantStates::Waiting));
    }

    // spawn black king
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("black/king.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 4.0 * super::SQUARE_SIZE, super::OFFSET + 7.0 * super::SQUARE_SIZE, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::black(Kind::King, (4, 7), EnPassantStates::Done));

    // spawn black queen
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("black/queen.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 3.0 * super::SQUARE_SIZE, super::OFFSET + 7.0 * super::SQUARE_SIZE, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::black(Kind::Queen, (3, 7), EnPassantStates::Done));
        
    // spawn black bishop left
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("black/bishop.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 2.0 * super::SQUARE_SIZE, super::OFFSET + 7.0 * super::SQUARE_SIZE, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::black(Kind::Bishop, (2, 7), EnPassantStates::Done));
    
    // spawn black bishop right
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("black/bishop.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 5.0 * super::SQUARE_SIZE, super::OFFSET + 7.0 * super::SQUARE_SIZE, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::black(Kind::Bishop, (5, 7), EnPassantStates::Done));

    // spawn black knight left
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("black/knight.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 1.0 * super::SQUARE_SIZE, super::OFFSET + 7.0 * super::SQUARE_SIZE, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::black(Kind::Knight, (1, 7), EnPassantStates::Done));

    // spawn black knight right
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("black/knight.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 6.0 * super::SQUARE_SIZE, super::OFFSET + 7.0 * super::SQUARE_SIZE, 0.0),
            scale: Vec3::new(0.4, 0.3, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::black(Kind::Knight, (6, 7), EnPassantStates::Done));

    // spawn black rook left
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("black/rook.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET, super::OFFSET + 7.0 * super::SQUARE_SIZE, 0.0),
            scale: Vec3::new(0.4, 0.4, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::black(Kind::Rook, (0, 7), EnPassantStates::Done));

    // spawn black rook right
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("black/rook.png"),
        transform: Transform {
            translation: Vec3::new(super::OFFSET + 7.0 * super::SQUARE_SIZE, super::OFFSET + 7.0 * super::SQUARE_SIZE, 0.0),
            scale: Vec3::new(0.4, 0.4, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Piece::black(Kind::Rook, (7, 7), EnPassantStates::Done));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Queen,
    King,
    Rook,
    Bishop,
    Knight,
    Pawn,
    NoPiece,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceColor {
    Black,
    White,
}

#[derive(Component, Debug, Clone, Copy)]
struct Move {
    position: (i32, i32),
}

#[derive(Component, Debug, Clone)]
pub struct Piece {
    kind: Kind,
    color: PieceColor,
    position: (i32, i32),
    moves: Vec<(i32, i32)>,
    en_passant: EnPassantStates,
}

#[derive(Component, Debug, Clone)]
enum EnPassantStates {
    Ready,
    Waiting,
    Done,
}

impl Piece {
    fn white(kind: Kind, position: (i32, i32), en_passant_state: EnPassantStates) -> Piece {
        Piece {
            kind,
            color: PieceColor::White,
            position,
            moves: Vec::new(),
            en_passant: en_passant_state,
        }
    }
    
    fn black(kind: Kind, position: (i32, i32), en_passant_state: EnPassantStates) -> Piece {
        Piece {
            kind,
            color: PieceColor::Black,
            position,
            moves: Vec::new(),
            en_passant: en_passant_state,
        }
    }

    fn check_possible_moves(&mut self, pieces: Vec<Piece>) {
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
                        let mut pos_mov: (i32, i32) = (-1, -1);

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
                                                        pos_mov = (x_position - 1, y_position + 1);
                                                    }

                                                    if y_position == p.position.1 && x_position + 1 == p.position.0 && (x_position + 1) <= 7 {
                                                        pos_mov = (x_position + 1, y_position + 1);
                                                    }

                                                    info!("{:?}", p);
                                                },
                                                _ => {}
                                            }
                                        },
                                        _ => {}
                                    }
                                },
                                _ => {}
                            }
                        }

                        if !pawn_blocked {
                            if pos_mov.0 >= 0 {
                                possible_moves.push(pos_mov);
                            }
                            possible_moves.push((x_position, y_position + 1));
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
                        let mut pos_mov: (i32, i32) = (-1, -1);

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
                                                        pos_mov = (x_position - 1, y_position - 1);
                                                    }
                
                                                    if y_position == p.position.1 && x_position + 1 == p.position.0 && (x_position + 1) <= 7 {
                                                        pos_mov = (x_position + 1, y_position - 1);
                                                    }
                                                },
                                                _ => {}
                                            }
                                        },
                                        _ => {}
                                    }
                                },
                                _ => {}
                            }
                        }

                        if !pawn_blocked {
                            if pos_mov.1 >= 0 {
                                possible_moves.push(pos_mov);
                            }
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
    }
    
    fn promotion(&mut self, to: Kind) {
        if self.kind == Kind::Pawn {
            // Issue: Add => piece despawn and respawning new piece
    
            self.kind = to;
        }
    }
}

fn detection_system(
    mut mouse_button_input: ResMut<Input<MouseButton>>,
    mut piece_query: Query<(&mut Transform, &mut Piece)>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(pos) = window.cursor_position() {
        let x: i32 = (pos.x / super::SQUARE_SIZE) as i32;
        let y: i32 = (pos.y / super::SQUARE_SIZE) as i32;

        let mut transform_x = super::OFFSET;
        let mut transform_y = super::OFFSET;

        let mut pieces_on_the_board: Vec<Piece> = Vec::new();

        if mouse_button_input.just_released(MouseButton::Left) {
            piece_query.for_each( | query_info | {
                pieces_on_the_board.push(query_info.1.clone());
            });

            piece_query.for_each_mut( | (mut transform_piece,mut piece) | {
                if piece.position.0 == x && piece.position.1 == y {
                    piece.check_possible_moves(pieces_on_the_board.clone());

                    let postitions = piece.moves.clone();

                    let piece_x: i32 = (pos.x / super::SQUARE_SIZE) as i32;
                    let piece_y: i32 = (pos.y / super::SQUARE_SIZE) as i32;

                    if postitions.len() > 0 {
                        transform_x += postitions[0].0 as f32 * super::SQUARE_SIZE;
                        transform_y += postitions[0].1 as f32 * super::SQUARE_SIZE;

                        piece.position.0 = postitions[0].0;
                        piece.position.1 = postitions[0].1;
    
                        transform_piece.translation = Vec3::new(transform_x, transform_y, 1.0);
                        
                        info!("{:?}", postitions);
                    }

                    /* code used to transform piece 1 square up

                    if let Some(new_pos) = window.cursor_position() {

                        let piece_x: i32 = (pos.x / super::SQUARE_SIZE) as i32;
                        let piece_y: i32 = (pos.y / super::SQUARE_SIZE) as i32;

                        piece.position.0 = piece_x;
                        piece.position.1 = piece_y + 1;

                        transform_x += piece.position.0 as f32 * super::SQUARE_SIZE;
                        transform_y += piece.position.1 as f32 * super::SQUARE_SIZE;
                        
                        transform_piece.translation = Vec3::new(transform_x, transform_y, 1.0);
                    }
                    */
                }
            });
        }
    }
}
