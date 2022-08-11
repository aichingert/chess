/*              CHESS CLONE 
By:             Tobias Aichinger
Bevy version:   "0.7" / 2022
*/

use bevy::prelude::*;

// Calculate offset because piece is always placed in the middle
const OFFSET: f32 = (-(8 as f32 / 2.0 * SQUARE_SIZE)) + SQUARE_SIZE / 2.;
const X_OFFSET: f32 = MARGIN / 2.0;
const MARGIN: f32 = 4.0;
const SQUARE_SIZE: f32 = 75.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

mod piece;
mod board;
mod logic;
mod states;
mod menu;

use piece::PiecePlugin;
use board::BoardPlugin;
use states::GameState;
use menu::MenuPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "chess".to_string(),
            width: SQUARE_SIZE * (8.0 + MARGIN),
            height: SQUARE_SIZE * 8.0,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_state(GameState::Menu)
        .add_plugins(DefaultPlugins)
        .add_plugin(MenuPlugin)
        .add_plugin(PiecePlugin)
        .add_plugin(BoardPlugin)
        .run();
}