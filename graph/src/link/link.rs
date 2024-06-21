use std::sync::{Arc, RwLock};

use crate::{edge, node, NO_POISON};

use super::{Read, Write};

pub struct Link<B> {
    body: Arc<RwLock<B>>,
}

impl<B> Read for Link<B>
where
    B: edge::Read,
{
    type Edge = B;
    fn read<F: FnOnce(&<B::Stem as node::Read>::Unit)>(&self, read: F) {
        let body = self.body.read().expect(NO_POISON);
        body.read(read);
    }
}

impl<B> Write for Link<B>
where
    B: edge::Write,
{
    type Edge = B;
    fn write<F: FnOnce(&mut <B::Stem as node::Write>::Unit)>(&self, write: F) {
        let body = self.body.read().expect(NO_POISON);
        body.write(write);
    }
}
