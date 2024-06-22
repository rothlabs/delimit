use crate::node;

use super::Respond;

pub struct Responder<V>(Box<dyn Respond<Root = node::Responder<V>>>);

impl<V> Respond for Responder<V> {
    type Root = node::Responder<V>;
    fn respond(&self, memo: V) {
        self.0.respond(memo);
    }
}