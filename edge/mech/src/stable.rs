use super::*;
use generic_array::{ArrayLength, GenericArray};
use typenum::{op, Cmp, Integer, IsGreater, IsGreaterOrEqual, Max, Min, Sub1, Unsigned, N1, P1, U0, U1};
use std::{marker::PhantomData, ops::Sub};

// mod shape;


// impl method to get flat buffer version only if traits follow

pub struct Shape<T, B> {
    pub block: T,
    pub bound: B,
}

pub struct Flow {
    pub form: usize,
    pub warp: usize,
}

pub struct Extrude<T> {
    pub block: T,
}

pub struct Vector<T, const D: usize> {
    pub id: u32,
    pub data: [T; D],
}

pub trait Rank0 {}
pub trait Rank1 {}
pub trait Rank2 {}

impl<T, const D: usize> Rank0 for Vector<T, D> {}
impl<T> Rank1 for Extrude<T> where T: Rank0 {}
impl<T> Rank2 for Extrude<T> where T: Rank1 {}

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

