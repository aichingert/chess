use {bevy::prelude::*, bevy::input::mouse::MouseMotion, bevy::input::mouse::MouseWheel};

#[derive()]
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(print_mouse_events_system);
    }
}

fn print_mouse_events_system(
    mut mouse_button_input_events: EventReader<MouseButton>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    for event in mouse_button_input_events.iter() {
        info!("{:?}", event);
    }

    for event in mouse_motion_events.iter() {
        info!("{:?}", event);
    }

    for event in cursor_moved_events.iter() {
        info!("{:?}", event);
    }

    for event in mouse_wheel_events.iter() {
        info!("{:?}", event);
    }
}
