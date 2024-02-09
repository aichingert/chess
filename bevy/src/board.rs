use bevy::prelude::*;

use crate::consts::{BROWN_COLOR, LIGHT_BROWN_COLOR, SQUARE_SIZE};

#[derive(Component)]
struct Square {
    x: u8,
    y: u8,
    
}

pub fn create_board(mut commands: Commands) {
    for row in 0..8 {
        for col in 0..8 {
            // 0 => | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 
            // 1 => | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 

            // 
            
            let color = if (row + col) & 1 == 0 {
                BROWN_COLOR              
            } else {
                LIGHT_BROWN_COLOR              
            };
 
            get_square(&mut commands, color, row, col);
        }
    }
}

fn get_square(commands: &mut Commands, color: Color, y: u8, x: u8) -> Entity {  
    let pos = Vec2::new(
        (x * SQUARE_SIZE) as f32,
        (y * SQUARE_SIZE) as f32,
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
                    scale: Vec3::new(SQUARE_SIZE as f32, SQUARE_SIZE as f32, 0.),
                    ..default()
                },
                ..default()
            },
            Square { x, y },
    )).id()
}
