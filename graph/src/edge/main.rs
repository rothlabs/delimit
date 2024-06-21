use std::{
    any::Any, sync::{Arc, RwLock, Weak}
};

use serde::Serialize;

use crate::{Meta, Node};

use super::{Edge, NO_POISON, ROOT};

pub struct Main<R, S> {
    pub stem: Arc<RwLock<S>>,
    pub root: Option<Weak<RwLock<R>>>,
    pub meta: Meta,
}

impl<R, S> Edge for Main<R, S>
where
    R: Node,
    S: Node,
{
    type R = R;
    type S = S;
    fn new(unit: S::U) -> Self {
        Self {
            stem: Arc::new(RwLock::new(S::new(unit))),
            root: None,
            meta: Meta::new(),
        }
    }
    fn solve(&self, task: S::T) -> S::L { // TODO: rename to load?
        let mut node = self.stem.write().expect(NO_POISON);
        node.solve(task)
    }
    fn unit(&self) -> S::U {
        let node = self.stem.read().expect(NO_POISON);
        node.unit().clone()
    }
    fn read<F: FnOnce(&S::U)>(&self, read: F) {
        let node = self.stem.read().expect(NO_POISON);
        read(&node.unit()); 
    }
    fn write<F: FnOnce(&mut S::U) -> R::V>(&self, write: F) {
        let mut stem = self.stem.write().expect(NO_POISON);
        let variance = write(&mut stem.unit_mut());
        if let Some(weak) = &self.root {
            let arc = weak.upgrade().expect(ROOT);
            let mut root = arc.write().expect(NO_POISON);
            root.react(variance);
        }
    }
    fn root(&mut self, stem: &Arc<RwLock<R>>) {
        self.root = Some(Arc::downgrade(stem));
    }
}

impl<R, S> Clone for Main<R, S> {
    fn clone(&self) -> Self {
        Self {
            stem: self.stem.clone(),
            root: self.root.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<R, St>  Serialize for Main<R, St>  {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.meta.serialize(serializer)
    }
}