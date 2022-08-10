use bevy::prelude::*;
use crate::states::GameState;

pub struct MenuPlugin;

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

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgb(0.9, 0.9, 0.9).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Play".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(1.0, 0.0, 0.0),
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

fn cleanup_menu(mut commands: Commands, button: Query<Entity, With<Button>>) {
    commands.entity(button.single()).despawn_recursive();
}