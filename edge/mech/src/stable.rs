// use std::ops::Sub;

use std::ops::Sub;

use super::*;
use generic_array::{ArrayLength, GenericArray};
use typenum::{U1, Sub1};

mod shape;

pub struct Shape<T, D> 
where 
    D: ArrayLength,// + Sub<U1>,
    // <D as Sub<U1>>::Output: ArrayLength,
    // U2: Mul<N>,
    // Sub1<D>: ArrayLength,
{
    pub bounds: Vec<Grc<Shape<T,   U1   >>>,
    pub kind: shape::Kind<T, D>,
}

pub struct Vector<T, D: ArrayLength> {
    pub id: u32,
    pub data: GenericArray<T, D>
}

