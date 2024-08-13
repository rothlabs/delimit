use crate::*;

/// Work that holds a load. The most simple work that allows read, write, and copy of the load.
#[derive(Debug, Hash, Serialize)]
pub struct Leaf {
    load: Load,
}

impl FromItem for Leaf {
    type Item = Load;
    fn new(load: Self::Item) -> Self {
        Self { load }
    }
}

impl ToLoad for Leaf {
    type Load = Load;
    fn load(&self) -> Self::Load {
        self.load.clone()
    }
}

impl DoRead for Leaf {
    type Item = Load;
    fn do_read(&self) -> &Self::Item {
        &self.load
    }
}

impl DoReadLoad for Leaf {
    fn do_read_load(&self) -> load::ResultRef {
        Ok(&self.load)
    }
}

impl WriteLoadWork for Leaf {
    type Item = Load;
    fn write_load_work<T, F: FnOnce(&mut Self::Item) -> T>(&mut self, write: F) -> T {
        write(&mut self.load)
    }
}

impl DoReact for Leaf {
    fn do_react(&mut self, _: &Id) -> react::Result {
        Ok(())
    }
}

impl DoSolve for Leaf {
    fn do_solve(&mut self, task: Task) -> solve::Result {
        match task {
            Task::Serial => self.serial(),
            Task::Hash => self.digest(),
            _ => no_solver()
        }
    }
}

impl DoRebut for Leaf {
    fn do_rebut(&mut self) -> Ring {
        Ring::new()
    }
}

impl Clear for Leaf {
    fn clear(&mut self) {}
}

// impl Serialize for Leaf {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         self.load.serialize(serializer)
//     }
// }