mod sim;

use sim::lattice::Lattice;

fn main() {
    // simulation parameters
    let ti = 10.0;
    let tf = 0.02;
    let step = 0.01;
    // initialize lattice
    let mut lat = Lattice::square(10);
    lat.fill_block(0.5);
    sim::anneal(&mut lat, ti, tf, step);
    sim::draw_lattice(&lat);
}
