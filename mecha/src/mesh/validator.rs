use super::*;
use std::collections::HashSet;

/// Mesh validation. Produces new mesh with go/no-go attribute.
/// "Go" means the triangle intersections with exactly three trianlges along their edges with same winding.
/// "No-go" means the triangle intersections in some other way such as a crossing in the middle.
#[derive(Default, Clone, Debug)]
pub struct Validator {
    /// Expect non-indexed single-attribute of XYZ position.
    mesh: Node,
    /// Distance threshold for hit/miss of intersection tests
    tolerance: Node,
}

impl Validator {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn vector(&mut self, vector: impl Into<Node>) -> &mut Self {
        self.mesh = vector.into();
        self
    }
    pub fn tolerance(&mut self, tolerance: impl Into<Node>) -> &mut Self {
        self.tolerance = tolerance.into();
        self
    }
    fn trade(&mut self, trade: &dyn Trade) -> adapt::Result {
        self.mesh = self.mesh.trade(trade);
        self.tolerance = self.tolerance.trade(trade);
        Ok(Gain::None)
    }
    /// Validator main function 
    fn main(&self) -> solve::Result {
        self.mesh.read_vf64(|source| {
            let mesh = self.mesh(source);
            Ok(mesh.leaf().node().tray())
        })
    }
    /// Parametric triangles from mesh source vector
    fn triangles(&self, source: &Vec<f64>) -> Vec<Triangle> {
        source
            .windows(9)
            .step_by(9)
            .map(|points| Triangle::new(&points))
            .collect()
    }
    /// Make go/no-go mesh vector
    fn mesh(&self, source: &Vec<f64>) -> Vec<f64> {
        let triangles = self.triangles(source);
        let mut mesh = vec![];
        for (i, tri_a) in triangles.iter().enumerate() {
            mesh.extend(&source[i..i + 9]);
            let intersects = self.intersects(tri_a, &triangles);
            self.go_no_go(&mut mesh, intersects);
        }
        mesh
    }
    /// Find intersection hash set by comparing with other triangles
    fn intersects(&self, tri_a: &Triangle, triangles: &Vec<Triangle>) -> HashSet<Intersection> {
        let mut intersects = HashSet::new();
        for tri_b in triangles {
            if tri_a != tri_b {
                if let Some(intersect) = tri_a.intersect(tri_b, self.tolerance.f64()) {
                    intersects.insert(intersect);
                }
            }
        }
        intersects
    }
    /// Push go/no-go attribute element to mesh vector
    fn go_no_go(&self, mesh: &mut Vec<f64>, intersects: HashSet<Intersection>) {
        if intersects.len() == 3 && !intersects.contains(&Intersection::Other) {
            mesh.push(1.);
        } else {
            mesh.push(0.);
        }
    }
}

impl Adapt for Validator {
    fn adapt(&mut self, post: Post) -> adapt::Result {
        match post {
            Post::Trade(trade) => self.trade(trade.as_ref()),
            _ => did_not_adapt(post),
        }
    }
}

impl Solve for Validator {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Main => self.main(),
            _ => did_not_solve(),
        }
    }
}
