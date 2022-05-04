use bevy::{prelude::*, ecs::{entity::Entities, component::ComponentId},  input::mouse::{MouseMotion}};

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_system)
        .add_startup_system(spawn_pieces);
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
    .insert(Piece::white(Kind::King, (4, 1)));
    
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
    .insert(Piece::white(Kind::Queen, (3, 1)));
        
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
    .insert(Piece::white(Kind::Bishop, (2, 1)));
     
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
    .insert(Piece::white(Kind::Bishop, (5, 1)));
 
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
    .insert(Piece::white(Kind::Knight, (1, 1)));
 
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
    .insert(Piece::white(Kind::Knight, (6, 1)));

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
    .insert(Piece::white(Kind::Rook, (0, 1)));

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
    .insert(Piece::white(Kind::Rook, (7, 1)));
 
 
 
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
    .insert(Piece::black(Kind::King, (4, 1)));

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
    .insert(Piece::black(Kind::Queen, (3, 1)));
        
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
    .insert(Piece::black(Kind::Bishop, (2, 1)));
    
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
    .insert(Piece::black(Kind::Bishop, (5, 1)));

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
    .insert(Piece::black(Kind::Knight, (1, 1)));

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
    .insert(Piece::black(Kind::Knight, (6, 1)));

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
    .insert(Piece::black(Kind::Rook, (0, 1)));

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
    .insert(Piece::black(Kind::Rook, (7, 1)));
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


fn move_system() {}