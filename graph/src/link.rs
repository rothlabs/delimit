use std::sync::{Arc, RwLock};

use crate::*;

use serde::Serialize;
pub use sole::{IntoSole, ToSole};

#[cfg(test)]
mod tests;

mod sole;

/// Link to a load.
pub type Sole<L> = Link<edge::Sole<L>>;
pub type Pair<U, L> = Link<edge::Pair<U, L>>;
pub type Solver<L> = Link<dyn SolveShare<L> + Send + Sync>;

/// Points to one edge which in turn points to one node.
/// Units hold links as source of input used to compute output.
pub struct Link<E: ?Sized> {
    edge: Arc<RwLock<E>>,
    meta: Meta,
}

impl<L> Link<dyn SolveShare<L> + Send + Sync> {
    pub fn with_reactor(&self, reactor: &Root) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: edge.solver_with_root(reactor.clone()),
            meta: self.meta.clone(),
        }
    }
}

impl<E> ToLoad for Link<E>
where
    E: ToLoad,
{
    type Load = E::Load;
    fn load(&self) -> Self::Load {
        let edge = self.edge.read().expect(NO_POISON);
        edge.load()
    }
}

impl<E: ?Sized> Clone for Link<E> {
    fn clone(&self) -> Self {
        Self {
            edge: self.edge.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<E> PartialEq for Link<E> {
    fn eq(&self, other: &Self) -> bool {
        Arc::<RwLock<E>>::ptr_eq(&self.edge, &other.edge) && self.meta == other.meta
    }
}

impl<E> FromItem for Link<E>
where
    E: FromItem,
{
    type Item = E::Item;
    fn new(unit: Self::Item) -> Self {
        Self {
            edge: Arc::new(RwLock::new(E::new(unit))),
            meta: Meta::new(),
        }
    }
}

impl<E> WithRoot for Link<E>
where
    E: WithRoot,
{
    type Root = E::Root;
    fn with_root(&self, root: &Self::Root) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: Arc::new(RwLock::new(edge.with_root(root))),
            meta: self.meta.clone(),
        }
    }
}

impl<E> ToRootEdge for Link<E>
where
    E: 'static + EventReact + Send + Sync,
{
    fn reactor(&self) -> RootEdge {
        let edge = self.edge.clone() as Arc<RwLock<dyn EventReact + Send + Sync>>;
        RootEdge {
            item: Arc::downgrade(&edge),
            meta: self.meta.clone(),
        }
    }
}

impl<E: ?Sized> Solve for Link<E>
where
    E: Solve,
{
    type Load = E::Load;
    fn solve(&self) -> Self::Load {
        let edge = self.edge.read().expect(NO_POISON);
        edge.solve()
    }
}

impl<E> Reader for Link<E>
where
    E: 'static + Reader + EventReact + AddRoot<Root = RootEdge> + Send + Sync,
{
    type Unit = E::Unit;
    fn reader<F: FnOnce(&Self::Unit)>(&self, read: F) {
        // TODO: first read and check if it is not added as reactor and then write to do so
        let mut edge = self.edge.write().expect(NO_POISON);
        edge.reader(read);
        let reactor = self.reactor();
        edge.add_root(reactor);
    }
}

impl<E> Writer for Link<E>
where
    E: Writer,
{
    type Unit = E::Unit;
    fn writer<F: FnOnce(&mut Self::Unit)>(&self, write: F) {
        let edge = self.edge.read().expect(NO_POISON);
        edge.writer(write);
    }
}

impl<E> WriterWithPack for Link<E>
where
    E: WriterWithPack,
{
    type Unit = E::Unit;
    fn writer<F: FnOnce(&mut Pack<Self::Unit>)>(&self, write: F) {
        let edge = self.edge.read().expect(NO_POISON);
        edge.writer(write);
    }
}

impl<ER, NR, U, L> ToSolver for Link<Edge<ER, Node<NR, work::Pair<U, L>>>>
where
    ER: 'static + Send + Sync,
    NR: 'static + Send + Sync,
    U: 'static + Send + Sync,
    L: 'static + Send + Sync,
    Edge<ER, Node<NR, work::Pair<U, L>>>: SolveShare<L>,
{
    type Load = L;
    fn solver(&self) -> Solver<L> {
        let edge = self.edge.clone() as Arc<RwLock<dyn SolveShare<L> + Send + Sync>>;
        Solver {
            edge,
            meta: self.meta.clone(),
        }
    }
}

impl<E: ?Sized> Serialize for Link<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.meta.serialize(serializer)
    }
}
