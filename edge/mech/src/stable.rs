use generic_array::{arr, ArrayLength, GenericArray};
use std::ops::Mul;
use typenum::{op, U2};

fn bean_doubler<N>(beans: GenericArray<i32, N>) -> GenericArray<i32, op!(U2 * N)>
where
    N: ArrayLength,
    U2: Mul<N>,
    <U2 as Mul<N>>::Output: ArrayLength,
{
    let mut doubled = GenericArray::default();
    for i in 0..N::USIZE {
        doubled[2 * i] = beans[i];
        doubled[2 * i + 1] = beans[i];
    }
    return doubled;
}

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

