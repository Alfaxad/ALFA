use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

use crate::components::*;
use crate::resources::*;

pub fn setup_system(
    mut commands: Commands,
    population: Res<Population>,
    mut interaction_rules: ResMut<InteractionRules>,
    mut species_radii: ResMut<SpeciesRadii>,
    current_index: Res<CurrentGenomeIndex>,
    sim_params: Res<SimulationParameters>,
) {
    commands.spawn(Camera2dBundle::default());

    // Simple static obstacle in the center
    let obstacle_shape = shapes::Circle {
        radius: 50.0,
        center: Vec2::ZERO,
    };
    let path = GeometryBuilder::build_as(&obstacle_shape);
    commands.spawn((
        ShapeBundle {
            path,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Fill::color(Color::DARK_GRAY),
        Obstacle { radius: 50.0 },
    ));

    let genome = &population.genomes[current_index.0];
    interaction_rules.species_count = population.species_count;
    interaction_rules.rules = genome.rules.clone();
    species_radii.radii = genome.radii.clone();
    species_radii.radii_sqr = species_radii
        .radii
        .iter()
        .map(|r| r * r)
        .collect();

    let width = 1280.0;
    let height = 720.0;
    let species_count = population.species_count;
    let particles_per_species = sim_params.particles_per_species;
    let mut rng = rand::thread_rng();

    for s in 0..species_count {
        for _ in 0..particles_per_species {
            let shape = shapes::Circle {
                radius: 3.0,
                center: Vec2::ZERO,
            };

            let path = GeometryBuilder::build_as(&shape);

            commands.spawn((
                ShapeBundle {
                    path,
                    transform: Transform::from_xyz(
                        rng.gen_range(-width / 2.0..width / 2.0),
                        rng.gen_range(-height / 2.0..height / 2.0),
                        0.0,
                    ),
                    ..default()
                },
                Fill::color(species_color(s)),
                Particle { species: s },
                Velocity(Vec2::ZERO),
            ));
        }
    }
}

fn species_color(s: usize) -> Color {
    match s {
        0 => Color::RED,
        1 => Color::YELLOW,
        2 => Color::CYAN,
        3 => Color::GREEN,
        _ => Color::WHITE,
    }
}

pub fn physics_system(
    mut query: Query<(&Particle, &mut Velocity, &mut Transform)>,
    params: Res<GlobalParameters>,
    interaction_rules: Res<InteractionRules>,
    species_radii: Res<SpeciesRadii>,
    obstacle_query: Query<(&Transform, &Obstacle)>,
) {
    let particles: Vec<(usize, Vec2, Vec2)> = query
        .iter_mut()
        .map(|(p, v, t)| (p.species, t.translation.truncate(), v.0))
        .collect();

    let mut new_velocities = Vec::with_capacity(particles.len());

    for (i, &(s1, pos1, vel1)) in particles.iter().enumerate() {
        let mut fx = 0.0;
        let mut fy = 0.0;
        let r2 = species_radii.radius_sqr(s1);

        for (j, &(s2, pos2, _)) in particles.iter().enumerate() {
            if i == j {
                continue;
            }
            let dx = pos1.x - pos2.x;
            let dy = pos1.y - pos2.y;
            let d2 = dx * dx + dy * dy;

            if d2 > 0.0 && d2 < r2 {
                let f = interaction_rules.get(s1, s2) / d2.sqrt();
                fx += f * dx;
                fy += f * dy;
            }
        }

        // obstacle repulsion
        for (ot, obs) in obstacle_query.iter() {
            let op = ot.translation.truncate();
            let dx = pos1.x - op.x;
            let dy = pos1.y - op.y;
            let d2 = dx * dx + dy * dy;
            let rr = obs.radius + 5.0;
            if d2 < rr * rr && d2 > 0.0 {
                let dist = d2.sqrt();
                let f = params.wall_repel / dist;
                fx += f * dx / dist;
                fy += f * dy / dist;
            }
        }

        fy -= params.gravity;
        let vmix = 1.0 - params.viscosity;
        let vx = vel1.x * vmix + fx * params.time_scale;
        let vy = vel1.y * vmix + fy * params.time_scale;
        new_velocities.push(Vec2::new(vx, vy));
    }

    for ((_, mut v, mut t), newv) in query.iter_mut().zip(new_velocities) {
        t.translation.x += newv.x;
        t.translation.y += newv.y;
        v.0 = newv;
    }
}

pub fn boundary_system(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Particle>>,
) {
    let window = window_query.single();
    let width = window.width();
    let height = window.height();
    let half_w = width / 2.0;
    let half_h = height / 2.0;

    for (mut t, mut v) in query.iter_mut() {
        let x = t.translation.x;
        let y = t.translation.y;
        let mut vx = v.0.x;
        let mut vy = v.0.y;

        if x < -half_w {
            t.translation.x = -half_w;
            vx = -vx;
        }
        if x > half_w {
            t.translation.x = half_w;
            vx = -vx;
        }
        if y < -half_h {
            t.translation.y = -half_h;
            vy = -vy;
        }
        if y > half_h {
            t.translation.y = half_h;
            vy = -vy;
        }

        v.0 = Vec2::new(vx, vy);
    }
}

pub fn adaptive_learning_system(
    mut state: ResMut<AdaptiveLearningState>,
    mut population: ResMut<Population>,
    mut interaction_rules: ResMut<InteractionRules>,
    mut current_index: ResMut<CurrentGenomeIndex>,
    query: Query<(Entity, &Transform), With<Particle>>,
    mut commands: Commands,
    mut species_radii: ResMut<SpeciesRadii>,
    sim_params: Res<SimulationParameters>,
    metric: Res<FitnessMetric>,
    mut logger: ResMut<Logger>,
) {
    state.frame_count += 1;
    if state.frame_count % state.evaluate_interval == 0 {
        let positions: Vec<Vec2> = query.iter().map(|(_, t)| t.translation.truncate()).collect();
        let score = match *metric {
            FitnessMetric::Cohesion => measure_cluster_cohesion(&positions),
            FitnessMetric::Dispersion => measure_dispersion(&positions),
            FitnessMetric::Coverage => measure_coverage(&positions),
        };
        let i = current_index.0;
        population.genomes[i].fitness = score;
        state.last_score = score;
        logger.log(population.generation, score);

        state.tested_count += 1;
        if state.tested_count == population.genomes.len() {
            population.select_and_reproduce();
            state.tested_count = 0;
        }

        let next_index = (i + 1) % population.genomes.len();
        current_index.0 = next_index;

        for (e, _) in query.iter() {
            commands.entity(e).despawn_recursive();
        }

        let genome = &population.genomes[next_index];
        interaction_rules.species_count = population.species_count;
        interaction_rules.rules = genome.rules.clone();
        species_radii.radii = genome.radii.clone();
        species_radii.radii_sqr = species_radii.radii.iter().map(|r| r * r).collect();

        let width = 1280.0;
        let height = 720.0;
        let species_count = population.species_count;
        let particles_per_species = sim_params.particles_per_species;
        let mut rng = rand::thread_rng();

        for s in 0..species_count {
            for _ in 0..particles_per_species {
                let shape = shapes::Circle {
                    radius: 3.0,
                    center: Vec2::ZERO,
                };

                let path = GeometryBuilder::build_as(&shape);

                commands.spawn((
                    ShapeBundle {
                        path,
                        transform: Transform::from_xyz(
                            rng.gen_range(-width / 2.0..width / 2.0),
                            rng.gen_range(-height / 2.0..height / 2.0),
                            0.0,
                        ),
                        ..default()
                    },
                    Fill::color(species_color(s)),
                    Particle { species: s },
                    Velocity(Vec2::ZERO),
                ));
            }
        }
    }
}

fn measure_cluster_cohesion(positions: &[Vec2]) -> f32 {
    if positions.is_empty() {
        return 0.0;
    }
    let centroid: Vec2 = positions.iter().copied().reduce(|a, b| a + b).unwrap() / (positions.len() as f32);
    let mut sum_dist = 0.0;
    for &p in positions {
        sum_dist += p.distance_squared(centroid);
    }
    let avg_dist = sum_dist / (positions.len() as f32);
    1.0 / (1.0 + avg_dist)
}

fn measure_dispersion(positions: &[Vec2]) -> f32 {
    if positions.is_empty() {
        return 0.0;
    }
    let mut max_dist = 0.0;
    for &p in positions {
        for &q in positions {
            let d = p.distance_squared(q);
            if d > max_dist {
                max_dist = d;
            }
        }
    }
    1.0 / (1.0 + max_dist)
}

fn measure_coverage(positions: &[Vec2]) -> f32 {
    if positions.is_empty() {
        return 0.0;
    }
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    for p in positions {
        min_x = min_x.min(p.x);
        max_x = max_x.max(p.x);
        min_y = min_y.min(p.y);
        max_y = max_y.max(p.y);
    }
    let area = (max_x - min_x) * (max_y - min_y);
    1.0 / (1.0 + area)
}

pub fn ui_system(
    mut egui_context: EguiContexts,
    mut params: ResMut<GlobalParameters>,
    mut adaptive: ResMut<AdaptiveLearningState>,
    mut species_radii: ResMut<SpeciesRadii>,
    population: Res<Population>,
    current_index: Res<CurrentGenomeIndex>,
    mut sim_params: ResMut<SimulationParameters>,
    mut metric: ResMut<FitnessMetric>,
) {
    egui::Window::new("Simulation Controls").show(egui_context.ctx_mut(), |ui| {
        ui.heading("Global Parameters");
        ui.add(egui::Slider::new(&mut params.viscosity, 0.1..=2.0).text("Viscosity"));
        ui.add(egui::Slider::new(&mut params.gravity, 0.0..=1.0).text("Gravity"));
        ui.add(egui::Slider::new(&mut params.wall_repel, 0.0..=100.0).text("Wall Repel"));
        ui.add(egui::Slider::new(&mut params.time_scale, 0.1..=5.0).text("Time Scale"));

        ui.separator();
        ui.heading("Adaptive Learning");
        ui.label(format!("Generation: {}", population.generation));
        ui.label(format!("Current Genome: {}", current_index.0));
        ui.add(egui::Slider::new(&mut adaptive.mutation_rate, 0.0..=0.1).text("Mutation Rate"));
        ui.label(format!("Evaluate Interval: {}", adaptive.evaluate_interval));
        ui.label(format!("Last Score: {:.4}", adaptive.last_score));
        ui.add(egui::Slider::new(&mut sim_params.particles_per_species, 50..=500).text("Particles/Species"));
        ui.horizontal(|ui| {
            ui.label("Fitness Metric:");
            ui.selectable_value(&mut *metric, FitnessMetric::Cohesion, "Cohesion");
            ui.selectable_value(&mut *metric, FitnessMetric::Dispersion, "Dispersion");
            ui.selectable_value(&mut *metric, FitnessMetric::Coverage, "Coverage");
        });

        ui.separator();
        ui.heading("Species Radii");
        for (i, r) in species_radii.radii.iter_mut().enumerate() {
            ui.add(egui::Slider::new(r, 10.0..=200.0).text(format!("Radius for species {}", i)));
        }
        species_radii.radii_sqr = species_radii.radii.iter().map(|r| r * r).collect();
    });
}
