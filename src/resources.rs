use bevy::prelude::*;
use rand::Rng;

#[derive(Resource)]
pub struct GlobalParameters {
    pub viscosity: f32,
    pub gravity: f32,
    pub wall_repel: f32,
    pub time_scale: f32,
}

impl Default for GlobalParameters {
    fn default() -> Self {
        Self {
            viscosity: 0.7,
            gravity: 0.0,
            wall_repel: 40.0,
            time_scale: 1.0,
        }
    }
}

#[derive(Resource, Default, Clone)]
pub struct InteractionRules {
    pub rules: Vec<f32>,
    pub species_count: usize,
}

impl InteractionRules {
    pub fn get(&self, s1: usize, s2: usize) -> f32 {
        self.rules[s1 * self.species_count + s2]
    }
}

#[derive(Clone)]
pub struct Genome {
    pub rules: Vec<f32>,
    pub radii: Vec<f32>,
    pub fitness: f32,
}

#[derive(Resource)]
pub struct Population {
    pub genomes: Vec<Genome>,
    pub species_count: usize,
    pub generation: usize,
}

impl Population {
    pub fn new(species_count: usize, size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut genomes = Vec::new();
        for _ in 0..size {
            let mut rules = vec![0.0; species_count * species_count];
            for r in rules.iter_mut() {
                *r = rng.gen_range(-1.0..1.0);
            }
            let radii: Vec<f32> = (0..species_count)
                .map(|_| rng.gen_range(20.0..100.0))
                .collect();
            genomes.push(Genome {
                rules,
                radii,
                fitness: 0.0,
            });
        }

        Self {
            genomes,
            species_count,
            generation: 0,
        }
    }

    pub fn select_and_reproduce(&mut self) {
        self.genomes.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        let elite1 = self.genomes[0].clone();
        let elite2 = self.genomes[1].clone();
        let size = self.genomes.len();
        let mut rng = rand::thread_rng();

        for i in 2..size {
            let mut child = elite1.clone();
            // Crossover rules
            for j in 0..child.rules.len() {
                if rng.gen_bool(0.5) {
                    child.rules[j] = elite2.rules[j];
                }
            }
            // Crossover radii
            for j in 0..child.radii.len() {
                if rng.gen_bool(0.5) {
                    child.radii[j] = elite2.radii[j];
                }
            }
            // Mutate rules
            for v in child.rules.iter_mut() {
                if rng.gen_bool(0.05) {
                    *v += rng.gen_range(-0.1..0.1);
                    *v = v.clamp(-1.0, 1.0);
                }
            }
            // Mutate radii
            for r in child.radii.iter_mut() {
                if rng.gen_bool(0.05) {
                    *r += rng.gen_range(-5.0..5.0);
                    *r = r.clamp(10.0, 200.0);
                }
            }
            child.fitness = 0.0;
            self.genomes[i] = child;
        }

        self.generation += 1;
    }
}

#[derive(Resource)]
pub struct SpeciesRadii {
    pub radii: Vec<f32>,
    pub radii_sqr: Vec<f32>,
}

impl SpeciesRadii {
    pub fn new(species_count: usize) -> Self {
        let radii = vec![80.0; species_count];
        let radii_sqr = radii.iter().map(|r| r * r).collect();
        Self { radii, radii_sqr }
    }

    pub fn radius_sqr(&self, s: usize) -> f32 {
        self.radii_sqr[s]
    }
}

#[derive(Resource)]
pub struct AdaptiveLearningState {
    pub frame_count: usize,
    pub evaluate_interval: usize,
    pub mutation_rate: f32,
    pub last_score: f32,
    pub tested_count: usize,
}

impl Default for AdaptiveLearningState {
    fn default() -> Self {
        Self {
            frame_count: 0,
            evaluate_interval: 600,
            mutation_rate: 0.05,
            last_score: 0.0,
            tested_count: 0,
        }
    }
}

#[derive(Resource)]
pub struct CurrentGenomeIndex(pub usize);

#[derive(Resource)]
pub struct SimulationParameters {
    pub particles_per_species: usize,
}

impl Default for SimulationParameters {
    fn default() -> Self {
        Self {
            particles_per_species: 200,
        }
    }
}

#[derive(Resource, Clone, PartialEq)]
pub enum FitnessMetric {
    Cohesion,
    Dispersion,
    Coverage,
}

impl Default for FitnessMetric {
    fn default() -> Self {
        FitnessMetric::Cohesion
    }
}

#[derive(Resource)]
pub struct Logger {
    file: std::fs::File,
}

impl Logger {
    pub fn new(path: &str) -> Self {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .expect("Unable to open log file");
        Self { file }
    }

    pub fn log(&mut self, generation: usize, fitness: f32) {
        use std::io::Write;
        let _ = writeln!(self.file, "{},{}", generation, fitness);
    }
}
