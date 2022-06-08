pub mod lattice;

use std::collections::HashSet;
// use flo_draw::*;
// use flo_canvas::*;
use lattice::{Lattice, Point};

// pub fn init_draw(lat: &mut lattice::Lattice, canvas: &flo_canvas::DrawingTarget) {
//     let bounds: [i16; 4] = lat.boundaries();
//     let pts: Vec<Point> = lat.points.clone();
//     let occ: HashSet<usize> = lat.occupied.set.clone();
//     let adj: Vec<Vec<usize>> = lat.adjacency.clone();
//     let scale = 1000.0 / *bounds.iter().max().unwrap() as f32;
//     let sbounds: Vec<f32> = bounds.iter().map(|x| *x as f32 * scale).collect();
//     let radius = 15.0;

//     canvas.draw(|gc| {
//         gc.clear_canvas(Color::Rgba(0.0, 0.0, 0.0, 1.0));
//         gc.canvas_height(1100.0);
//         gc.center_region(sbounds[0] - 50.0, sbounds[2] - 50.0, sbounds[1] + 50.0, sbounds[3] + 50.0);
//         gc.stroke_color(Color::Rgba(1.0, 1.0, 1.0, 1.0));
//         gc.line_width(3.0);

//         // draw each connection in the lattice
//         let mut i = 0;
//         for avec in adj {
//             let x0 = scale * pts[i].x as f32; 
//             let y0 = scale * pts[i].y as f32;
//             for a in avec {
//                 gc.new_path();
//                 gc.move_to(x0, y0);
//                 gc.line_to(scale * pts[a].x as f32, scale * pts[a].y as f32);
//                 gc.stroke();
//             }
//             i += 1;
//         }

//         // draw each site in the lattice
//         i = 0;
//         for p in pts {
//             gc.new_path();
//             gc.circle(p.x as f32 * scale, p.y as f32 * scale, radius);
//             if occ.contains(&i) {
//                 gc.fill_color(Color::Rgba(0.3, 1.0, 0.3, 1.0));
//             } else {
//                 gc.fill_color(Color::Rgba(0.0, 0.0, 0.0, 1.0));
//             }  
//             gc.fill();
//             gc.stroke();
//             i += 1;
//         }
        
//     });
// }

// pub fn update_draw(lat: &mut Lattice, canvas: &flo_canvas::DrawingTarget) {
//     let bounds = lat.boundaries(); // xmin, xmax, ymin, ymax
//     let pts: Vec<Point> = lat.points.clone();
//     let occ: HashSet<usize> = lat.occupied.set.clone();

//     let scale = 1000.0 / *bounds.iter().max().unwrap() as f32;
//     let radius = 15.0;

//     canvas.draw(|gc| {
//         // refill sites
//         let mut i = 0;
//         for p in pts {
//             gc.new_path();
//             gc.circle(p.x as f32 * scale, p.y as f32 * scale, radius);
//             if occ.contains(&i) {
//                 gc.fill_color(Color::Rgba(0.3, 1.0, 0.3, 1.0));
//             } else {
//                 gc.fill_color(Color::Rgba(0.0, 0.0, 0.0, 1.0));
//             }  
//             gc.fill();
//             i += 1;
//         }       
//     });
// }

pub fn step_simulation<R>(lattice: &mut Lattice, rng: &mut R, temp: &f32) 
where R: rand::Rng + ?Sized { 
    lattice.perform_move(rng, temp);
}
