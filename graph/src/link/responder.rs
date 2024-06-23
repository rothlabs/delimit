use std::sync::{RwLock, Weak};

use crate::{edge, NO_POISON, ROOT};

use super::Respond;

#[derive(Clone)]
pub struct Responder<M>(Weak<RwLock<Box<dyn Respond<Edge = edge::Responder<M>>>>>);

impl<M> Respond for Responder<M> {
    type Edge = edge::Responder<M>;
    fn respond(&self, memo: M) {
        let arc = self.0.upgrade().expect(ROOT);
        let root = arc.write().expect(NO_POISON);
        root.respond(memo);
    }
}
