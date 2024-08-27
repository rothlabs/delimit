use crate::*;

/// Work that holds a tray. The most simple work that allows read, write, and copy of the tray.
#[derive(Debug, Hash, Serialize, Deserialize)]
pub struct Leaf {
    pub tray: Tray,
    // digest: Option<Gain>,
}

impl Leaf {
    pub fn apex(self) -> Apex {
        Apex::Leaf(link::Leaf::new(self.tray))
    }
    fn digest(&mut self) -> solve::Result {
        let mut state = DefaultHasher::new();
        self.tray.hash(&mut state);
        state.finish().gain()
    }
}

impl FromItem for Leaf {
    type Item = Tray;
    fn new(tray: Self::Item) -> Self {
        Self { tray }
    }
}

impl ToTray for Leaf {
    type Tray = Tray;
    fn tray(&self) -> Self::Tray {
        self.tray.clone()
    }
}

impl DoRead for Leaf {
    type Item = Tray;
    fn do_read(&self) -> &Self::Item {
        &self.tray
    }
}

impl DoReadTray for Leaf {
    fn do_read_tray(&self) -> tray::ResultRef {
        Ok(&self.tray)
    }
}

impl WriteTrayWork for Leaf {
    type Item = Tray;
    fn write_tray_work<T, F: FnOnce(&mut Self::Item) -> T>(&mut self, write: F) -> T {
        write(&mut self.tray)
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
            Task::Digest(state) => self.digest(),
            _ => no_solver(self, task),
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
//         self.tray.serialize(serializer)
//     }
// }
