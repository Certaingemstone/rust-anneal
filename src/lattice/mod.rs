mod sparse;

pub struct Point {
    pub position: [i32; 3],
    pub occupied: bool,
}

pub struct Lattice {
    n: usize,
    pub points: Vec<Point>,
    pub adjacency: sparse::CSRMat,
}

impl Lattice {
    pub fn new(pts: Vec<Point>, adj: sparse::CSRMat) -> Result<Self, ()> {
        // TODO: Add some kind of checks that CSRMat adj is valid for the given pts
        // see https://stackoverflow.com/questions/65375808/how-to-validate-struct-creation
        Ok(Self {n: pts.len(), points: pts, adjacency: adj})
    }

    pub fn triangular() {
        // creates triangular lattice, like in 8.044 exam
    }

    pub fn square() {
        // creates square lattice
    }
}