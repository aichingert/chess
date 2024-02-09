use bevy::prelude::*;

mod board;
mod consts;

fn main() { App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, init)
    .run()
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    board::create_board(commands);
}
