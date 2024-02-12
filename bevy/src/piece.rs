use bevy::prelude::*;

use crate::consts::{SQUARE_SIZE, OFFSET};
use crate::board::Board;

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
       app
           .add_event::<MovePieceEvent>()
           .add_event::<TakePieceEvent>()
           .add_systems(Startup, create_pieces)
           .add_systems(Update, move_piece)
           .add_systems(Update, take_piece); 
    }
}

#[derive(Event)]
pub struct MovePieceEvent(pub (Entity, (u8, u8)));

#[derive(Event)]
pub struct TakePieceEvent(pub Entity);

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

#[derive(Component, Copy, Clone, Eq, PartialEq)]
pub struct Piece {
    pub kind: Kind,
    pub team: PieceColor,
    pub loc: (u8, u8),
}

impl Piece {
    fn new(team: PieceColor, kind: Kind, loc: (u8, u8)) -> Self {
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

    pub fn get_vec3(&self) -> Vec3 {
        Vec3::new(OFFSET + self.loc.0 as f32 * SQUARE_SIZE, OFFSET + self.loc.1 as f32 * SQUARE_SIZE, 1.)
    }
}

fn create_pieces(mut commands: Commands, asset_server: Res<AssetServer>, mut board: ResMut<Board>) {
    use Kind::*;
    let pieces = [Rook, Knight, Bishop, King, Queen, Bishop, Knight, Rook];
    
    let create_spritebundle = |piece: &Piece| -> SpriteBundle {
        SpriteBundle {
            texture: asset_server.load(piece.get_asset_path()),
            transform: Transform {
                translation: piece.get_vec3(),
                scale: Vec3::new(0.4, 0.35, 1.0),
                ..default()
            },
            ..default()
        }
    };

    for i in 0..8 {
        let w_pawn = Piece::new(PieceColor::White, Kind::Pawn, (i, 1));
        let b_pawn = Piece::new(PieceColor::Black, Kind::Pawn, (i, 6));
        let white = Piece::new(PieceColor::White, pieces[i as usize], (i, 0));
        let black = Piece::new(PieceColor::Black, pieces[i as usize], (i, 7));
        
        board.add_entity(1, i as usize, commands.spawn((create_spritebundle(&w_pawn), w_pawn)).id());
        board.add_entity(6, i as usize, commands.spawn((create_spritebundle(&b_pawn), b_pawn)).id());
        board.add_entity(0, i as usize, commands.spawn((create_spritebundle(&white), white)).id());
        board.add_entity(7, i as usize, commands.spawn((create_spritebundle(&black), black)).id());
    }
}

fn move_piece(mut piece_move_ev: EventReader<MovePieceEvent>, mut query: Query<(&mut Piece, &mut Transform)>) {
    for ev in piece_move_ev.read() {
        if let Ok((mut piece, mut transform)) = query.get_mut(ev.0.0) {
            piece.loc = ev.0.1;
            transform.translation = piece.get_vec3();
        }
    }
}

fn take_piece(mut commands: Commands, mut piece_take_ev: EventReader<TakePieceEvent>) {
    for ev in piece_take_ev.read() {
        commands.entity(ev.0).despawn_recursive();
    }
}
