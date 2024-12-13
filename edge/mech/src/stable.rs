use super::*;
use generic_array::{ArrayLength, GenericArray};
use typenum::{op, NonZero, N1};
use std::ops::Add;

mod shape;

pub struct Shape<T, D> 
where 
    D: ArrayLength,
    N1: Add<D>,
    op!(N1 + D): ArrayLength,
{
    pub bounds: Vec<Grc<Shape<T, op!(N1 + D)>>>,
    pub kind: shape::Kind<T, D>,
}

pub struct Vector<T, D: ArrayLength> {
    pub id: u32,
    pub data: GenericArray<T, D>
}

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

