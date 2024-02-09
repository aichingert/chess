use bevy::prelude::*;

use crate::piece::{PieceColor, Turn};
use crate::states::GameState;
use crate::board::{GameFinishedEvent, Winner};

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(setup_timer)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_timer)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Playing)
                    .with_system(cleanup_timer)
            );
    }
}

pub struct TimerPlugin;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Timer {
    time: f32,
    id: u8
}

impl Timer {
    fn new(id: u8) -> Self {
        Self { 
            time: super::TIME_PER_PLAYER, 
            id
        }
    }
}

impl Into<std::string::String> for Timer {
    fn into(self) -> std::string::String {
        let minutes: i32 = self.time as i32 / 60;
        let seconds: i32 = self.time as i32 % 60;

        format!("{minutes}:{:02}", seconds)
    }
}

impl std::fmt::Display for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let minutes: i32 = self.time as i32 / 60;
        let seconds: i32 = self.time as i32 % 60;
        
        write!(f, "{minutes}:{:02}", seconds)
    }
}

fn setup_timer(
    mut commands: Commands,
    materials: Res<AssetServer>
) {
    let text_style = TextStyle {
        font: materials.load("fonts/FiraSans-Bold.ttf"),
        font_size: 60.0,
        color: super::TEXT_COLOR,
    };

    let box_size = Vec2::new(300.0, 200.0);
    let mut box_position = Vec2::new(-350.0, -50.0);

    let white_timer: Timer = Timer::new(1);
    let black_timer: Timer = Timer::new(2);

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(white_timer, text_style.clone()),
        text_2d_bounds: bevy::text::Text2dBounds {
        // Wrap text in the rectangle
            size: box_size,
        },
        transform: Transform::from_xyz(
            box_position.x,
            box_position.y,
            1.0,
        ),
        ..default()
    })
    .insert(white_timer);

    box_position = Vec2::new(-350.0, 150.0);

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(black_timer, text_style),
        text_2d_bounds: bevy::text::Text2dBounds {
        // Wrap text in the rectangle
            size: box_size,
        },
        transform: Transform::from_xyz(
            box_position.x,
            box_position.y,
            1.0,
        ),
        ..default()
    })
    .insert(black_timer);
}

fn update_timer(
    mut timer_query: Query<(&mut Text, &mut Timer)>,
    mut game_end_event: EventWriter<GameFinishedEvent>,
    mut winner: ResMut<Winner>,
    turn: Res<Turn>,
    time: Res<Time>
) {
    let timer_id: u8;

    if turn.0 == PieceColor::White {
        timer_id = 1;
    } else {
        timer_id = 2;
    }

    for (mut text, mut timer) in timer_query.iter_mut() {
        if timer.time <= 0.0 {
            winner.winner = Some( match turn.0 {
                PieceColor::Black => PieceColor::White,
                PieceColor::White => PieceColor::Black,
            });
            game_end_event.send(GameFinishedEvent);
        }

        if timer.id == timer_id {
            timer.time -= time.delta_seconds();
        }

        text.sections[0].value = timer.to_string();
    }
}

fn cleanup_timer(
    mut commands: Commands,
    timer_query: Query<(Entity, &Timer)>
) {
    for (entity, _) in timer_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}