/*              CHESS CLONE 
By:             Tobias Aichinger
Bevy version:   "0.7" / 2022
*/

use bevy::prelude::*;

// Calculate offset because piece is always placed in the middle
const OFFSET: f32 = (-(8 as f32 / 2.0 * SQUARE_SIZE)) + SQUARE_SIZE / 2.;
const X_OFFSET: f32 = MARGIN / 2.0;
const MARGIN: f32 = 3.0;
const SQUARE_SIZE: f32 = 75.0;
const WIDTH: f32 = SQUARE_SIZE * (8.0 + MARGIN);
const HEIGHT: f32 = SQUARE_SIZE * 8.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const TEXT_COLOR: Color = Color::WHITE;

const TIME_PER_PLAYER: i32 = 180;

mod piece;
mod board;
mod logic;
mod states;
mod menu;
mod timer;

use piece::PiecePlugin;
use board::BoardPlugin;
use states::GameState;
use menu::MenuPlugin;
use timer::TimerPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "chess".to_string(),
            width: WIDTH,
            height: HEIGHT,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_state(GameState::Menu)
        .add_plugins(DefaultPlugins)
        .add_plugin(MenuPlugin)
        .add_plugin(TimerPlugin)
        .add_plugin(PiecePlugin)
        .add_plugin(BoardPlugin)
        .run();
}