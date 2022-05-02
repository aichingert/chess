use bevy::{prelude::*, ecs::entity::Entities};

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(print_mouse_events_system);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Queen,
    King,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

fn spawn_pieces(
    commands: Commands,
    
) {

}

fn print_mouse_events_system(
    mut mouse_button_input: Res<Input<MouseButton>>,
    mut query: Query<(Entity, &mut Transform, &Piece)>,
) {
    let mut direction: f32 = 0.0;
    let mut piece_transform = query.single_mut();

    if mouse_button_input.pressed(MouseButton::Left) {
        direction -= 1.0;
    }

    if mouse_button_input.pressed(MouseButton::Right) {
        direction += 1.0;
    }

    let new_piece_position = piece_transform.translation.x + direction;


    piece_transform.translation.x = new_piece_position.clamp(-(super::SQUARE_SIZE * 8.0) / 2.0, (super::SQUARE_SIZE * 8.0) / 2.0);
}