use bevy::prelude::*;
use crate::states::GameState;

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

#[derive(Component, Debug, Clone)]
pub struct Piece {
    pub kind: Kind,
    pub color: PieceColor,
    pub pos: (u8, u8),
}

// Turn struct has the current color that has the move
pub struct Turn(pub PieceColor);

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_pieces)
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_pieces)
            );
    }
}

impl Default for Turn {
    fn default() -> Self {
        Self(PieceColor::White)
    }
}

// has the change function after move the turn changes to the enemy color
// white => black
// black => white
impl Turn {
    pub fn change(&mut self) {
        match self.0 {
            PieceColor::White => {
                self.0 = PieceColor::Black;
            },
            PieceColor::Black => {
                self.0 = PieceColor::White;
            },
        }
    }
}

// Implements the constructors for the a white piece and the black piece
// also the function promotion => which promotes a pawn to any piece you want, except a pawn again and a king
impl Piece {
    pub fn white(kind: Kind, position: (u8, u8)) -> Piece {
        Piece {
            kind,
            color: PieceColor::White,
            pos: position,
        }
    }
    
    pub fn black(kind: Kind, position: (u8, u8)) -> Piece {
        Piece {
            kind,
            color: PieceColor::Black,
            pos: position,
        }
    }
}


fn spawn_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let pieces: [Kind; 8] = [Kind::Rook, Kind::Knight, Kind::Bishop, Kind::Queen, Kind::King, Kind::Bishop, Kind::Knight, Kind::Rook];
    let path: [&str; 8] = ["rook.png", "knight.png", "bishop.png", "queen.png", "king.png", "bishop.png", "knight.png", "rook.png"];

    //
    // Spawning pawns
    //

    for i in 0..8 {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("white/pawn.png"),
                transform: Transform {
                    translation: Vec3::new(super::OFFSET + i as f32 * super::SQUARE_SIZE, super::OFFSET + super::SQUARE_SIZE, 1.0),
                    scale: Vec3::new(0.4, 0.4, 1.0),
                    ..default()
                },
                ..default()
            })
            .insert(Piece::white(Kind::Pawn, (i, 1)));

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
    
    //
    // Spawning pieces
    //

    for i in 0..8 {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load(&format!("white/{}", path[i])),
                transform: Transform {
                    translation: Vec3::new(super::OFFSET + i as f32 * super::SQUARE_SIZE, super::OFFSET, 1.0),
                    scale: Vec3::new(0.4, 0.3, 1.0),
                    ..default()
                },
                ..default()
            })
            .insert(Piece::white(pieces[i], (i as u8, 0)));

        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load(&format!("black/{}", path[i])),
                transform: Transform {
                    translation: Vec3::new(super::OFFSET + i as f32 * super::SQUARE_SIZE, super::OFFSET + 7.0 * super::SQUARE_SIZE, 1.0),
                    scale: Vec3::new(0.4, 0.3, 1.0),
                    ..default()
                },
                ..default()
            })
            .insert(Piece::black(pieces[i], (i as u8, 7)));
    }
}

fn move_pieces(_time: Res<Time>, mut query: Query<(&mut Transform, &Piece)>) {
    for (mut transform, piece) in query.iter_mut() {
        if transform.translation.x != piece.pos.0 as f32 && transform.translation.y != piece.pos.1 as f32 {
            transform.translation = Vec3::new(super::OFFSET + piece.pos.0 as f32 * super::SQUARE_SIZE, super::OFFSET + piece.pos.1 as f32 * super::SQUARE_SIZE, 1.0);
        }

        /* if you want to see the pieces moving

            // Get the direction to move in
            let direction = Vec3::new(super::OFFSET + piece.pos.0 as f32 * super::SQUARE_SIZE, super::OFFSET + piece.pos.1 as f32 * super::SQUARE_SIZE, 1.0) - transform.translation;

            // Only move if the piece isn't already there (distance is big)
            if direction.length() > 0.1 {
                transform.translation += direction.normalize() * (time.delta_seconds() * 100.);
            }
        */
    }
}