use super::*;

pub struct Plot<N> {
    pub point: Vector3<N>,
    pub velocity_u: Vector3<N>,
    pub velocity_v: Vector3<N>,
}