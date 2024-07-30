use bevy::prelude::*;

mod board;
use board::BoardPlugin;

mod piece;
use piece::PiecePlugin;

mod consts;
use consts::{HEIGHT, WIDTH};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WIDTH, HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(BoardPlugin)
        .add_plugins(PiecePlugin)
        .add_systems(Startup, init)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
