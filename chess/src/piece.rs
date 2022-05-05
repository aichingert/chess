/*
    delete this shit now
*/

use bevy::{prelude::*, ecs::{entity::Entities, component::ComponentId, query},  input::mouse::{MouseMotion}};
use core::time;
use std::thread::sleep;

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
        .insert(Piece::white(Kind::Pawn, (i, 1)));
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
    .insert(Piece::white(Kind::King, (4, 0)));
    
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
    .insert(Piece::white(Kind::Queen, (3, 0)));
        
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
    .insert(Piece::white(Kind::Bishop, (2, 0)));
     
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
    .insert(Piece::white(Kind::Bishop, (5, 0)));
 
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
    .insert(Piece::white(Kind::Knight, (1, 0)));
 
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
    .insert(Piece::white(Kind::Knight, (6, 0)));

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
    .insert(Piece::white(Kind::Rook, (0, 0)));

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
    .insert(Piece::white(Kind::Rook, (7, 0)));
 
 
 
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
        .insert(Piece::black(Kind::Pawn, (i, 6)));
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
    .insert(Piece::black(Kind::King, (4, 7)));

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
    .insert(Piece::black(Kind::Queen, (3, 7)));
        
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
    .insert(Piece::black(Kind::Bishop, (2, 7)));
    
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
    .insert(Piece::black(Kind::Bishop, (5, 7)));

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
    .insert(Piece::black(Kind::Knight, (1, 7)));

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
    .insert(Piece::black(Kind::Knight, (6, 7)));

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
    .insert(Piece::black(Kind::Rook, (0, 7)));

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
    .insert(Piece::black(Kind::Rook, (7, 7)));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Queen,
    King,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceColor {
    Black,
    White,
}


#[derive(Component, Debug)]
pub struct Piece {
    kind: Kind,
    color: PieceColor,
    position: (i32, i32),
}

impl Piece {
    fn white(kind: Kind, position: (i32, i32)) -> Piece {
        Piece {
            kind,
            color: PieceColor::White,
            position,
        }
    }

    fn black(kind: Kind, position: (i32, i32)) -> Piece {
        Piece {
            kind,
            color: PieceColor::Black,
            position,
        }
    }
}


fn detection_system(
    mouse_button_input: Res<Input<MouseButton>>,
    piece_query: Query<&Piece>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(pos) = window.cursor_position() {
        let x = pos.x / super::SQUARE_SIZE;
        let y = pos.y / super::SQUARE_SIZE;

        if mouse_button_input.just_pressed(MouseButton::Left) {
            //move_system(query, (x as i32, y as i32));
            if check_if_piece_is_on_position(piece_query, (x as i32, y as i32)) {
                info!("piece found");
            } else {
                info!("no piece there");
            }
        }
    }
}

fn check_if_piece_is_on_position(
    query: Query<&Piece>,
    pressed_pos: (i32, i32)
) -> bool {
    let mut is_there_a_piece = false;

    query.for_each( | piece| {
        if piece.position.0 == pressed_pos.0 && piece.position.1 == pressed_pos.1 {
            info!("{:?}", piece);
            is_there_a_piece = true;
        }
    });

    is_there_a_piece
}

fn move_system(
    mut query: Query<(&mut Transform, &mut Piece)>,
    position: (i32, i32)
) {
    for (mut transform, mut pieces) in query.iter_mut() {
        let mut x = super::OFFSET;
        let mut y =super::OFFSET;
        let mut transforming: bool = false;

        match pieces.color {
            PieceColor::White => {
                match pieces.kind {
                    Kind::Pawn => {
                        if pieces.position.1 + 1 < 7 {
                            pieces.position.1 += 1;
                            y += pieces.position.1 as f32 * super::SQUARE_SIZE;
                            x += pieces.position.0 as f32 * super::SQUARE_SIZE;
                            transforming = true;
                        }
                    }
                    Kind::Knight => {
        
                    }
                    Kind::Bishop => {
        
                    }
                    Kind::Rook => {
        
                    }
                    Kind::Queen => {
        
                    }
                    Kind::King => {
                        
                    }
                    _ => {},
                }
            }
            PieceColor::Black => {
                match pieces.kind {
                    Kind::Pawn => {
                        if pieces.position.1 - 1 > 0 {
                            pieces.position.1 -= 1;
                            y += pieces.position.1 as f32 * super::SQUARE_SIZE;
                            x += pieces.position.0 as f32 * super::SQUARE_SIZE;
                            transforming = true;
                        }
                    }
                    Kind::Knight => {
        
                    }
                    Kind::Bishop => {
        
                    }
                    Kind::Rook => {
        
                    }
                    Kind::Queen => {
        
                    }
                    Kind::King => {
                        
                    }
                    _ => {},
                }
            }
        }

        if transforming {
            transform.translation = Vec3::new(x, y, 0.0);
        }
    }
}