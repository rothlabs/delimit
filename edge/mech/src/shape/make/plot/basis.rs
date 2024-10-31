use super::*;

pub struct Grid<'a> {
    pub basis: &'a Basis<'a>,
    pub count: &'a Hub<u32>,
}