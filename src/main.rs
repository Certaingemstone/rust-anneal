mod sim;

use sim::lattice::Lattice;
use std::thread;
use std::time::Duration;
use flo_draw::*;

fn main() {
    with_2d_graphics(|| {
        // simulation parameters
        let ti = 1.0;
        let tf = 0.002;
        let step = 0.001;

        // initialize lattice
        let mut lat = Lattice::square(10);
        lat.fill_block(0.5);

        // simulate and draw
        assert_eq!(ti > tf, true);
        assert_eq!(step > 0.0, true);
        let mut t = ti;
        let mut rng = rand::thread_rng();
        let canvas = create_drawing_window("lattice");
        let mut i = 0;
        sim::init_draw(&mut lat, &canvas);
        thread::sleep(Duration::from_secs(1));
        while t > tf {
            sim::step_simulation(&mut lat, &mut rng, &t);
            t -= step;
            i += 1;
            if i % 10 == 0 {
                sim::update_draw(&mut lat, &canvas);
                thread::sleep(Duration::from_secs(1));
            }
        }
    });
}
