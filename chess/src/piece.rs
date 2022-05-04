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
            texture: asset_server.load("king.png"),
            transform: Transform {
                translation: Vec3::new(super::OFFSET + i as f32 * super::SQUARE_SIZE, super::OFFSET + super::SQUARE_SIZE, 0.0),
                scale: Vec3::new(0.08, 0.08, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Piece::white(Kind::Pawn, (i, 1)));
    }

    // Spawning black pawns
    for i in 0..8 {
        commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("king.png"),
            transform: Transform {
                translation: Vec3::new(super::OFFSET + i as f32 * super::SQUARE_SIZE, super::OFFSET + super::SQUARE_SIZE * 6.0, 0.0),
                scale: Vec3::new(0.08, 0.08, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Piece::black(Kind::Pawn, (i, 6)));
    }
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