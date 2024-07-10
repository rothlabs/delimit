use super::Respond;

pub struct Responder<M>(Box<dyn Respond<Memo = M>>);

impl<M> Respond for Responder<M> {
    type Memo = M;
    fn respond(&mut self, memo: M) {
        self.0.respond(memo);
    }
}
