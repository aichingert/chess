use bevy::prelude::*;

#[derive()]
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(print_mouse_events_system);
    }
}

#[derive(Component)]
struct Piece;

fn print_mouse_events_system(
    mut mouse_button_input: Res<Input<MouseButton>>,
    mut query: Query<&mut Transform, With<Piece>>,
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
