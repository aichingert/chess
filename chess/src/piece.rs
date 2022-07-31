use bevy::prelude::*;

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_pieces)
            .add_system(detection_system);
    }
}

fn spawn_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let pieces: [Kind; 8] = [Kind::Rook, Kind::Knight, Kind::Bishop, Kind::Queen, Kind::King, Kind::Bishop, Kind::Knight, Kind::Rook];
    let path: [&str; 8] = ["rook.png", "knight.png", "bishop.png", "queen.png", "king.png", "bishop.png", "knight.png", "rook.png"];

    //
    // Spawning pawns
    //

    for i in 0..8 {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("white/pawn.png"),
                transform: Transform {
                    translation: Vec3::new(super::OFFSET + i as f32 * super::SQUARE_SIZE, super::OFFSET + super::SQUARE_SIZE, 1.0),
                    scale: Vec3::new(0.4, 0.4, 1.0),
                    ..default()
                },
                ..default()
            })
            .insert(Piece::white(Kind::Pawn, (i, 1)));

        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("black/pawn.png"),
                transform: Transform {
                    translation: Vec3::new(super::OFFSET + i as f32 * super::SQUARE_SIZE, super::OFFSET + super::SQUARE_SIZE * 6.0, 0.0),
                    scale: Vec3::new(0.4, 0.4, 1.0),
                    ..default()
                },
                ..default()
            })
            .insert(Piece::black(Kind::Pawn, (i, 6)));
    }
    
    //
    // Spawning pieces
    //

    for i in 0..8 {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load(&format!("white/{}", path[i])),
                transform: Transform {
                    translation: Vec3::new(super::OFFSET + i as f32 * super::SQUARE_SIZE, super::OFFSET, 1.0),
                    scale: Vec3::new(0.4, 0.3, 1.0),
                    ..default()
                },
                ..default()
            })
            .insert(Piece::white(pieces[i], (i as i32, 0)));

        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load(&format!("black/{}", path[i])),
                transform: Transform {
                    translation: Vec3::new(super::OFFSET + i as f32 * super::SQUARE_SIZE, super::OFFSET + 7.0 * super::SQUARE_SIZE, 1.0),
                    scale: Vec3::new(0.4, 0.3, 1.0),
                    ..default()
                },
                ..default()
            })
            .insert(Piece::black(pieces[i], (i as i32, 7)));
    }
}

pub enum GameState {
    TakePiece,
} 

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Queen,
    King,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceColor {
    Black,
    White,
}

// Piece struct which stores the en_passant state for every piece but is only used for pawns
#[derive(Component, Debug, Clone)]
pub struct Piece {
    pub kind: Kind,
    pub color: PieceColor,
    pub position: (i32, i32),
    pub moves: Vec<(i32, i32)>,
}

// Point struct used for piece highlighting so you can iterate over the points and remove them after moving
#[derive(Component)]
pub struct Point;

// Turn struct has the current color that has the move
pub struct Turn {
    color_to_move: PieceColor,
}

// has the change function after move the turn changes to the enemy color
// white => black
// black => white
impl Turn {
    fn change(&mut self) {
        match self.color_to_move {
            PieceColor::White => {
                self.color_to_move = PieceColor::Black;
            },
            PieceColor::Black => {
                self.color_to_move = PieceColor::White;
            },
        }
    }
}

// Implements the constructors for the a white piece and the black piece
// also the function promotion => which promotes a pawn to any piece you want, except a pawn again and a king
impl Piece {
    fn white(kind: Kind, position: (i32, i32)) -> Piece {
        Piece {
            kind,
            color: PieceColor::White,
            position,
            moves: Vec::new(),
        }
    }
    
    fn black(kind: Kind, position: (i32, i32)) -> Piece {
        Piece {
            kind,
            color: PieceColor::Black,
            position,
            moves: Vec::new(),
        }
    }
    
    pub fn promotion(&mut self, to: Kind) {
        if self.kind == Kind::Pawn {
            // Issue: Add => piece despawn and respawning new piece
    
            self.kind = to;
        }
    }
}

// Is detecting if the player pressed on a piece and calculates the possible moves for it.
// After that it is going to highlight the possible moves as well
fn detection_system(
    mouse_button_input: ResMut<Input<MouseButton>>,
    mut piece_query: Query<(&mut Transform, &mut Piece)>,
    windows: Res<Windows>,
    mut commands: Commands
) {
    let window = windows.get_primary().unwrap();
    if let Some(pos) = window.cursor_position() {
        let x: i32 = (pos.x / super::SQUARE_SIZE) as i32;
        let y: i32 = (pos.y / super::SQUARE_SIZE) as i32;

        let mut transform_x = super::OFFSET;
        let mut transform_y = super::OFFSET;

        let mut pieces_on_the_board: Vec<Piece> = Vec::new();

        // Here is the calculation and it moves it automaticaly to the first possible move
        if mouse_button_input.just_released(MouseButton::Left) {
            piece_query.for_each( | query_info | {
                pieces_on_the_board.push(query_info.1.clone());
            });

            piece_query.for_each_mut( | (mut transform_piece,mut piece) | {
                if piece.position.0 == x && piece.position.1 == y {
                    piece.calculate_pseudo_legal_moves(pieces_on_the_board.clone());

                    let postitions = piece.moves.clone();
                    
                    if postitions.len() > 0 {
                        piece.position.0 = postitions[0].0;
                        piece.position.1 = postitions[0].1;
                        
                        transform_x += postitions[0].0 as f32 * super::SQUARE_SIZE;
                        transform_y += postitions[0].1 as f32 * super::SQUARE_SIZE;

                        piece.position.0 = postitions[0].0;
                        piece.position.1 = postitions[0].1;
    
                        transform_piece.translation = Vec3::new(transform_x, transform_y, 1.0);
                        
                        info!("{:?}", postitions);
                    }


                }
            });
        }

        // Here is the piece highlighting with right click (for now) => puts a red dot on the squares it can move
        if mouse_button_input.just_pressed(MouseButton::Right) {
            piece_query.for_each( | query_info | {
                pieces_on_the_board.push(query_info.1.clone());
                
            });

            piece_query.for_each_mut( | mut transform_and_piece | {
                if transform_and_piece.1.position.0 == x && transform_and_piece.1.position.1 == y {
                    transform_and_piece.1.calculate_pseudo_legal_moves(pieces_on_the_board.clone());
                    let positions = transform_and_piece.1.moves.clone();

                    for position in positions {
                        let point_position = Vec2::new(super::SQUARE_SIZE * position.0 as f32 + super::OFFSET, super::SQUARE_SIZE * position.1 as f32 + super::OFFSET);

                        info!("{:?}", position);

                        commands
                            .spawn()
                            .insert(Point)
                            .insert_bundle( SpriteBundle {
                                sprite: Sprite {
                                    color: Color::rgb(255.0, 0.0, 0.0),
                                    ..default()
                                },
                                transform: Transform {
                                    translation: point_position.extend(0.0),
                                    scale: Vec3::new(20.0, 20.0, 1.0),
                                    ..default()
                                },
                                ..default()
                        });
                    }
                }
            });
        }
    }
}

fn despawn_taken_pieces(
    mut commands: Commands,
    query: Query<(Entity, &Piece)>,
) {
    query.for_each(|(entity, piece)| {

        commands.entity(entity).despawn_recursive();
    })
}