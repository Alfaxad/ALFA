// roles.rs - Define detailed behaviors for each bee role

use bevy::prelude::*;
use rand::Rng;

// Enum representing different roles in the bee colony
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeeRole {
    Forager,
    Guard,
    Scout,
    Nurse,
    Queen,
}

impl BeeRole {
    // Generate a random role for initialization
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=4) {
            0 => BeeRole::Forager,
            1 => BeeRole::Guard,
            2 => BeeRole::Scout,
            3 => BeeRole::Nurse,
            _ => BeeRole::Queen,
        }
    }
}

// Behavior details for each role
impl BeeRole {
    pub fn act(&self, velocity: &mut Vec2, acceleration: &mut Vec2, transform: &Transform, pheromones: &mut ResMut<Pheromones>, colony: &Res<Colony>) {
        match self {
            BeeRole::Forager => Self::forage_behavior(velocity, acceleration, transform, pheromones),
            BeeRole::Guard => Self::guard_behavior(velocity, acceleration, transform, colony),
            BeeRole::Scout => Self::scout_behavior(velocity, acceleration, transform, pheromones),
            BeeRole::Nurse => Self::nurse_behavior(velocity, acceleration),
            BeeRole::Queen => Self::queen_behavior(velocity, acceleration),
        }
    }

    // Forager bee behavior - Find and collect food, return to hive
    fn forage_behavior(velocity: &mut Vec2, acceleration: &mut Vec2, transform: &Transform, pheromones: &mut ResMut<Pheromones>) {
        if let Some(target) = pheromones.to_food.get_steer_target(&transform.translation, BEE_SCAN_RADIUS) {
            let steering_force = get_steering_force(target, transform.translation.truncate(), *velocity);
            *acceleration += steering_force;
        } else {
            // Default random movement
            *acceleration += get_rand_unit_vec2() * 0.2;
        }
    }

    // Guard bee behavior - Patrol the hive and defend against threats
    fn guard_behavior(velocity: &mut Vec2, acceleration: &mut Vec2, transform: &Transform, colony: &Res<Colony>) {
        let dist_to_hive = transform.translation.distance(Vec3::new(HIVE_LOCATION.0, HIVE_LOCATION.1, 0.0));
        if dist_to_hive > GUARD_PATROL_RADIUS {
            let target = Vec2::new(HIVE_LOCATION.0, HIVE_LOCATION.1);
            let steering_force = get_steering_force(target, transform.translation.truncate(), *velocity);
            *acceleration += steering_force;
        } else {
            // Patrol around the hive
            *acceleration += get_rand_unit_vec2() * 0.1;
        }
    }

    // Scout bee behavior - Explore the environment and discover resources
    fn scout_behavior(velocity: &mut Vec2, acceleration: &mut Vec2, transform: &Transform, pheromones: &mut ResMut<Pheromones>) {
        if let Some(target) = pheromones.to_food.get_steer_target(&transform.translation, BEE_SCAN_RADIUS) {
            let steering_force = get_steering_force(target, transform.translation.truncate(), *velocity);
            *acceleration += steering_force;
        } else {
            // Default exploration movement
            *acceleration += get_rand_unit_vec2() * 0.3;
        }
    }

    // Nurse bee behavior - Stay in the hive, care for brood
    fn nurse_behavior(_velocity: &mut Vec2, _acceleration: &mut Vec2) {
        // Nurse bees stay in the hive and don't need movement outside the hive
        // Placeholder for any logic for taking care of brood, feeding, etc.
    }

    // Queen bee behavior - Stay in the hive and manage brood production
    fn queen_behavior(_velocity: &mut Vec2, _acceleration: &mut Vec2) {
        // Queen remains in the hive
        // Placeholder for any logic for managing egg-laying and hive activities
    }
}

// Utility function for getting a steering force based on target and current position
fn get_steering_force(target: Vec2, current: Vec2, velocity: Vec2) -> Vec2 {
    let desired = target - current;
    let steering = desired - velocity;
    steering * 0.05
}

// Utility function for getting a random unit vector in 2D
fn get_rand_unit_vec2() -> Vec2 {
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..(std::f32::consts::PI * 2.0));
    Vec2::new(angle.cos(), angle.sin())
}
