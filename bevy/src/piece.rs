use bevy::prelude::*;

use crate::consts::{SQUARE_SIZE, OFFSET};

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
       app
           .add_event::<MovePieceEvent>()
           .add_systems(Startup, create_pieces)
           .add_systems(Update, move_piece); 
    }
}

#[derive(Event)]
pub struct MovePieceEvent(pub Entity);

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

    fn get_vec3(&self) -> Vec3 {
        Vec3::new(OFFSET + self.loc.0 as f32 * SQUARE_SIZE, OFFSET + self.loc.1 as f32 * SQUARE_SIZE, 1.)
    }
}

fn create_pieces(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        
        commands.spawn((create_spritebundle(&w_pawn), w_pawn));
        commands.spawn((create_spritebundle(&white), white));
        commands.spawn((create_spritebundle(&b_pawn), b_pawn));
        commands.spawn((create_spritebundle(&black), black));
    }
}

fn move_piece(mut piece_move_ev: EventReader<MovePieceEvent>, mut query: Query<(&Piece, &mut Transform)>) {
    for ev in piece_move_ev.read() {
        if let Ok((piece, mut transform)) = query.get_mut(ev.0) {
            transform.translation = piece.get_vec3();
        }
    }
}
