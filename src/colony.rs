use crate::{
    bee::{Bee, CurrentRole},
    roles::RoleSwitchThresholds,
    utils::get_rand_unit_vec2,
    *,
};
use crate::roles::{CurrentRole, RoleSwitchThresholds};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Colony {
    pub role_switch_thresholds: RoleSwitchThresholds,
    pub num_foragers: u32,
    pub num_guards: u32,
    pub num_nurses: u32,
    pub num_scouts: u32,
    pub num_maintenance: u32,
}

pub const MAX_FOOD_STORAGE: f32 = 500.0;
pub const MIN_FOOD_THRESHOLD: f32 = 50.0;
pub const GUARD_SWITCH_RADIUS: f32 = 50.0;

impl Plugin for ColonyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Colony::default())
            .add_systems(Update, update_colony_state);
    }
}

fn update_colony_state(
    mut colony: ResMut<Colony>,
    bee_query: Query<(&CurrentRole, &Transform), With<Bee>>,
) {
    let mut num_foragers = 0;
    let mut num_guards = 0;
    let mut num_nurses = 0;
    let mut num_scouts = 0;
    let mut num_maintenance = 0;

    for (role, _transform) in bee_query.iter() {
        match role {
            CurrentRole::Forager => num_foragers += 1,
            CurrentRole::Guard => num_guards += 1,
            CurrentRole::Nurse => num_nurses += 1,
            CurrentRole::Scout => num_scouts += 1,
            CurrentRole::Maintenance => num_maintenance += 1,
        }
    }

    colony.num_foragers = num_foragers;
    colony.num_guards = num_guards;
    colony.num_nurses = num_nurses;
    colony.num_scouts = num_scouts;
    colony.num_maintenance = num_maintenance;
} 
