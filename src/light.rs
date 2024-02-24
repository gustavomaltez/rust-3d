use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct SunMoonLight;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_light)
            .add_systems(Update, rotate_light);
    }
}

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::rgb(1.0, 1.0, 1.0),
                illuminance: 100000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 1.0, 0.0),
                rotation: Quat::from_rotation_x(310.0_f32.to_radians()),
                ..default()
            },
            ..default()
        },
        SunMoonLight,
    ));
}

fn rotate_light(mut query: Query<(&SunMoonLight, &mut Transform,)>) {
    for (_, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(-1.0_f32.to_radians()));

        // println!(
        //     "Light rotation: {}",
        //     transform.rotation.to_axis_angle().1.to_degrees()
        // );
    }
}
