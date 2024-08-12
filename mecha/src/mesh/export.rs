use super::*;
use std::fs;

const HEAD: &str = r"
# Validation Export
mtllib go_no_go.mtl
";

const GO_HEAD: &str = r"
usemtl GoMat
";

const NO_GO_HEAD: &str = r"
usemtl NoGoMat
";

/// OBJ mesh export.
#[derive(Default, Clone, Debug)]
pub struct ValidationExport {
    /// Expect file system path of destination directory
    path: Node,
    /// Expect interviewed attrib array of XYZ Go/No-Go
    mesh: Node,
}

impl ValidationExport {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set export path
    pub fn path(&mut self, path: impl Into<Node>) -> &mut Self {
        self.path = path.into();
        self
    }

    /// Set mesh to be exported
    pub fn mesh(&mut self, mesh: impl Into<Node>) -> &mut Self {
        self.mesh = mesh.into();
        self
    }

    /// Trade nodes for others with same semantics and different graph properties
    fn trade(&mut self, trade: &dyn Trade) -> adapt::Result {
        self.path = self.path.trade(trade);
        Ok(Gain::None)
    }

    /// Exporter main function
    fn main(&self) -> solve::Result {
        self.mesh.read_vf64(|mesh| self.export(mesh))
    }

    /// Export mesh
    fn export(&self, mesh: &[f64]) -> solve::Result {
        let path = self.path.string()?;
        let mut out = HEAD.to_owned();
        let mut go_out = GO_HEAD.to_owned();
        let mut no_go_out = NO_GO_HEAD.to_owned();
        let mut vertex_index = 1;
        let mut validation_index = 9;
        for d in mesh.windows(10).step_by(10) {
            out += &format!("v {:.6} {:.6} {:.6}\n", d[0], d[1], d[2]);
            out += &format!("v {:.6} {:.6} {:.6}\n", d[3], d[4], d[5]);
            out += &format!("v {:.6} {:.6} {:.6}\n", d[6], d[7], d[8]);
            let face = &format!(
                "f {} {} {}\n",
                vertex_index,
                vertex_index + 1,
                vertex_index + 2
            );
            if mesh[validation_index] > 0.5 {
                go_out += face;
            } else {
                no_go_out += face;
            }
            vertex_index += 3;
            validation_index += 10;
        }
        out += &go_out;
        out += &no_go_out;
        fs::write(path, out)?;
        Ok(Tray::None)
    }
}

impl Adapt for ValidationExport {
    /// Mutate node by Post options
    fn adapt(&mut self, post: Post) -> adapt::Result {
        match post {
            Post::Trade(trade) => self.trade(trade.as_ref()),
            _ => did_not_adapt(post),
        }
    }
}

impl Solve for ValidationExport {
    /// Solve node by Task options
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Main => self.main(),
            _ => did_not_solve(),
        }
    }
}
