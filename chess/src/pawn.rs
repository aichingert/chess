use bevy::prelude::*;

#[derive(Component)]
pub struct Pawn;

pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_pawn);
    }
}

fn pawn_movement(pawn_query: Query<(&Pawn, &mut Transform)>, mouse: Res<Input<MouseButton>>) {

}

fn spawn_pawn(mut commands: Commands, ) {

}