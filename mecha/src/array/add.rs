use super::*;

/// Add vector to each vector in base.
pub struct Add {
    base: Stem,
    vector: Vector,
}

impl Add {
    pub fn new(base: &Stem, vector: &Vector) -> Hold<Link<Self>, Role> {
        let link = Link::new(Self {
            base: base.clone(),
            vector: vector.clone(),
        });
        let role = Role {
            part: Part::Add(link.clone()),
            form: link.plan(),
        };
        Hold { link, role }
    }
    pub fn array(&self) -> Array {
        let mut array = self.base.load(Task::Array).array();
        array.each(|i, b| self.vector.get([i[0]]) + b);
        array
    }
}

impl Solve for Add {
    type Load = Load;
    type Task = Task;
    fn solve(&self, task: Task) -> Load {
        match task {
            Task::Array => Bare::Array(self.array()),
            Task::GpuRun => Bare::GpuRun,
        }
        .ace()
    }
}

// self.base.read(Task::Array, |load| {
//     let mut array = load.array().clone();
//     array.each_mut(|i, b|{
//         self.vector.get([i[0]]) + b
//     });
//     array
// })
