use bevy::prelude::*;
use crate::states::GameState;
use crate::board::Winner;
use crate::piece::PieceColor;

const BUTTON_COLOR: Color = Color::WHITE;
const TEXT_COLOR: Color = Color::WHITE;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_camera)
            .add_system_set(
                SystemSet::on_enter(GameState::Menu)
                    .with_system(setup_menu)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Menu)
                    .with_system(click_play_button)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Menu)
                    .with_system(cleanup_menu)
            );
    }
}

pub struct MenuPlugin;

#[derive(Component)]
struct TextComponent;

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_menu(
    mut commands: Commands,
    materials: Res<AssetServer>,
    mut winner: ResMut<Winner>,
) {
    let text_style = TextStyle {
        font: materials.load("fonts/FiraSans-Bold.ttf"),
        font_size: 60.0,
        color: TEXT_COLOR,
    };

    if let Some(color) = winner.winner {
        let box_size = Vec2::new(300.0, 200.0);
        let box_position = Vec2::new(0.0, -250.0);

        commands.spawn_bundle(Text2dBundle {
            text: Text::from_section(match color {
                PieceColor::White => "White won",
                PieceColor::Black => "Black won"
            }, text_style),
            text_2d_bounds: bevy::text::Text2dBounds {
                // Wrap text in the rectangle
                size: box_size,
            },
            transform: Transform::from_xyz(
                box_position.x - box_size.x / 2.0,
                box_position.y + box_size.y / 2.0,
                1.0,
            ),
            ..default()
        })
        .insert(TextComponent);

        winner.winner = None;
    }

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: BUTTON_COLOR.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Play".to_string(),
                        style: TextStyle {
                            font: materials.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: BUTTON_COLOR,
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
}

fn click_play_button(
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::Playing).unwrap();
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.15, 0.15, 0.15).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

fn cleanup_menu(
    mut commands: Commands, 
    button: Query<Entity, With<Button>>,
    text: Query<Entity, With<TextComponent>>
) {
    commands.entity(button.single()).despawn_recursive();
    for entity in text.iter() {
        commands.entity(entity).despawn_recursive();
    }
}