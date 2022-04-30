/*              CHESS CLONE 
By:             Tobias Aichinger
Bevy version:   "0.7" / 2022
*/

use bevy::{prelude::*, window::WindowResizeConstraints};

mod input;

use input::InputPlugin;


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
        .add_plugin(InputPlugin)
        .add_startup_system(create_board)
        .run();
}

#[derive(Component)]
struct Square;


fn create_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Calculate position because it's automaticly placed in the middle
    let center_of_bricks = 0.0;
    let left_edge_of_bricks = center_of_bricks - (8 as f32 / 2.0 * SQUARE_SIZE);
    let bottom_edge_of_bricks = center_of_bricks - (8 as f32 / 2.0 * SQUARE_SIZE);

    let offset_x = left_edge_of_bricks + SQUARE_SIZE / 2.;
    let offset_y = bottom_edge_of_bricks + SQUARE_SIZE / 2.;


    // Create Chessboard 8x8
    for row in 0..8 {
        for column in 0..8 {
            let square_position = Vec2::new(
                offset_x + column as f32 * (SQUARE_SIZE),
                offset_y + row as f32 * (SQUARE_SIZE),
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

    // Adding pieces (random png for now) only temporary here
    commands
    .spawn_bundle(SpriteBundle {
        texture: asset_server.load("king.png"),
        transform: Transform {
            scale: Vec3::new(0.1, 0.1, 1.0),
            ..default()
        },
        ..default()
    });
}