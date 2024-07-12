use crate::*;

pub trait Write {
    type Unit;
    fn write<F: FnOnce(&mut Self::Unit)>(&mut self, write: F);
}

pub trait Writer {
    type Unit;
    fn writer<F: FnOnce(&mut Self::Unit)>(&self, write: F);
}

pub trait WriteWithRoot {
    type Unit;
    fn write_with_root<F: FnOnce(&mut Pack<Self::Unit>)>(&mut self, write: F, reactor: &Root);
}

pub trait WriterWithPack {
    type Unit;
    fn writer<F: FnOnce(&mut Pack<Self::Unit>)>(&self, write: F);
}

pub trait Grantor {
    type Load;
    fn grantor(&mut self) -> Self::Load;
}

pub trait Solver {
    type Task;
    type Load;
    fn solver(&mut self, task: Self::Task) -> Self::Load;
}

pub struct Pack<'a, U: 'a> {
    pub unit: &'a mut U,
    pub root: &'a Root,
}

// pub trait WriterWithReactor {
//     type Unit;
//     fn writer_with_reactor<F: FnOnce(&mut Pack<Self::Unit>)>(&self, write: F, reactor: &RootNode);
// }

// pub trait WriteWithReactor {
//     type Unit;
//     fn write_with_reactor<F: FnOnce(&mut Self::Unit, &Reactor)>(
//         &mut self,
//         write: F,
//         reactor: &Reactor,
//     );
// }
