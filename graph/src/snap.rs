use super::*;

#[derive(Debug, Clone)]
pub struct Snap<U> {
    pub imports: Vec<Import>,
    pub unit: U,
}

impl<U> Snap<U> {
    pub fn import(mut self, import: impl Into<Import>) -> Self {
        self.imports.push(import.into());
        self
    }
}

impl<U> Snap<U>
where
    U: 'static + Adapt + Solve + SendSync + Debug,
{
    pub fn apex(self) -> Apex {
        // TODO: make ploy directly so Edge does not need to be read imediately
        Node::from_snap(self).ploy().unwrap().into()
    }
}

pub trait IntoSnapWithImport
where
    Self: Sized,
{
    fn import(self, import: impl Into<Import>) -> Snap<Self>;
}

impl<T> IntoSnapWithImport for T
where
    T: 'static + Adapt + Solve + SendSync + Debug,
{
    fn import(self, import: impl Into<Import>) -> Snap<Self> {
        Snap {
            imports: vec![import.into()],
            unit: self,
        }
    }
}

pub trait IntoSnapWithImports
where
    Self: Sized,
{
    fn imports(self, imports: Vec<Import>) -> Snap<Self>;
}

impl<T> IntoSnapWithImports for T
where
    T: 'static + Adapt + Solve + SendSync + Debug,
{
    fn imports(self, imports: Vec<Import>) -> Snap<Self> {
        Snap {
            imports,
            unit: self,
        }
    }
}
