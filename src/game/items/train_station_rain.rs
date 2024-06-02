use bevy::ecs::system::EntityCommands;
use bevy::math::vec3;
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
    let mut raindrop_gradient = Gradient::new();
    raindrop_gradient.add_key(0.0, Vec3::splat(1.).extend(0.0));
    raindrop_gradient.add_key(0.1, Vec3::splat(1.).extend(0.5));
    raindrop_gradient.add_key(0.9, Vec3::splat(1.).extend(0.5));
    raindrop_gradient.add_key(1.0, Vec3::splat(1.).extend(0.0));

    let writer = ExprWriter::new();

    let size = SetSizeModifier {
        size: CpuValue::Uniform((Vec2::new(1.0, 4.0), Vec2::new(1.0, 5.0))),
        ..default()
    };

    let init_pos =
        SetAttributeModifier::new(Attribute::POSITION,
                                  writer.rand(ValueType::Vector(VectorType::VEC3F))
                                      .mul(writer.lit(Vec3::new(2600., 1., 0.0)))
                                      .sub(writer.lit(Vec3::new(1300., 0., 0.0)))
                                      .add(writer.lit(Vec3::new(0.0, 0.0, 0.0)))
                                      .expr());

    let init_vel =
        SetAttributeModifier::new(Attribute::VELOCITY, writer.lit(Vec3::new(0., -500., 0.)).expr());

    let lifetime = writer.rand(ValueType::Scalar(ScalarType::Float)).mul(writer.lit(0.8)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let module = writer.finish();

    let effect = EffectAsset::new(32768, Spawner::rate(200_f32.into()), module)
        .with_name("TrainStationRain")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .render(ColorOverLifetimeModifier { gradient: raindrop_gradient })
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
                transform: Transform::from_xyz(0.0, 224.0, 100.0),
                ..Default::default()
            });
        });
    }
}
