mod PSOEngine;
mod Particle;
use crate::PSOEngine::PSOEngine as pso;

//TODO: Fitness Function as Variable, Differnetiate Minimize/Maximize
fn main() {
    // Parameters for the PSO algorithm
    let dimensions = 10;
    let swarm_size = 40;
    let max_iterations = 1000;

    // Bounds for each dimension
    let bounds: Vec<Vec<f64>> = (0..dimensions)
        .map(|_| vec![-5.12, 5.12])
        .collect();

    let w = 0.729844;
    let c1 = 1.496180;
    let c2 = 1.496180;

    // Create the PSO engine
    let mut pso = pso::new(
        swarm_size,
        max_iterations,
        dimensions,
        bounds,
        w,
        c1,
        c2,
    );

    // Run the PSO algorithm
    let best_particle = pso.runPSO();

    // Output the results
    println!("Best fitness: {}", best_particle.fitness);
    println!("Best position: {:?}", best_particle.position);
}
