use std::collections::HashMap;
use shape::*;

mod shape;

pub struct Shape {
    points: HashMap<PointId, Point>,
    knots: HashMap<KnotVectorId, KnotVector>,

    // blocks: 
}