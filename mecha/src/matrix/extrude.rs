use super::*;

/// Adds vector to each column and keeps the original columns.
/// This results in a matrix with double the columns.
/// If each column is a shape point, this extrudes the shape by vector.  
pub struct Extrude {
    stem: Stem,
    vector: Vector,  
}

impl Extrude {
    pub fn new(stem: &Stem, vector: &Vector) -> Hold<Link<Self>, Role> {
        let link = Link::new(Self {
            stem: stem.clone(),
            vector: vector.clone(), 
        });
        let role = Role {
            part: Part::Extrude(link.clone()),
            form: link.plan(),
        };
        Hold { link, role }
    }
    pub fn matrix(&self) -> Matrix {
        let mut out = Matrix::new(4, 4);
        // let wow = self.stem.grant();
        self.stem.read(Task::Matrix, |unit| {
            if let Load::Matrix(mat) = unit {
                
            }
        });
        out
    }
}

impl Solve for Extrude {
    type Load = Load;
    type Task = Task;
    fn solve(&self, task: Task) -> Load {
        match task {
            Task::Matrix => Load::Matrix(self.matrix()),
            Task::GpuMap => Load::GpuMap,
        }
    }
}

