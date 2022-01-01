pub struct Point {
    pub position: [i16; 3],
    pub occupied: bool,
}

pub struct Lattice {
    n: usize,
    pub points: Vec<Point>,
    pub adjacency: Vec<Vec<usize>>, // adjacency list for points
}

impl Lattice {
    pub fn new(pts: Vec<Point>, adj: Vec<Vec<usize>>) -> Result<Self, ()> {
        // TODO: Add some kind of checks that CSRMat adj is valid for the given pts
        // see https://stackoverflow.com/questions/65375808/how-to-validate-struct-creation
        Ok(Self {n: pts.len(), points: pts, adjacency: adj})
    }

    // creates triangular NxN lattice with wraparound boundary conditions
    //pub fn triangular(N: i16) -> Result<Self, ()> {
    //    new(pts, adj)
    //}

    // creates square NxN lattice with wraparound boundary conditions
    pub fn square(N: usize) -> Result<Self, ()> {
        // generate points
        let mut pts: Vec<Point> = Vec::with_capacity(N*N);
        for i in 0..N {
            for j in 0..N {
                let x = i.try_into().unwrap();
                let y = j.try_into().unwrap();
                pts.push(Point {position: [x, y, 0], occupied: false})
            }
        }
        // generate adjacencies, (i,j) adjacent to (i+/-1, j), (i, j+/-1)
        let mut adj_list: Vec<Vec<usize>> = Vec::with_capacity(N);
        let last_col_start = N*N - N;
        for i in 0..N*N {

            let l: usize;
            let r: usize;
            let u: usize;
            let d: usize;

            if i < N { // left boundary on x-y plane
                l = i + last_col_start;
            } else {
                l = i - N;
            }
            
            if i >= last_col_start { // right boundary
                r = i - last_col_start;
            } else {
                r = i + N;
            }

            if i % N == 0 { // bottom boundary
                d = i + N - 1;
            } else {
                d = i - 1;
            }

            if i % N == N - 1 { // top boundary
                u = i + 1 - N;
            } else { 
                u = i + 1;
            }

            adj_list.push(vec![l, r, u, d]);
        }
        Lattice::new(pts, adj_list)
    }
}