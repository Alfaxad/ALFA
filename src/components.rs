use bevy::prelude::*;

#[derive(Component)]
pub struct Particle {
    pub species: usize,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);
