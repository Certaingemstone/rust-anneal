use rand::{thread_rng, Rng};
use std::collections::HashSet;

pub struct VecSet<T> {
    pub set: HashSet<T>,
    pub vec: Vec<T>,
}

impl<T> VecSet<T> 
where T: Clone + Eq + std::hash:: Hash {
    fn new() -> Self {
        Self {
            set: HashSet::new(),
            vec: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.set.clear();
        self.vec.clear();
    } 

    fn insert(&mut self, elem: T) {
        assert_eq!(self.set.len(), self.vec.len());
        let was_new = self.set.insert(elem.clone());
        if was_new {
            self.vec.push(elem);
        }
    }

    fn remove_random(&mut self) -> T {
        assert_eq!(self.set.len(), self.vec.len());
        let index = thread_rng().gen_range(0..self.vec.len());
        let elem = self.vec.swap_remove(index); 
        let was_present = self.set.remove(&elem);
        assert!(was_present);
        elem
    }

    fn difference(&self, vec: &Vec<T>) -> Vec<T> {
        let mut res: Vec<T> = Vec::with_capacity(vec.len() + 1);
        // get vec, without elements that are contained in self
        for elem in vec {
            if !self.contains(elem) {
                res.push(elem.clone())
            }
        }
        res
    }

    pub fn contains(&self, elem: &T) -> bool {
        self.set.contains(elem)
    }

}

#[derive(Clone)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

pub struct Lattice {
    n: usize, // number of points
    pub points: Vec<Point>,
    pub occupied: VecSet<usize>, // indices into points
    pub adjacency: Vec<Vec<usize>>, // adjacency list for points
}

// Methods
impl Lattice {

    pub fn fill_block(&mut self, frac: f32) {
        let amount = frac * (self.n) as f32;
        self.occupied.clear();
        for i in 0..amount as usize {
            self.occupied.insert(i)
        }
    }

    pub fn fill_random(&mut self, frac: f32) {
        let mut rng = rand::thread_rng();
        let amount = frac * (self.n) as f32;
        let indexvec = rand::seq::index::sample(&mut rng, self.n, amount as usize);
        self.occupied.clear();
        for i in indexvec {
            self.occupied.insert(i);
        }
    }

    pub fn boundaries(&self) -> [i16; 4] {
        let mut xmin: i16 = 32767;
        let mut xmax: i16 = -32767;
        let mut ymin: i16 = 32767;
        let mut ymax: i16 = -32767;
        for p in &self.points {
            if p.x < xmin {
                xmin = p.x;
            }
            if p.x > xmax {
                xmax = p.x;
            }
            if p.y < ymin {
                ymin = p.y;
            }
            if p.y > ymax {
                ymax = p.y;
            }
        }
        [xmin, xmax, ymin, ymax]
    }

    pub fn perform_move<R>(&mut self, rng: &mut R, temp: &f32)
    where R: rand::Rng + ?Sized {
        let target_indices = self.remove_and_propose_moves();
        let target_energies = self.get_energies(&target_indices);
        let destination_index = self.choose_move(rng, &target_indices, &target_energies, temp);
        self.occupied.insert(destination_index);
    }

    fn remove_and_propose_moves(&mut self) -> Vec<usize> {
        // find a random occupied point and make it unoccupied
        let idx = self.occupied.remove_random();
        // get its neighbors
        let mut targets: Vec<usize> = self.occupied
            .difference(&self.adjacency[idx]); // that are unoccupied,
        targets.push(idx); // and include the first point
        targets
    }

    fn get_energies(&self, targets: &Vec<usize>) -> Vec<f32> {
        let mut res: Vec<f32> = Vec::with_capacity(targets.len());
        for idx in targets {
            // iterate over its neighbors
            let mut energy: f32 = 0.0;
            for adj_idx in &self.adjacency[*idx] {
                if self.occupied.contains(adj_idx) {
                    energy += 1.0;
                }
            }
            res.push(energy)
        }
        res
    }

    fn choose_move<R>(&mut self, rng: &mut R, targets: &Vec<usize>, energies: &Vec<f32>, temp: &f32) -> usize 
    where R: rand::Rng + ?Sized {
        if targets.len() == 1 {
            return targets[0]
        }
        // canonical ensemble state transition
        let rel_probabilities: Vec<f32> = energies.iter().map(|e| 10000.0 * libm::expf(-e/temp)).collect();
        let partition: f32 = rel_probabilities.iter().sum();
        println!("{}", partition);
        let probabilities: Vec<f32> = rel_probabilities.iter().map(|p| p / partition).collect();
        // choose an index from targets
        let r: f32 = rng.gen_range(0.0..1.0);
        let mut c: f32 = 0.0;
        let mut i: usize = 0;
        println!("{:?}", probabilities);
        for p in probabilities {
            c += p;
            if r < c {
                break;
            }
            i += 1;
        }
        targets[i]
    }
}

// Constructors
impl Lattice {

    pub fn new(pts: Vec<Point>, adj: Vec<Vec<usize>>) -> Self {
        // see https://stackoverflow.com/questions/65375808/how-to-validate-struct-creation
        Self {n: pts.len(), points: pts, occupied: VecSet::new(), adjacency: adj}
    }

    // creates triangular NxN lattice with wraparound boundary conditions
    //pub fn triangular(N: i16) -> Result<Self, ()> {
    //    new(pts, adj)
    //}

    // creates square NxN lattice with wraparound boundary conditions
    pub fn square(n: usize) -> Self {
        // generate points
        let mut pts: Vec<Point> = Vec::with_capacity(n*n);
        for i in 0..n {
            for j in 0..n {
                let x = i.try_into().unwrap();
                let y = j.try_into().unwrap();
                pts.push(Point {x: x, y: y})
            }
        }
        // generate adjacencies, (i,j) adjacent to (i+/-1, j), (i, j+/-1)
        let mut adj_list: Vec<Vec<usize>> = Vec::with_capacity(n);
        let last_col_start = n*n - n;
        for i in 0..n*n {

            let l: usize;
            let r: usize;
            let u: usize;
            let d: usize;

            if i < n { // left boundary on x-y plane
                l = i + last_col_start;
            } else {
                l = i - n;
            }
            
            if i >= last_col_start { // right boundary
                r = i - last_col_start;
            } else {
                r = i + n;
            }

            if i % n == 0 { // bottom boundary
                d = i + n - 1;
            } else {
                d = i - 1;
            }

            if i % n == n - 1 { // top boundary
                u = i + 1 - n;
            } else { 
                u = i + 1;
            }

            adj_list.push(vec![l, r, u, d]);
        }
        Lattice::new(pts, adj_list)
    }
}