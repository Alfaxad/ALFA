use crate::{
    pheromone::Pheromones,
    roles::CurrentRole,
    utils::{calc_rotation_angle, get_rand_unit_vec2},
    colony::Colony,
    *,
};
use bevy::{
    math::{vec2, vec3},
    prelude::*,
    time::common_conditions::on_timer,
};
use rand::{thread_rng, Rng};
use std::{f32::consts::PI, time::Duration};

pub struct BeePlugin;

#[derive(Component)]
pub struct Bee;
#[derive(Component)]
pub struct Velocity(pub Vec2);
#[derive(Component)]
pub struct Acceleration(pub Vec2);
#[derive(Component)]
pub struct PhStrength(pub f32);

#[derive(Resource)]
pub struct BeeFollowCameraPos(pub Vec2);

// Constants
pub const NUM_BEES: u32 = 100;
pub const HIVE_LOCATION: (f32, f32) = (0.0, 0.0);
pub const BEE_SPRITE_SCALE: f32 = 1.0;
pub const BEE_INITIAL_PH_STRENGTH: f32 = 10.0;
pub const BEE_SCAN_RADIUS: f32 = 50.0;
pub const GUARD_PATROL_RADIUS: f32 = 75.0;
pub const BEE_SPEED: f32 = 1.5;

impl Plugin for BeePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .insert_resource(BeeFollowCameraPos(Vec2::ZERO))
            .add_systems(
                Update,
                update_bee_behavior.run_if(on_timer(Duration::from_secs_f32(1.0))),
            )
            .add_systems(Update, update_position);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..NUM_BEES {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("bee.png"),
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    ..default()
                },
                transform: Transform::from_xyz(HIVE_LOCATION.0, HIVE_LOCATION.1, 1.0)
                    .with_scale(Vec3::splat(BEE_SPRITE_SCALE)),
                ..Default::default()
            },
            Bee,
            Velocity(get_rand_unit_vec2()),
            Acceleration(Vec2::ZERO),
            PhStrength(BEE_INITIAL_PH_STRENGTH),
            CurrentRole::Forager,
        ));
    }
}

fn update_bee_behavior(
    mut bee_query: Query<(&mut Velocity, &mut Acceleration, &Transform, &CurrentRole), With<Bee>>,
    mut pheromones: ResMut<Pheromones>,
    colony: Res<Colony>,
) {
    for (mut velocity, mut acceleration, transform, role) in bee_query.iter_mut() {
        match role {
            CurrentRole::Forager => {
                if let Some(target) = pheromones.to_food.get_steer_target(&transform.translation, BEE_SCAN_RADIUS) {
                    let steering_force = get_steering_force(target, transform.translation.truncate(), velocity.0);
                    acceleration.0 += steering_force;
                }
            }
            CurrentRole::Guard => {
                let dist_to_hive = transform.translation.distance(Vec3::new(HIVE_LOCATION.0, HIVE_LOCATION.1, 0.0));
                if dist_to_hive > GUARD_PATROL_RADIUS {
                    let target = Vec2::new(HIVE_LOCATION.0, HIVE_LOCATION.1);
                    let steering_force = get_steering_force(target, transform.translation.truncate(), velocity.0);
                    acceleration.0 += steering_force;
                }
            }
            _ => {}
        }
    }
}

fn update_position(
    mut bee_query: Query<(&mut Transform, &mut Velocity, &mut Acceleration), With<Bee>>,
) {
    for (mut transform, mut velocity, mut acceleration) in bee_query.iter_mut() {
        let old_pos = transform.translation;

        if !acceleration.0.is_nan() {
            velocity.0 = (velocity.0 + acceleration.0).normalize();
            let new_translation =
                transform.translation + vec3(velocity.0.x, velocity.0.y, 0.0) * BEE_SPEED;
            if !new_translation.is_nan() {
                transform.translation = new_translation;
            }
        }

        acceleration.0 = Vec2::ZERO;
        transform.rotation =
            Quat::from_rotation_z(calc_rotation_angle(old_pos, transform.translation) + PI / 2.0);
    }
}

fn get_steering_force(target: Vec2, current: Vec2, velocity: Vec2) -> Vec2 {
    let desired = target - current;
    let steering = desired - velocity;
    steering * 0.05
}
