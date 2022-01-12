pub mod lattice;

use std::collections::HashSet;
use flo_draw::*;
use flo_canvas::*;
use lattice::{Lattice, Point};

pub fn draw_lattice(lat: &Lattice) {
    let bounds = lat.boundaries(); // xmin, xmax, ymin, ymax
    let testpoints: Vec<Point> = lat.points.clone();
    let occupied: HashSet<usize> = lat.occupied.set.clone();
    
    with_2d_graphics(move || {
        
        let scale = 1000.0 / *bounds.iter().max().unwrap() as f32;
        let sbounds: Vec<f32> = bounds.iter().map(|x| *x as f32 * scale).collect();
        let radius = 15.0;
        let canvas = create_drawing_window("Lattice");

        canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(0.0, 0.0, 0.0, 1.0));
            gc.canvas_height(1100.0);
            gc.center_region(sbounds[0] - 50.0, sbounds[2] - 50.0, sbounds[1] + 50.0, sbounds[3] + 50.0);

            // draw each site in the lattice
            let mut i = 0;
            for p in testpoints {
                gc.new_path();
                gc.circle(p.x as f32 * scale, p.y as f32 * scale, radius);
                if occupied.contains(&i) {
                    gc.fill_color(Color::Rgba(0.3, 1.0, 0.3, 1.0));
                } else {
                    gc.fill_color(Color::Rgba(0.3, 0.3, 0.3, 1.0));
                }  
                gc.fill();
                gc.line_width(1.0);
                gc.stroke_color(Color::Rgba(1.0, 1.0, 1.0, 1.0));
                gc.stroke();
                i += 1;
            }

            // draw each connection in the lattice
            
        });
    });
}

pub fn anneal(lattice: &mut Lattice, temp_i: f32, temp_f: f32, step: f32) {
    assert_eq!(temp_i > temp_f, true);
    assert_eq!(step > 0.0, true);
    let mut temp: f32 = temp_i;
    let mut rng = rand::thread_rng();
    while temp > temp_f {
        lattice.perform_move(&mut rng, &temp);
        temp -= step;
    }
}

#[allow(dead_code)]
pub fn test_draw() {
    with_2d_graphics(|| {
        let canvas = create_drawing_window("Hello");

        canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(0.0, 0.0, 0.0, 1.0));
            gc.canvas_height(1000.0);
            gc.center_region(0.0, 0.0, 1000.0, 1000.0);

            // Draw a rectangle...
            gc.new_path();
            gc.move_to(0.0, 0.0);
            gc.line_to(1000.0, 0.0);
            gc.line_to(1000.0, 1000.0);
            gc.line_to(0.0, 1000.0);
            gc.line_to(0.0, 0.0);

            gc.fill_color(Color::Rgba(1.0, 1.0, 0.8, 1.0));
            gc.fill();

            // Draw a triangle on top
            gc.new_path();
            gc.move_to(200.0, 200.0);
            gc.line_to(800.0, 200.0);
            gc.line_to(500.0, 800.0);
            gc.line_to(200.0, 200.0);

            gc.fill_color(Color::Rgba(0.0, 0.0, 0.8, 1.0));
            gc.fill();
        });
    });
}