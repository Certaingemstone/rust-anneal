mod sim;

use sim::lattice::Lattice;
use std::{io, thread};
use std::time::Duration;
use flo_draw::*;

fn main() {
    with_2d_graphics(|| {
        // simulation parameters
        let mut buffer = String::new();
        let mut stdin = io::stdin();

        println!("Enter initial temperature as float");
        stdin.read_line(&mut buffer).expect("failed to read input");
        let ti: f32 = buffer.trim().parse().unwrap();
        buffer.clear();

        println!("Enter temperature step as float");
        stdin.read_line(&mut buffer).expect("failed to read input");
        let step: f32 = buffer.trim().parse().unwrap();
        buffer.clear();

        let tf = step * 2.0;

        // initialize lattice
        println!("Choose lattice type: square, tri");
        stdin.read_line(&mut buffer).expect("failed to read input");
        let mut lat: Lattice;
        match buffer.trim() {
            "square" => {
                lat = Lattice::square(10);
            }
            "tri" => {
                lat = Lattice::triangular(10);
            }
            _ => { panic!(); }
        }
        buffer.clear();
        
        println!("Choose lattice fill proportion as float (0,1)");
        stdin.read_line(&mut buffer).expect("failed to read input");
        let fill: f32 = buffer.trim().parse().unwrap();
        assert_eq!(fill > 0.0, true);
        assert_eq!(fill < 1.0, true);
        lat.fill_block(fill);
        buffer.clear();

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
