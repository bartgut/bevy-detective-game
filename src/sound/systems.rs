use bevy::prelude::*;
use rand::prelude::*;
use crate::sound::components::AudioPlayable;
use crate::sound::typewriting::components::TypewriterClickedSoundEffect;

pub fn typewriting_sound(commands: &mut Commands, sign: Option<char>) {
    match sign {
        Some(sign) => match sign {
            '\n' => {}
            _ => {
                let chosen = thread_rng().gen_range(0..4) + 1;
                commands.spawn(TypewriterClickedSoundEffect {
                    chosen_letter: chosen,
                });
            }
        },
        None => {}
    }
}

pub fn play_when_added<T: Component + AudioPlayable>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    playable: Query<&T, Added<T>>,
) {
    for playable in playable.iter() {
        playable.play(&mut commands, &asset_server);
    }
}
