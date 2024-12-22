use std::collections::HashSet;
// use std::cell::Cell;
use std::pin::Pin;
// use std::sync::Arc;
// use std::rc::{Rc, self};
use std::future::Future;

pub mod store;
pub mod agent;

pub type Grc<T> = std::rc::Rc<T>;
type Weak<T> = std::rc::Weak<T>;
type Cell<T> = std::cell::RefCell<T>;
type PinFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

pub struct Store<T> {
    pub len: usize,
    free_head: usize,
    data: Vec<store::Entry<T>>,
}

pub struct Agent<T> {
    pub unit: T,
    // out:
}

// pub enum Hub<T> {
//     Base(T),
//     // Agent
// }

pub type Result<T> = std::result::Result<T, Error>;

/// Graph Error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Generic(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}


// This section is for a database crate

pub struct GenIndex {
    gen: u32,
    index: usize,
}

// We can find a different name besides "Hub" 
// Value and Index should be the only Variants
pub enum Hub<T> {
    Value(T),
    Index(GenIndex),
}

pub struct Storage<T> {
    // I need to replace Vec<Option<T>> with Vec<Option<Entry<T>>> where entry holds the generation number
    data: Vec<Option<T>>,
}

impl<T> Storage<T> {
    fn get(&self, gen_index: &GenIndex) -> Option<&T> {
        // some check to make sure it is same generation, if not, return None
        self.data[gen_index.index].as_ref()
    }
}

pub trait GetSet<T> {
    fn get(&self, index: &GenIndex) -> Option<&T>;
    fn set(&self, index: GenIndex) {}
}

// This section is domane-specific, utilizing the database crate

struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

struct Matrix {
    // bunch of transform matrix numbers
}

struct Knot(f64);

struct Thickness(f64);

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

struct Curve {
    points: Vec<Hub<Vector>>,
    knots: Vec<Hub<Knot>>,
}

struct Surface {
    curves: Vec<Hub<Curve>>,
    knots: Vec<Hub<Knot>>,
}

enum Shape {
    Point(Vector),
    Curve(Curve),
    Surface(Surface),
}

enum Style {
    Color(Color),
    Thickness(Thickness)
}

struct View {
    shape: Hub<Shape>,
    style: HashSet<Hub<Style>>,
}

// Instead of having a Relative variant in Hub, we have a relative type that describes 
// a thing to be relative to and how we relative to it.
// In this case, Transformed represents whatever T is, but after it has been transformed by the matrix,
// T can be a point, curve, or whatever where it makes semantic sense to apply a transform matrix to 
// get a new one relative to the relative_to field. Exactly how and when that is achieve is out of scope of the database 
// and could be interpreted and computed in a CPU computational graph and/or GPU commands
struct Transformed<T> {
    relative_to: Hub<T>,
    matrix: Hub<Matrix>,
}

// Just a random example of a group that the user has ultimately specified. 
// see how it can be in the database
struct Selection<T> {
    targets: Vec<Hub<T>>,
    // might need some additional info about a selection:
    some_meta_stuff_or_constant: u8,
}


struct Database {
    vectors: Storage<Vector>,
    knots: Storage<Knot>,
    shapes: Storage<Shape>,
    styles: Storage<Style>,
    views: Storage<View>,
    // showing how it in some cases there is no Storage<T>. 
    // There might only be one place to see all shapes that are selected by the user
    selected_shapes: Selection<Shape>,
    // here the focus is on how Transformed<Shape> represents a shape seperate from the one it points to
    // but it is relative to it by the Transformed::matrix field
    transformed_shapes: Storage<Transformed<Shape>>,
}

// this type of repetitive code is usually generated with a macro
// like how we use derive(Clone)
impl GetSet<Vector> for Database {
    fn get(&self, index: &GenIndex) -> Option<&Vector> {
        self.vectors.get(index)
    }
}
impl GetSet<Shape> for Database {
    fn get(&self, index: &GenIndex) -> Option<&Shape> {
        self.shapes.get(index)
    }
}
impl GetSet<Style> for Database {
    fn get(&self, index: &GenIndex) -> Option<&Style> {
        self.styles.get(index)
    }
}
// implement GetSet for other types in the Database





