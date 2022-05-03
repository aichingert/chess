/*              CHESS CLONE 
By:             Tobias Aichinger
Bevy version:   "0.7" / 2022
*/

use bevy::{prelude::*, window::WindowResizeConstraints};

// Calculate offset because piece is always placed in the middle
const OFFSET: f32 = (-(8 as f32 / 2.0 * SQUARE_SIZE)) + SQUARE_SIZE / 2.;

const SQUARE_SIZE: f32 = 75.0;

mod piece;
mod board;

use piece::*;
use board::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "chess".to_string(),
            width: SQUARE_SIZE * 8.0,
            height: SQUARE_SIZE * 8.0,
            resize_constraints: WindowResizeConstraints {
                min_width: SQUARE_SIZE * 8.0,
                min_height: SQUARE_SIZE * 8.0,
                max_width: SQUARE_SIZE * 8.0,
                max_height: SQUARE_SIZE * 8.0,
            },
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PiecePlugin)
        .add_plugin(BoardPlugin)
        .run();
}