use lib::sim::*;
mod lib;

const WIDTH: usize = 80;
const HEIGHT: usize = 80;

fn main() {
    let mut sim: Simulation<WIDTH, HEIGHT> = Simulation::new(10);
    sim.start();
}
