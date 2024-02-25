use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use crate::clickable::items::resource::ItemResource;
use crate::game::items::train_station_rain::TrainStationRain;
use crate::level_state::LevelState;
use crate::movement::linear_movement::components::Linear2DMovementComponent;
use crate::movement::sin_movement::components::SinMovementComponent;
use crate::spawnable::components::SpawnableChild;

#[derive(Component)]
pub struct FlyingOrb {
    pub effect_description: Handle<EffectAsset>,
}

pub fn flying_orb_prepare(
    mut items: ResMut<ItemResource>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 1., 1., 1.));
    gradient.add_key(1.0, Vec4::splat(0.));

    // Create a new expression module
    let mut module = Module::default();

    let size = SetSizeModifier {
        size: CpuValue::Single(Vec2::new(2.0, 2.0)),
        ..default()
    };

    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(0.5),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.lit(10.),
    };

    let lifetime = module.lit(4.);
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let accel = module.lit(Vec3::new(0., 0., 0.));
    let update_accel = AccelModifier::new(accel);

    let effect = EffectAsset::new(32768, Spawner::rate(200_f32.into()), module)
        .with_name("FlyingOrb")
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
            items.push(Box::new(FlyingOrb {
                effect_description: effect_handle,
            }))
        });
}

impl SpawnableChild for FlyingOrb {
    fn spawn_child(&self, level: &mut EntityCommands, _: &Res<AssetServer>) {
        level.with_children(|parent| {
            parent.spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(self.effect_description.clone_weak()),
                    transform: Transform::from_xyz(0.0, 0.0, 100.0),
                    ..Default::default()
                },
                SinMovementComponent {
                    amplitude: 20.0,
                    to: Vec2::new(-1280.0, 0.0),
                    velocity: Vec2::new(150.0, 0.0),
                    frequency: 0.025,
                },
            ));
        });
    }
}
