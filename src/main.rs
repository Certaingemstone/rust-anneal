mod sim;

use sim::lattice::Lattice;
use std::{io, thread};
use std::time::Duration;

fn main() {
    // simulation parameters
    let mut buffer = String::new();
    let mut stdin = io::stdin();

    // println!("Enter initial temperature");
    // stdin.read_line(&mut buffer).expect("failed to read input");
    // let ti: f32 = buffer.trim().parse().unwrap();
    // buffer.clear();
    let ti: f32 = 1.0;

    println!("Enter temperature step (t starts at 1.0)");
    stdin.read_line(&mut buffer).expect("failed to read input");
    let step: f32 = buffer.trim().parse().unwrap();
    buffer.clear();

    let tf = step * 3.0 + 0.005;

    println!("Choose lattice size");
    stdin.read_line(&mut buffer).expect("failed to read input");
    let dim: usize = buffer.trim().parse().unwrap();
    buffer.clear();

    // initialize lattice
    println!("Choose lattice type: square, tri");
    stdin.read_line(&mut buffer).expect("failed to read input");
    let mut lat: Lattice;
    match buffer.trim() {
        "square" => {
            lat = Lattice::square(dim);
        }
        "tri" => {
            lat = Lattice::triangular(dim);
        }
        _ => { panic!(); }
    }
    buffer.clear();
    
    println!("Choose lattice fill proportion (0,1)");
    stdin.read_line(&mut buffer).expect("failed to read input");
    let fill: f32 = buffer.trim().parse().unwrap();
    assert_eq!(fill > 0.0, true);
    assert_eq!(fill < 1.0, true);
    lat.fill_random(fill);
    buffer.clear();

    println!("Animate? (y/n)");
    stdin.read_line(&mut buffer).expect("failed to read input");
    let draw: bool;
    match buffer.trim() {
        "y" => {
            draw = true;
        }
        _ => {
            draw = false;
        }
    }

    // simulate and draw
    assert_eq!(ti > tf, true);
    assert_eq!(step > 0.0, true);
    let mut t = ti;
    let mut rng = rand::thread_rng();
    // let canvas = create_drawing_window("lattice");
    let mut i = 0;
    // sim::init_draw(&mut lat, &canvas);
    if draw {
        while t > tf {
            sim::step_simulation(&mut lat, &mut rng, &t);
            t -= step;
            i += 1;
            if i % dim == 0 {
                // sim::update_draw(&mut lat, &canvas);
                // thread::sleep(Duration::from_millis(1000));
                println!("Temperature: {}", t);
            }
        }
    } else {
        while t > tf {
            sim::step_simulation(&mut lat, &mut rng, &t);
            t -= step;
            println!("Temperature: {}", t);
        }
        // sim::update_draw(&mut lat, &canvas);
    }
        
    println!("Done!")
}
