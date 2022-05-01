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
    color: PieceColor,
    position: (f32, f32),
}

pub trait Move {
    fn standard(piece: Piece, from: (f32, f32), to: (f32, f32)) -> Piece {
        piece
    }
}

impl Piece {
    fn white(kind: Kind, position: (f32, f32)) -> Piece {
        Piece {
            kind,
            color: PieceColor::White,
            position,
        }
    }

    fn black(kind: Kind, position: (f32, f32)) -> Piece {
        Piece {
            kind,
            color: PieceColor::Black,
            position,
        }
    }
}

impl Move for Piece {
    fn standard(piece: Piece, from: (f32, f32), to: (f32, f32)) -> Piece {

        piece
    }

}