pub trait Write {
    type Unit;
    fn write<F: FnOnce(&mut Self::Unit)>(&mut self, write: F); 
}

pub trait WriteInner {
    type Unit;
    fn write<F: FnOnce(&mut Self::Unit)>(&self, write: F);
}