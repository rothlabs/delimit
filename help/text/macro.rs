//////////////// Make a struct?
macro_rules! make_printer {
    ($($element: ident: $ty: ty),*) => {
        struct Foo { $($element: $ty),* }
    }
}
make_printer!(x: i32, y: String);




//////////////// Cloning Box<dyn Trait>:

pub trait Unit: DynClone {
    fn leaf(&self) -> Edge<String>;
    fn json(&self) -> String;
}

// macro to generate stuff needed to clone Box<dyn Unit>
dyn_clone::clone_trait_object!(Unit);

// this is how to do it without the macro
pub trait CloneUnit {
    fn clone_unit<'a>(&self) -> Box<dyn Unit>;
}
impl<T> CloneUnit for T
where
    T: Unit + Clone + 'static,
{
    fn clone_unit(&self) -> Box<dyn Unit> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn Unit> {
    fn clone(&self) -> Self {
        self.clone_unit()
    }
}


//////////////// serialize trait objects:
serialize_trait_object!(Unit);
pub trait Unit: DynClone + erased_serde::Serialize {
    fn leaf(&self) -> Edge<String>;
    fn json(&self) -> String;
}

// do it without the macro:
impl Serialize for Box<dyn Unit> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.json())
    }
}