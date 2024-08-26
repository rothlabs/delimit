use super::*;

#[derive(Deserialize)]
pub struct Snap<U> {
    imports: Vec<Import>,
    unit: U,
} 

impl<U> Snap<U> {
    pub fn import(mut self, import: impl Into<Import>) -> Self {
        self.imports.push(import.into());
        self
    }
}

impl<U> Snap<U> 
where 
    U: 'static + Adapt + Solve + SendSync + Debug
{
    pub fn apex(self) -> Apex {
        Node::make2(self.unit, self.imports).ploy().into()
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
            unit: self
        }
    }
}