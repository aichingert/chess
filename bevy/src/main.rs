use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

mod board;
use board::BoardPlugin;

mod piece;
use piece::{Kind, MovePieceEvent, Piece, PieceColor, PiecePlugin};

mod consts;
use consts::{HEIGHT, WIDTH};

use std::time::Duration;

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
        //.add_systems(Update, test_move.run_if(on_timer(Duration::from_secs(2))))
        .run()
}

/*
fn test_move(
    mut ev_move_piece: EventWriter<MovePieceEvent>,
    mut query: Query<(Entity, &mut Piece)>,
) {
    for (entity, mut piece) in query.iter_mut() {
        if piece.kind == Kind::King && piece.team == PieceColor::White {
            piece.loc.1 += 1;
            ev_move_piece.send(MovePieceEvent(entity));
        }
    }
}
*/

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
