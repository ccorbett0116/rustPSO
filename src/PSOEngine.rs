use std::cell::RefCell;
use std::rc::Rc;
use crate::Particle::Particle;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;

pub(crate) struct PSOEngine {
    bounds : Vec<Vec<f64>>,
    swarm : Vec<Particle>,
    globalBest : Vec<f64>,
    globalBestFitness : f64,
    maxIterations : i64,
    w : f64,
    c1 : f64,
    c2 : f64,
    generator : Rc<RefCell<StdRng>>
}
impl PSOEngine {
    pub fn new(
        swarmSize: usize,
        maxIterations: i64,
        dimensions: usize,
        bounds: Vec<Vec<f64>>,
        w: f64,
        c1: f64,
        c2: f64, ) -> Self {
        let generator = Rc::new(RefCell::new(StdRng::seed_from_u64(7246325)));
        let globalBest = vec![f64::INFINITY; dimensions];
        let mut swarm = Vec::with_capacity(swarmSize);
        for _ in 0..swarmSize {
            let generatorClone = Rc::clone(&generator);
            let position: Vec<f64> = (0..dimensions)
                .map(|i| generatorClone.borrow_mut().gen_range(bounds[i][0]..bounds[i][1]))
                .collect();
            let velocity = vec![0.0; dimensions];

            // Choose either newMinimizer or newMaximizer based on optimization goal
            let particle = Particle::newMinimizer(position, velocity, generatorClone); // You can use `generator` directly if not cloning

            swarm.push(particle);
        }
        PSOEngine {
            maxIterations,
            w,
            c1,
            c2,
            bounds,
            swarm,
            globalBest,
            globalBestFitness: f64::INFINITY,
            generator
        }
    }
    pub fn runPSO(&mut self) -> Particle {
        for _i in 0..self.maxIterations {
            // Initialize neighbourhood best fitness to positive infinity
            let mut neighbourhoodBestFitness = f64::INFINITY;
            let mut neighbourhoodBestPosition = Vec::new();

            for p in &mut self.swarm {
                // Set fitness
                p.set_fitness(Self::rastriginFitness(p));

                // Update personal best
                if p.fitness <= p.bestFitness {
                    p.bestFitness = p.fitness;
                    p.personalBest = p.position.clone(); // Copy the position
                }

                // Update neighbourhood best
                if p.bestFitness <= neighbourhoodBestFitness {
                    neighbourhoodBestFitness = p.bestFitness;
                    neighbourhoodBestPosition = p.personalBest.clone(); // Copy the personal best
                }
            }

            // Update global best
            if neighbourhoodBestFitness <= self.globalBestFitness {
                self.globalBestFitness = neighbourhoodBestFitness;
                self.globalBest = neighbourhoodBestPosition; // Assign the best position found
            }

            // Add further particle updates if needed
            for p in &mut self.swarm {
                p.update_velocity(&self.globalBest, self.w, self.c1, self.c2);
                for j in 0..p.position.len(){
                    p.position[j] += p.velocity[j];
                    if p.position[j] < self.bounds[j][0] {
                        p.position[j] = self.bounds [j][0]
                    }
                    else if p.position[j] > self.bounds[j][1] {
                        p.position[j] = self.bounds [j][1]
                    }
                }
            }
        }
        Particle::newFinal(self.globalBest.clone(), vec![0.0; self.globalBest.len()], self.globalBestFitness, Rc::clone(&self.generator))
    }
    pub fn rastriginFitness(p: &Particle) -> f64 {
        let mut sum = 10.0 * (p.position.len() as f64);
        for d in &p.position {
            let cos_term = f64::cos(2.0 * std::f64::consts::PI * d);
            sum += d.powf(2.0) - 10.0 * cos_term;
        }
        sum
    }
}
