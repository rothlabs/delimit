// use super::*;

use super::Number;

pub struct Vector3<N> {
    pub x: N,
    pub y: N,
    pub z: N,
}

impl<N> Vector3<N> 
where 
    N: Number
{
    pub fn new(vector: &[N]) -> Self {
        Self {
            x: vector[0],
            y: vector[1],
            z: vector[2],
        }
    }
}

impl<N> std::ops::Add for &Vector3<N> 
where 
    N: Number
{
    type Output = Vector3<N>;
    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}