use super::*;

pub struct Extrude {
    stem: Stem,
    vector: Vector, 
}

impl Solve for Extrude {
    type Load = Load;
    type Task = Task;
    fn solve(&self, task: Task) -> Load {
        let wow = self.stem.solve(task);
    }
}

