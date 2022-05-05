use std::os::windows;

use bevy::prelude::*;

const BROWN_COLOR: Color = Color::rgb(181.0 / 255.0, 136.0 / 255.0, 99.0 / 255.0);
const LIGTH_BROWN_COLOR: Color = Color::rgb(240.0 / 255.0, 217.0 / 255.0, 181.0 / 255.0);

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_board);
    }
}

#[derive(Component)]
pub struct Square {
    pos: (i32, i32),
}

fn create_board(
    mut commands: Commands
)   {

    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Create Chessboard 8x8
    for row in 0..8 {
        for column in 0..8 {
            let square_position = Vec2::new(
                super::OFFSET + column as f32 * (super::SQUARE_SIZE),
                super::OFFSET + row as f32 * (super::SQUARE_SIZE),
            );

            if (row + column) % 2 != 0 {
                // Insert brown square
                commands
                .spawn()
                .insert(Square {
                    pos: (row, column)
                })
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: LIGTH_BROWN_COLOR,
                        ..default()
                    },
                    transform: Transform {
                        translation: square_position.extend(0.0),
                        scale: Vec3::new(super::SQUARE_SIZE, super::SQUARE_SIZE, 0.0),
                        ..default()
                    },
                    ..default()
                });
            }
            else {
                // Insert white square 
                commands
                .spawn()
                .insert(Square {
                    pos: (row, column)
                })
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: BROWN_COLOR,
                        ..default()
                    },
                    transform: Transform {
                        translation: square_position.extend(0.0),
                        scale: Vec3::new(super::SQUARE_SIZE, super::SQUARE_SIZE, 1.0),
                        ..default()
                    },
                    ..default()
                });
            }
        }
    }
}
