use bevy::prelude::*;

#[derive(Component)]
pub struct Animated {
    pub handle: Handle<AnimationClip>,
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, play_animation);
    }
}

fn play_animation(mut players: Query<&mut AnimationPlayer>, animations: Query<&Animated>) {
    for mut player in players.iter_mut() {
        for animation in animations.iter() {
            if !player.is_playing_clip(&animation.handle) {
                println!("Playing animation {:?}", animation.handle);
                player.play(animation.handle.clone());
            }
        }
    }
}
