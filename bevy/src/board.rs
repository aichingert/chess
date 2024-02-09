use bevy::prelude::*;

use crate::consts::{BROWN_COLOR, LIGHT_BROWN_COLOR, SQUARE_SIZE, OFFSET};


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

pub fn create_board(mut commands: Commands) {
    for row in 0..8 {
        for col in 0..8 {          
            let color = if (row + col) & 1 == 1 {
                BROWN_COLOR              
            } else {
                LIGHT_BROWN_COLOR              
            };
 
            create_square(&mut commands, color, row, col);
        }
    }
}

fn create_square(commands: &mut Commands, color: Color, y: u8, x: u8) {  
    let pos = Vec2::new(
        OFFSET + x as f32 * SQUARE_SIZE,
        OFFSET + y as f32 * SQUARE_SIZE,
    );
    
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    ..default()
                },
                transform: Transform {
                    translation: pos.extend(0.),
                    scale: Vec3::new(SQUARE_SIZE, SQUARE_SIZE, 0.),
                    ..default()
                },
                ..default()
            },
            Square { x, y },
    ));
}
