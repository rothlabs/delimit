pub mod curve;

/// Vector of dimension K + 2. 
/// K is the number of axes required to describe 
/// orientation, useful revolutions and intersections
/// https://en.wikipedia.org/wiki/Exterior_algebra
/// https://en.wikipedia.org/wiki/Hodge_star_operator
/// https://en.wikipedia.org/wiki/Axis-angle_representation
#[derive(Debug)]
pub struct Vector<T, const K: usize> {
    pub x: T,
    pub y: T,
    pub z: [T; K],
}

pub enum Curve<T, const K: usize> {
    Differ(curve::Differ<T, K>),
    Spline(curve::Spline<T, K>),
}