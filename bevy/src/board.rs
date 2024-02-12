use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::consts::{COLORS, SQUARE_SIZE, OFFSET};
use crate::piece::{TakePieceEvent, MovePieceEvent, Piece};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Selected(None))
            .insert_resource(Board { entities: [None; 8 * 8] })
            .add_systems(Startup, create_board)
            .add_systems(Update, user_input);
    }
}

#[derive(Resource)]
struct Selected(Option<Entity>);

#[derive(Resource)]
pub struct Board {
    entities: [Option<Entity>; 8 * 8],
}

impl Board {
    pub fn add_entity(&mut self, r: usize, c: usize, piece: Entity) {
        self.entities[8 * r + c] = Some(piece);
    }

    pub fn get_entity(&self, r: usize, c: usize) -> Option<Entity> {
        self.entities[8 * r + c]
    }

    pub fn move_entity(&mut self, src: (usize, usize), dst: (usize, usize)) {
        self.entities[8 * dst.1 + dst.0] = self.entities[8 * src.1 + src.0];
        self.entities[8 * src.1 + src.0] = None;
    }
}

fn user_input(
    mut board: ResMut<Board>,
    mut selected: ResMut<Selected>, 
    mouse: Res<Input<MouseButton>>,
    pieces: Query<(Entity, &Piece)>,
    windows: Query<&Window, With<PrimaryWindow>>, 
    mut take_piece_wr: EventWriter<TakePieceEvent>,
    mut move_piece_wr: EventWriter<MovePieceEvent>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(Vec2 { x, y }) = windows.single().cursor_position() else { return; };
    let (r, c) = ((7 - (y / SQUARE_SIZE) as usize), ((x / SQUARE_SIZE) as usize));

    if selected.0.is_none() {
        selected.0 = board.get_entity(r, c);
        return;
    }

    let Ok((entity, piece)) = pieces.get(selected.0.unwrap()) else {
        return;
    };

    if let Some(Ok((dst_entity, dst_piece))) = board.get_entity(r, c).map(|e: Entity| pieces.get(e)) {
        if piece.team == dst_piece.team {
            selected.0 = Some(dst_entity);
            return;
        }

        take_piece_wr.send(TakePieceEvent(dst_entity));
    }

    selected.0 = None;
    board.move_entity((piece.loc.0 as usize, piece.loc.1 as usize), (c, r));
    move_piece_wr.send(MovePieceEvent((entity, (c as u8, r as u8))));
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
            ));
        }
    }
}
