use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct InputData {
    pub pressed_keys: Vec<KeyCode>,
    pub mouse_position: MousePosition,
}

#[derive(Default)]
pub struct MousePosition {
    // Mouse position on the screen
    pub source: Vec2,
    // Mouse position in the world floor (Y = 1.0)
    pub coordinates: Vec3,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputData>()
            .add_systems(Update, update_state);
    }
}

fn update_state(
    mut input_data: ResMut<InputData>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    // Push the keys that were just pressed
    for key in keyboard_input.get_just_pressed() {
        input_data.pressed_keys.push(*key);
    }

    // Remove the keys that were just released
    for key in keyboard_input.get_just_released() {
        input_data.pressed_keys.retain(|&k| k != *key);
    }

    // Get global mouse position and store it
    for event in cursor_moved_events.read() {
        input_data.mouse_position.source = event.position;
    }

    for (camera, global_transform) in camera_query.iter() {
        // Get the 3D ray from the camera to the mouse position
        let ray = camera
            .viewport_to_world(
                global_transform,
                input_data.mouse_position.source,
            )
            .expect(
                "InputPlugin :: update_state :: Unable to get ray from camera 
                to mouse position: For some reason the ray from the camera to 
                the mouse position is not available.",
            );

        // Get the distance to the plane (ground)
        let distance = ray
            .intersect_plane(
                Vec3::new(0.0, 1.0, 0.0),
                Plane3d::new(Vec3::new(0.0, 1.0, 0.0)),
            )
            .expect(
                "InputPlugin :: update_state :: Unable to get distance between 
                the camera and the ground: The ray does not intersect the ground.",
            );

        input_data.mouse_position.coordinates = ray.get_point(distance);
    }
}
