# ALFA: Adaptive Learning & Flexibility for Artificial Life


![Screenshot 2024-12-06 194031](https://github.com/user-attachments/assets/269d7cc4-2318-4700-8f97-d72cecfb468a)



ALFA is an experimental artificial life (A-Life) simulation platform designed to investigate how heterogeneity in agent populations influences collective dynamics, swarm intelligence, and emergent behaviors. By integrating adaptive learning mechanisms, morphological diversity, and flexible interaction rules, ALFA aims to provide new insights into the formation, stability, and adaptability of complex swarms—paving the way for more efficient and ethical swarm-based systems in both research and applied contexts.

## Overview

In nature, swarms of organisms such as birds, fish, and insects display remarkable coordination and robustness, despite individual agents often having limited capabilities and information. While homogeneous swarm models have advanced our understanding of collective behavior, there's less clarity on how heterogeneity in agent shapes, behaviors, and roles affects swarm-level properties.

ALFA addresses this gap by introducing heterogeneous agents—each species potentially having different interaction rules, morphologies, and behavioral tendencies. By employing adaptive learning and evolutionary strategies, the system continuously refines the interaction coefficients and morphological parameters to optimize for given objectives, such as minimizing cluster dispersion or achieving certain swarm formations.



https://github.com/user-attachments/assets/b7ead470-fecd-4f56-ba65-322cf216f868

## Key Features

- **Heterogeneous Swarms**: Multiple species of agents, each with distinct radii, interaction patterns, and morphological parameters, interact within the same environment.
  
- **Adaptive Learning**: A built-in evolutionary mechanism evaluates swarm configurations over periodic intervals. It measures "fitness" (e.g., based on cluster cohesion or other metrics) and uses selection, crossover, and mutation to evolve better interaction rules over time.
  
- **Flexible Interaction Rules**: The interactions between agents are governed by a matrix of coefficients defining attraction/repulsion forces. These coefficients evolve, enabling the swarm to adapt to complex environments and tasks.
  
- **Customizable Parameters**: Viscosity, gravity, wall repulsion, and time scaling can be adjusted interactively. Species-specific parameters like radii are also tunable, allowing real-time experimentation and rapid prototyping of swarm dynamics.
  
- **Visual & UI Integration**: With integrated UI controls (via `bevy_egui`) and geometric rendering (via `bevy_prototype_lyon`), you can intuitively modify simulation parameters, observe changes in real-time, and gain immediate visual feedback.

## Research & Applications

ALFA aspires to shed light on:
- **Complex Swarm Behaviors**: How do groups composed of diverse agent species self-organize into stable, functional patterns?
- **Optimization of Morphologies**: Which agent configurations yield optimal results for tasks like area coverage, cluster formation, or environment exploration?


## Getting Started

### Prerequisites

- **Rust and Cargo**: Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
- **Git**: Required to clone the repository.

### Cloning the Repository

```bash
git clone https://github.com/Alfaxad/ALFA.git
cd ALFA
```

### build the project

```bash
cargo build
```
### run the project

```bash
cargo run
```

