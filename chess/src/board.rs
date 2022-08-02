use bevy::prelude::*;

use crate::piece::Piece;

const BROWN_COLOR: Color = Color::rgb(181.0 / 255.0, 136.0 / 255.0, 99.0 / 255.0);
const LIGTH_BROWN_COLOR: Color = Color::rgb(240.0 / 255.0, 217.0 / 255.0, 181.0 / 255.0);

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .add_startup_system(create_board)
            .add_system(despawn_taken_pieces)
            .add_system(select_square);
    }
}

#[derive(Component, Debug)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl Square {
    pub fn new(x: u8, y: u8) -> Self {
        Self {
            x,
            y
        }
    }
}

// Point struct used for piece highlighting so you can iterate over the points and remove them after moving
#[derive(Component)]
pub struct Point;

#[derive(Component)]
struct Taken;

#[derive(Default)]
struct SelectedSquare {
    entity: Option<Entity>
}

#[derive(Default)]
struct SelectedPiece {
    entity: Option<Entity>
}

fn select_square(
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut squares_query: Query<(Entity, &Square)>,
    windows: Res<Windows>,
    mut commands: Commands
) {
    // Only run if the left button is pressed
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    let window: &Window = windows.get_primary().unwrap();

    if let Some(pos) = window.cursor_position() {
        let x: u8 = (pos.x / super::SQUARE_SIZE) as u8;
        let y: u8 = (pos.y / super::SQUARE_SIZE) as u8;

        squares_query.for_each_mut( | (entity, square) | {
            if square.x == x && square.y == y {
                commands.entity(entity).insert(Point);
            }
        });
    }
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
                .insert(Square::new(row, column))
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
                .insert(Square::new(row, column))
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

fn despawn_taken_pieces(
    mut commands: Commands,
    query: Query<(Entity, &Piece, &Taken)>
) {
    for (entity, piece, taken) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

