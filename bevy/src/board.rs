use bevy::prelude::*;

use crate::consts::{COLORS, SQUARE_SIZE, OFFSET};


pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_board);
    }
}

#[derive(Component)]
struct Square {
    x: u8,
    y: u8,
    
}

fn create_board(mut commands: Commands) {
    for row in 0..8 {
        for col in 0..8 {
            let pos = Vec3::new(
                OFFSET + col as f32 * SQUARE_SIZE, 
                OFFSET + row as f32 * SQUARE_SIZE, 0.
            );

            commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: COLORS[((row + col) & 1) as usize],
                            ..default()
                        },
                        transform: Transform {
                            translation: pos,
                            scale: Vec3::new(SQUARE_SIZE, SQUARE_SIZE, 0.),
                            ..default()
                        },
                        ..default()
                    },
                    Square { x: col, y: row },
            ));
        }
    }
}
