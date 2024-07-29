use super::*;

pub enum Part {
    Plain(plain::Part),
    Html(html::Part),
    Base(graph::part::Part),
}
