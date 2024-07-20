use std::cell::RefCell;
use std::rc::Rc;
use rand::rngs::StdRng;
use rand::{Rng};

pub(crate) struct Particle {
    pub(crate) bestFitness : f64,
    pub(crate) fitness : f64,
    pub(crate) position : Vec<f64>,
    pub(crate) velocity : Vec<f64>,
    pub(crate) personalBest : Vec<f64>,
    pub(crate) generator : Rc<RefCell<StdRng>>
}
impl Particle {
    pub fn newMinimizer(position: Vec<f64>, velocity: Vec<f64>, generator: Rc<RefCell<StdRng>>) -> Self {
        Particle {
            bestFitness: std::f64::INFINITY,
            fitness: std::f64::INFINITY,
            position,
            velocity,
            personalBest: Vec::new(),
            generator,
        }
    }
    pub fn newMaximizer(position: Vec<f64>, velocity: Vec<f64>, generator: Rc<RefCell<StdRng>>) -> Self {
        Particle {
            bestFitness: f64::NEG_INFINITY,
            fitness: f64::NEG_INFINITY,
            position,
            velocity,
            personalBest: Vec::new(),
            generator,
        }
    }
    pub fn newFinal(position : Vec<f64>, velocity : Vec<f64>, fitness : f64, generator : Rc<RefCell<StdRng>>) -> Self {
        Particle{
            bestFitness : fitness,
            fitness,
            position : position.clone(),
            velocity,
            personalBest : position,
            generator
        }
    }
    pub fn update_velocity(&mut self, neighbourhoodBest: &[f64], w: f64, c1: f64, c2: f64) {
        let mut rng = self.generator.borrow_mut();
        for i in 0..self.velocity.len() {
            self.velocity[i] = (w * self.velocity[i])
                + (c1 * rng.gen::<f64>() * (self.personalBest[i] - self.position[i]))
                + (c2 * rng.gen::<f64>() * (neighbourhoodBest[i] - self.position[i]));
        }
    }
    pub fn set_fitness(&mut self, fitness: f64){
        self.fitness = fitness;
    }
}