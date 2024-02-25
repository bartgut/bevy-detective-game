use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_hanabi::EffectAsset;
use crate::clickable::items::resource::ItemResource;
use crate::level_state::LevelState;
use crate::spawnable::components::SpawnableChild;

#[derive(Component)]
pub struct TrainStationRain {
    pub effect_description: Handle<EffectAsset>,
}

pub fn train_station_rain_prepare(
    mut items: ResMut<ItemResource>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 1., 1., 1.));
    gradient.add_key(1.0, Vec4::splat(0.));

    // Create a new expression module
    let mut module = Module::default();

    let size = SetSizeModifier {
        size: CpuValue::Single(Vec2::new(1.0, 5.0)),
        ..default()
    };

    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(0.5),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.lit(500.),
    };

    let lifetime = module.lit(4.);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let accel = module.lit(Vec3::new(0., -500., 0.));
    let update_accel = AccelModifier::new(accel);

    let effect = EffectAsset::new(32768, Spawner::rate(200_f32.into()), module)
        .with_name("TrainStationRain")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .update(update_accel)
        .render(ColorOverLifetimeModifier { gradient })
        .render(size);

    let effect_handle = effects.add(effect);
    items
        .items
        .entry(LevelState::TrainPlatform)
        .and_modify(|items| {
            items.push(Box::new(TrainStationRain {
                effect_description: effect_handle,
            }))
        });
}

impl SpawnableChild for TrainStationRain {
    fn spawn_child(&self, level: &mut EntityCommands, _: &Res<AssetServer>) {
        level.with_children(|parent| {
            parent.spawn(ParticleEffectBundle {
                effect: ParticleEffect::new(self.effect_description.clone_weak()),
                transform: Transform::from_xyz(0.0, 460.0, 100.0),
                ..Default::default()
            });
        });
    }
}
