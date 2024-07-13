use crate::*;

pub trait ToSole<L> {
    fn leaf(&self) -> Sole<L>;
}

impl ToSole<String> for str {
    fn leaf(&self) -> Sole<String> {
        Sole::new(self.to_owned())
    }
}

pub trait IntoSole<L> {
    fn into_sole(self) -> Sole<L>;
}

impl<L> IntoSole<L> for L {
    fn into_sole(self) -> Sole<L> {
        Sole::new(self)
    }
}
