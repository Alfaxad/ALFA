use bevy::prelude::*;

#[derive(Component)]
pub struct Particle {
    pub species: usize,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Obstacle {
    pub radius: f32,
}
