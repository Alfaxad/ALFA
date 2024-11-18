// Updated `grid.rs`
use bevy::prelude::*;
use crate::config::{MAX_PHEROMONE_STRENGTH, PH_CACHE_GRID_SIZE, PH_DECAY_RATE, PH_GRID_VIZ_MIN_STRENGTH, PH_GRID_OPACITY, W, H};
use crate::pheromone::Pheromones;
use crate::utils::get_rand_unit_vec2;
use std::collections::HashMap;

#[derive(Default)]
pub struct Grid {
    pub signals: DecayGrid,
}

impl Grid {
    pub fn new() -> Self {
        let signals = HashMap::new();
        Grid {
            signals: DecayGrid::new(signals, MAX_PHEROMONE_STRENGTH),
        }
    }

    pub fn decay(&mut self) {
        self.signals.decay_values(PH_DECAY_RATE);
    }

    pub fn update_image(&self, img_bytes: &mut [u8]) {
        for (key, &strength) in &self.signals.values {
            let (tx, ty) = (key.0 / PH_CACHE_GRID_SIZE, key.1 / PH_CACHE_GRID_SIZE);
            let idx = (ty * W as usize + tx) * 4;
            if idx.saturating_add(3) >= img_bytes.len() || strength < PH_GRID_VIZ_MIN_STRENGTH {
                continue;
            }
            img_bytes[idx + 3] = cmp::min(img_bytes[idx + 3].saturating_add(strength), PH_GRID_OPACITY);
        }
    }
}

// Updated `pheromone.rs`
use bevy::prelude::*;
use crate::config::{
    PH_DECAY_INTERVAL, PH_KD_TREE_UPDATE_INTERVAL, PH_IMG_UPDATE_SEC, W, H, FOOD_LOCATION,
    HIVE_LOCATION, VIZ_COLOR_STRENGTH, VIZ_DECAY_RATE, VIZ_COLOR_TO_FOOD, VIZ_COLOR_TO_HOME,
    VIZ_MAX_COLOR_STRENGTH,
};
use crate::grid::WorldGrid;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct Pheromones {
    pub to_home: WorldGrid,
    pub to_food: WorldGrid,
}

pub struct PheromonePlugin;

impl Plugin for PheromonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pheromone_decay.run_if(on_timer(Duration::from_secs_f32(PH_DECAY_INTERVAL))))
            .add_systems(Update, update_kd_tree.run_if(on_timer(Duration::from_secs_f32(PH_KD_TREE_UPDATE_INTERVAL))))
            .add_systems(Update, pheromone_image_update.run_if(on_timer(Duration::from_secs_f32(PH_IMG_UPDATE_SEC))));
    }
}

fn pheromone_decay(mut pheromones: ResMut<Pheromones>) {
    pheromones.to_home.decay_values(VIZ_DECAY_RATE);
    pheromones.to_food.decay_values(VIZ_DECAY_RATE);
}

fn update_kd_tree(mut pheromones: ResMut<Pheromones>) {
    // Logic for updating kd-tree
}

fn pheromone_image_update(mut pheromones: ResMut<Pheromones>) {
    // Logic for updating pheromone image
}

// Updated `colony.rs`
use bevy::prelude::*;
use crate::bee::{Bee, CurrentRole};
use crate::pheromone::Pheromones;
use crate::config::{HIVE_LOCATION, GUARD_PATROL_RADIUS};

#[derive(Resource, Default)]
pub struct Colony;

pub struct ColonyPlugin;

impl Plugin for ColonyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_colony_state);
    }
}

fn update_colony_state(mut colony: ResMut<Colony>, query: Query<(&Transform, &Bee)>) {
    // Logic to update colony state
}

// Updated `utils.rs`
use bevy::math::vec2;
use rand::Rng;
use crate::config::{W, H};

pub fn get_rand_unit_vec2() -> Vec2 {
    let mut rng = rand::thread_rng();
    vec2(
        rng.gen_range(-(W as f32)..(W as f32)),
        rng.gen_range(-(H as f32)..(H as f32)),
    )
}

// Updated `pathviz.rs`
use bevy::prelude::*;
use crate::bee::{Bee, CurrentRole};
use crate::config::{PH_UNIT_GRID_SIZE, W, H, VIZ_COLOR_STRENGTH, VIZ_DECAY_RATE, VIZ_COLOR_TO_FOOD, VIZ_COLOR_TO_HOME, VIZ_MAX_COLOR_STRENGTH};
use std::collections::HashMap;

pub struct PathVizGrid {
    pub dg_food: DecayGrid,
    pub dg_home: DecayGrid,
}

impl PathVizGrid {
    pub fn new() -> Self {
        PathVizGrid {
            dg_food: DecayGrid::new(HashMap::new(), VIZ_MAX_COLOR_STRENGTH),
            dg_home: DecayGrid::new(HashMap::new(), VIZ_MAX_COLOR_STRENGTH),
        }
    }

    pub fn update_grid_values(&mut self, key: &Vec2) {
        self.dg_food.add_value(key, VIZ_COLOR_STRENGTH, 5.0);
        self.dg_home.add_value(key, VIZ_COLOR_STRENGTH, 5.0);
    }

    pub fn decay(&mut self) {
        self.dg_food.decay_values(VIZ_DECAY_RATE);
        self.dg_home.decay_values(VIZ_DECAY_RATE);
    }
}

// Updated `roles.rs`
use bevy::prelude::*;
use crate::pheromone::Pheromones;
use crate::colony::Colony;
use crate::config::{BEE_SCAN_RADIUS, HIVE_LOCATION, GUARD_PATROL_RADIUS};

pub struct CurrentRole;

impl CurrentRole {
    pub fn act(
        &self,
        velocity: &mut Vec2,
        acceleration: &mut Vec2,
        transform: &Transform,
        pheromones: &mut ResMut<Pheromones>,
        colony: &Res<Colony>,
    ) {
        // Role-specific logic here
    }
} 

// Ensure this struct is public
pub struct RoleSwitchThresholds;
