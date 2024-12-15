use super::*;

pub mod block;

pub struct Shape<T, B> {
    pub block: T,
    pub bound: B,
}

/// Vector of dimension K + 2
/// https://en.wikipedia.org/wiki/Exterior_algebra
/// https://en.wikipedia.org/wiki/Hodge_star_operator
#[derive(Debug)]
pub struct Vector<T, const K: usize> {
    pub id: u32,
    pub x: T,
    pub y: T,
    pub z: [T; K],
}

impl<T, const K: usize> Vector<T, K> {
    pub fn new(x: T, y: T, z: [T; K]) -> Grc<Self> {
        Self {
            id: rand::random(),
            x,
            y,
            z,
        }
        .into()
    }
}

impl<T, const K: usize> Rank<0> for Grc<Vector<T, K>> {}
impl<T, const K: usize> Blade<K> for Grc<Vector<T, K>> {}

// #[derive(Debug)]
// pub struct Extrude<T> {
//     pub block: T,
// }
// impl<T> Rank<1> for Grc<Extrude<T>> where T: Rank<0> {}
// impl<T> Rank<2> for Grc<Extrude<T>> where T: Rank<1> {}
// impl<T, const D: usize> Blade<D> for Grc<Extrude<T>> where T: Blade<D> {}

// impl<T> Flatten for Extrude<T>
// where
//     T: Rank<0> + Blade<1>,
// {
//     fn flat(&self) -> Vec<f32> {
//         vec![1., 2., 3.]
//     }
// }

// #[derive(Debug)]
// pub struct Matrix<T, const D: usize> {
//     pub id: u32,
//     pub data: [[T; D]; D],
// }

// How to place many bounds for many blocks?
// a block cannot have bounds because blocks are made of blocks.
// blocks only get bounds as a Shape
// a block could be many blocks but all of the same rank and dim
// no! a block is just one block. When I do some kind of instancing/pattern, it is no longer a Block!
// So I don't pick a bunch of curves and then extrude into a bunch of surfaces. I extrude each surface
// "at a time" but they all reference the same extrusion vector.
// but what if I make a pattern of 1000 curves first? After extrusion, it becomes 1000 extrusions where
// each extrusion references the same curve and same vector but each have their own pattern value
// no, it is just the pattern that points to one extrusion
// It doesn't make sense to make a spline out of regular pattern blocks.
// so a block is just one block
// so a block gets boundaries as a shape and then patterns can be made of shapes
// a higher level thing could make a pattern where each element has different bounds
// but that gets translated to a 1000 shapes with their own position and bounds

// I could use a BTreeMap or HashMap of Points
// That way all of one type is in one location but not "flat" because they can still be linked by ID
// But then references can be broken because they are pointing with IDs.
// The main graph tech could have worked like that

// pub struct Block<C>
// where
//     C: Sub<U1> + IsGreater<U1>,
//     // <C as Sub<U1>>::Output: IsGreaterOrEqual<U0>
//     // <C as Max<U1>>::Output: Sub<U1>,
//     // <C as Sub<U1>>::Output: Sub<U1>
//     // <U1 as Max<D>>::Output: Unsigned + ArrayLength,
//     // <D as Max<U1>>::Output: Max<U1>,
// {
//     pub sub: Vec<Block<    <<C as Sub<U1>>::Output as IsGreaterOrEqual<U0>>::Output    >>,
//     pub crap: PhantomData<C>
// }

// fn bean_doubler<N>(beans: GenericArray<i32, N>) -> GenericArray<i32, op!(U2 * N)>
// where
//     N: ArrayLength,
//     U2: Mul<N>,
//     <U2 as Mul<N>>::Output: ArrayLength,
// {
//     let mut doubled = GenericArray::default();
//     for i in 0..N::USIZE {
//         doubled[2 * i] = beans[i];
//         doubled[2 * i + 1] = beans[i];
//     }
//     return doubled;
// }

// use super::*;
// use generic_array::{ArrayLength, GenericArray};
// use typenum::{op, IsGreater, Max, Sub1, Unsigned, N1, U0, U1};
// use std::{ops::Sub};

// mod shape;

// pub struct Shape<T, D>
// where
//     D: Unsigned + ArrayLength,
//     <U1 as Max<D>>::Output: Unsigned + ArrayLength,
//     // <D as Max<U1>>::Output: Max<U1>,
// {
//     pub bounds: Vec<Grc<Shape<T,  <U1 as Max<D>>::Output   >>>,
//     pub kind: shape::Kind<T, D>,
// }

// pub struct Vector<T, D: ArrayLength> {
//     pub id: u32,
//     pub data: GenericArray<T, D>
// }

// <<D as Max<U1>>::Output as Sub<U1>>::Output

// pub struct Shape<T, D>
// where
//     D: ArrayLength,
//     N1: Add<D>,
//     op!(N1 + D): ArrayLength,
// {
//     pub bounds: Vec<Grc<Shape<T, op!(N1 + D)>>>,
//     pub kind: shape::Kind<T, D>,
// }

// pub struct Shape2<T> {
//     pub kind: shape::Kind<T, U2>,
// }

// pub struct Shape3<T> {
//     pub bounds: Vec<Grc<Shape2<T>>>,
//     pub kind: shape::Kind<T, U3>,
// }

// // use std::ops::Sub;

// use std::ops::Sub;

// use super::*;
// use generic_array::{ArrayLength, GenericArray};
// use typenum::{U1, Sub1};

// mod shape;

// pub struct Shape<T, D>
// where
//     D: ArrayLength + std::ops::Sub<typenum::UInt<typenum::UTerm, typenum::B1>>,
//     <D as std::ops::Sub<typenum::UInt<typenum::UTerm, typenum::B1>>>::Output: generic_array::ArrayLength,
//     // Sub1<D>: ArrayLength,
// {
//     pub bounds: Vec<Grc<Shape<T,   <D as Sub<U1>>::Output   >>>,
//     pub kind: shape::Kind<T, D>,
// }
