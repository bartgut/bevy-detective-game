use bevy::prelude::*;
use rand::prelude::*;

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
