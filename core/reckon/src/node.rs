use super::*;

mod apex;

pub struct Apex<T: apex::Solve> {
    unit: T,
    base: Option<T::Base>
}

pub trait Solve {
    type Graph;
    type Base;
    fn solve<'a>(&'a mut self, graph: &'a mut Self::Graph) -> BoxFuture<Result<&Self::Base>>;
}

impl<T: apex::Solve> Solve for Apex<T> {
    type Base = T::Base;
    type Graph = <T as apex::Solve>::Graph;
    fn solve<'a>(&'a mut self, graph: &'a mut Self::Graph) -> BoxFuture<Result<&Self::Base>> {
        Box::new(async move {
            if self.base.is_none() {
                self.base = Some(self.unit.solve(graph).await?);
            }
            Ok(self.base.as_ref().expect("apex base should exist"))
        })
    }
}


// pub trait Engage: Solve {}
// impl<T> Engage for T where T: Solve {}