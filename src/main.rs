mod lattice;

use lattice::Lattice;

fn main() {
    // initialize lattice
    let mut lat = Lattice::square(4);
    lat.fill(0.25);
}

fn step(lattice: &mut Lattice) {

}
