use super::*;

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
    pub fn interpolate(&self, rhs: &Self, u: N) -> Self {
        &(self * (1. as N - u)) + &(rhs * u)
    }
    //pub fn zero() -> 
}

impl<N: Number> Add for &Vector3<N> {
    type Output = Vector3<N>;
    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<N: Number> Sub for &Vector3<N> {
    type Output = Vector3<N>;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<N: Number> Mul<N> for &Vector3<N> {
    type Output = Vector3<N>;
    fn mul(self, rhs: N) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}