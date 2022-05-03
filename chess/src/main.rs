/*              CHESS CLONE 
By:             Tobias Aichinger
Bevy version:   "0.7" / 2022
*/

use bevy::{prelude::*, window::WindowResizeConstraints};

mod piece;

use piece::*;

// Calculate offset because piece is always placed in the middle
const OFFSET: f32 = (-(8 as f32 / 2.0 * SQUARE_SIZE)) + SQUARE_SIZE / 2.;

const SQUARE_SIZE: f32 = 75.0;
const BROWN_COLOR: Color = Color::rgb(181.0 / 255.0, 136.0 / 255.0, 99.0 / 255.0);
const LIGTH_BROWN_COLOR: Color = Color::rgb(240.0 / 255.0, 217.0 / 255.0, 181.0 / 255.0);

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
        .add_startup_system(create_board)
        .run();
}

#[derive(Component)]
struct Square;

#[derive(Component)]
struct King;

fn create_board(mut commands: Commands) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());


    // Create Chessboard 8x8
    for row in 0..8 {
        for column in 0..8 {
            let square_position = Vec2::new(
                OFFSET + column as f32 * (SQUARE_SIZE),
                OFFSET + row as f32 * (SQUARE_SIZE),
            );

            if (row + column) % 2 != 0 {
                // Insert brown square
                commands
                .spawn()
                .insert(Square)
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: LIGTH_BROWN_COLOR,
                        ..default()
                    },
                    transform: Transform {
                        translation: square_position.extend(0.0),
                        scale: Vec3::new(SQUARE_SIZE, SQUARE_SIZE, 0.0),
                        ..default()
                    },
                    ..default()
                });
            }
            else {
                // Insert white square 
                commands
                .spawn()
                .insert(Square)
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: BROWN_COLOR,
                        ..default()
                    },
                    transform: Transform {
                        translation: square_position.extend(0.0),
                        scale: Vec3::new(SQUARE_SIZE, SQUARE_SIZE, 1.0),
                        ..default()
                    },
                    ..default()
                });
            }
        }
    }
}