// // "Vec of Structs" and "Struct of Vecs" are equivalent in the sense that they both take
// // the same amount of memory and both represant exactly the same possible states
// // The speaker in https://youtu.be/aKLntZcp27M?si=PyJVXphxpVlFp-w5&t=1619 suggests that it is better 
// // to use "Struct of Vecs" because you don't need to have a bunch of different structs for different 
// // types. But the same problem remains where some entities do not use a component and will be set to None always.
// // "archetypes" sorta help with this but then you are back to making many structs that are equivalent to Vec of Structs.

// // The proper solution is to use a combination of structs and enums in a way where you don't type out the same fields
// // over and over and you also don't end up with a bunch of Options that are always None

// // Vector of Structs:

// // Represents one colored shape.
// // the databased would hold colored_shapes: Vec<Option<ColoredShape>>
// struct ColoredShape {
//     shape: Option<Shape>,
//     color: Option<Color>,
// }

// // Struct of Vectors:

// // Represents many colored shapes
// struct ColoredShapesTable {
//     entites: Vec<Option<GenIndex>>,
//     shapes: Vec<Option<Shape>>,
//     colors: Vec<Option<Color>>,
// }


// // In the code below, I am using Vec of Structs because it is more natural to think about but with one critical 
// // difference: I removed the options from the struct. Notice I don't have the problem of ColoredPoint, ColoredCurve, etc 
// // because Shape is an enum of different primitives like points, curves, etc. 

// struct ColoredShapeSimple {
//     shape: Shape,
//     color: Color,
// }


// struct GenIndex {
//     gen: u32,
//     index: usize,
// }

// // We can find a different name besides "Hub" 
// // Value and Index should be the only Variants
// enum Hub<T> {
//     Value(T),
//     Index(GenIndex),
// }

// struct Storage<T> {
//     data: Vec<Option<T>>,
// }


// // This section is domane-specific, utilizing the database crate

// struct Vector {
//     x: f64,
//     y: f64,
//     z: f64,
// }

// struct Matrix {
//     // bunch of transform matrix numbers
// }

// enum Shape {
//     Point(Vector),
//     Curve(Curve),
//     // ...
// }

// // Instead of having a Relative variant in Hub, we have a relative type that describes 
// // a thing to be relative to and in what way are we relative to it? 
// // in this case, Transformed represents whatever T is, but after it has been transformed by the matrix,
// // T can be a point, curve, or whatever where it makes semantic sense to apply a transform matrix to 
// // get a new one relative to the relative_to field. Exactly how and when that is achieve is out of scope of the database 
// // and could be interpreted and computed in a CPU computational graph and/or GPU commands
// struct Transformed<T> {
//     relative_to: Hub<T>,
//     matrix: Hub<Matrix>,
// }

// struct Knot(f64);

// struct Thickness(f64);

// struct Color {
//     r: u8,
//     g: u8,
//     b: u8,
// }

// // represents one curve
// struct Curve {
//     points: Vec<Hub<Vector>>,
//     knots: Vec<Hub<Knot>>,
// }

// // Just a random example of a group that the user has ultimately specified. 
// // see how it can be in the database
// struct Selection<T> {
//     curve: Vec<Hub<T>>,
//     // might need some additional info about this selection:
//     some_meta_stuff_or_constant: u8,
// }

// struct Surface {
//     curves: Vec<Hub<Curve>>,
//     knots: Vec<Hub<Knot>>,
// }

// struct Database {
//     points: Storage<Vector>,
//     knots: Storage<Knot>,
//     thickness: Storage<Thickness>,
//     curves: Storage<Curve>,
//     surfaces: Storage<Surface>,
//     // showing how it in some cases there is no Storage<T>. 
//     // There might only be one place to see all shapes that are selected by the user
//     selected_shapes: Selection<Shape>,
//     // here the focus is on how Transformed<Shape> represents a shape seperate from the one it points to
//     // but it is relative to it by the Transformed::matrix field
//     transformed_shapes: Storage<Transformed<Shape>>,
// }

// // // Just a random example of a group that the user has ultimately specified. 
// // // There might be a Group<T> or Selection<T> that are used for different types in the Database
// // struct CurveGroup {
// //     curve: Vec<Hub<Curve>>,
// //     thickness: Hub<Thickness>,
// //     color: Hub<Color>,
// // }