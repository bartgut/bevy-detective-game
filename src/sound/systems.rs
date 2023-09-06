use bevy::prelude::*;
use rand::prelude::*;
use crate::sound::typewriting::components::AudioPlayable;

pub fn typewriting_sound(asset_server: &Res<AssetServer>, audio: &Res<Audio>, sign: Option<char>) {
    match sign {
        Some(sign) => match sign {
            '\n' => {}
            _ => {
                let chosen = thread_rng().gen_range(0..4) + 1;
                audio.play(
                    asset_server.load(format!("sound/typewriting/typewriting{}.ogg", chosen)),
                );
            }
        },
        None => {}
    }
}

pub fn play_when_added<T: Component + AudioPlayable>(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    playable: Query<&T, Added<T>>,
) {
    for playable in playable.iter() {
        playable.play(&asset_server, &audio);
    }
}
