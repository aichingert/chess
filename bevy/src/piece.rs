use bevy::prelude::*;

use crate::consts::{SQUARE_SIZE, OFFSET};

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
       app.add_systems(Startup, create_pieces); 
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Kind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PieceColor {
    Black,
    White,
}

#[derive(Component)]
pub struct Piece {
    kind: Kind,
    team: PieceColor,
    loc: (u8, u8),
}

impl Piece {
    fn new(kind: Kind, team: PieceColor, loc: (u8, u8)) -> Self {
        Self { kind, team, loc }
    }
    
    fn get_asset_path(&self) -> String {
        let mut path = match self.team {
            PieceColor::Black => String::from("black/"),
            PieceColor::White => String::from("white/"),
        };
        
        path.push_str(match self.kind {
            Kind::Pawn => "pawn.png",
            Kind::Knight => "knight.png",
            Kind::Bishop => "bishop.png",
            Kind::Rook => "rook.png",
            Kind::Queen => "queen.png",
            Kind::King => "king.png",          
        });
        
        path
    }
}

fn create_pieces(mut commands: Commands, asset_server: Res<AssetServer>) {
    use Kind::*;
    let pieces = [Rook, Knight, Bishop, King, Queen, Bishop, Knight, Rook];
    
    let create_spritebundle = |piece: &Piece| -> SpriteBundle {
        let pos = Vec3::new(OFFSET + piece.loc.0 as f32 * SQUARE_SIZE, OFFSET + piece.loc.1 as f32 * SQUARE_SIZE, 1.0);
        
        SpriteBundle {
            texture: asset_server.load(piece.get_asset_path()),
            transform: Transform {
                translation: pos,
                scale: Vec3::new(0.4, 0.35, 1.0),
                ..default()
            },
            ..default()
        }
    };

    for i in 0..8 {
        let w_pawn = Piece { team: PieceColor::White, kind: Kind::Pawn, loc: (i, 1) };
        let white = Piece { team: PieceColor::White, kind: pieces[i as usize], loc: (i, 0) };
        let b_pawn = Piece { team: PieceColor::Black, kind: Kind::Pawn, loc: (i, 6) };
        let black = Piece { team: PieceColor::Black, kind: pieces[i as usize], loc: (i, 7) };
        
        commands.spawn((create_spritebundle(&w_pawn), w_pawn));
        commands.spawn((create_spritebundle(&white), white));
        commands.spawn((create_spritebundle(&b_pawn), b_pawn));
        commands.spawn((create_spritebundle(&black), black));
    }
}
