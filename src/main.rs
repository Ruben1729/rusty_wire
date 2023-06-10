use lib::sim::*;
mod lib;

const WIDTH: usize = 50;
const HEIGHT: usize = 50;

fn main() {
    let mut sim: Simulation<WIDTH, HEIGHT> = Simulation::new(10);
    sim.start();
}
