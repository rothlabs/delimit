use super::*;

struct Span {
    rule: Rule,
    hedge: Hedge,
}

enum Rule {
    Basis(Basis)
}

struct Basis {
    order: u8,
    hedge: Hedge,
}

#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(into, strip_option))]
pub struct Nurbs {
    pub span: Hedge,
}