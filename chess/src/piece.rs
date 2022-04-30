use bevy::{prelude::*, ecs::entity::Entities};

pub enum Kind {
    Queen,
    King,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

pub enum PieceColor {
    Black,
    White,
}


#[derive(Component)]
pub struct Piece {
    kind: Kind,
    color: Color,
    position: (f32, f32),
}

impl Piece {
    pub fn white(kind: Kind, position: (f32, f32)) -> self {
        Piece {
            kind,
            color: PieceColor::White,
            position,
        }
    }

    pub fn black(kind: Kind, position: (f32, f32)) -> self {
        Piece {
            kind,
            color: PieceColor::Black,
            position,
        }
    }
}

pub impl CheckPawnMoves for Piece::Pawn {
    fn standard(from: (f32, f32), to: (f32, f32)) -> bool {

    }

    fn en_passant(from: (f32, f32), to: (f32, f32)) -> bool {

    }
}

pub impl PawnMoves for Piece::Pawn {
    fn standard(from: (f32, f32), to: (f32, f32), id: Entity) -> Entity {

    }

    fn en_passant(from: (f32, f32), to: (f32, f32), id: Entity) -> Entity {
        
    }
}