use super::*;
use std::collections::HashSet;

/// Mesh validation. Produce new mesh with go/no-go attribute.
/// "Go" means the triangle intersects with exactly three trianlges along their edges with same winding.
/// "No-go" means the triangle intersects in some other way like crossing in the middle.
#[derive(Default, Clone, Debug)]
pub struct Validate {
    /// Expect non-indexed single-attribute of XYZ position (Vec<f64>).
    mesh: Node,
    /// Distance threshold for intersection tests
    tolerance: Node,
}

impl Validate {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set source mesh vector
    pub fn mesh(&mut self, vector: impl Into<Node>) -> &mut Self {
        self.mesh = vector.into();
        self
    }

    /// Set intersection threshold
    pub fn tolerance(&mut self, tolerance: impl Into<Node>) -> &mut Self {
        self.tolerance = tolerance.into();
        self
    }

    /// Trade nodes for others with same semantics and different graph properties
    fn trade(&mut self, trade: &dyn Trade) -> adapt::Result {
        self.mesh = self.mesh.trade(trade);
        self.tolerance = self.tolerance.trade(trade);
        Ok(Gain::None)
    }

    /// Validator main function
    fn main(&self) -> solve::Result {
        self.mesh.read_vf64(|source| {
            let mesh = self.mesh_with_go_no_go(source);
            Ok(mesh.leaf().node().tray())
        })
    }

    /// Parametric triangles from mesh source vector
    fn triangles(&self, source: &[f64]) -> Vec<Triangle> {
        // 3 vector components by 3 points equals 9 values to make triangle
        source.windows(9).step_by(9).map(Triangle::new).collect()
    }

    /// Make go/no-go mesh vector
    fn mesh_with_go_no_go(&self, source: &[f64]) -> Vec<f64> {
        let triangles = self.triangles(source);
        let mut mesh = vec![];
        for (j, tri_a) in triangles.iter().enumerate() {
            // copy position attribute
            mesh.extend(&source[j * 9..(j * 9 + 9)]);
            let intersects = self.intersects(tri_a, &triangles);
            self.push_go_no_go(&mut mesh, intersects);
        }
        mesh
    }

    /// Find intersection type hash set by comparing with other triangles
    fn intersects(&self, tri_a: &Triangle, triangles: &[Triangle]) -> HashSet<Intersection> {
        let mut intersection_set = HashSet::new();
        for tri_b in triangles {
            if tri_a != tri_b {
                if let Some(intersect) = tri_a.intersect(tri_b, self.tolerance.f64()) {
                    intersection_set.insert(intersect);
                }
            }
        }
        intersection_set
    }

    /// Push go/no-go attribute element to mesh vector
    fn push_go_no_go(&self, mesh: &mut Vec<f64>, intersects: HashSet<Intersection>) {
        // Push 1.0 (Go) if triangle intersects with only 3 others on the edge and no crossings
        if intersects.len() == 3 && !intersects.contains(&Intersection::Cross) {
            mesh.push(1.0);
        } else {
            mesh.push(0.0);
        }
    }
}

impl Adapt for Validate {
    /// Mutate node by Post options
    fn adapt(&mut self, post: Post) -> adapt::Result {
        match post {
            Post::Trade(trade) => self.trade(trade.as_ref()),
            _ => did_not_adapt(post),
        }
    }
}

impl Solve for Validate {
    /// Solve node by Task options
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Main => self.main(),
            _ => did_not_solve(),
        }
    }
}
