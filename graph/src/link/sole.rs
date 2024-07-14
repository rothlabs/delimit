use crate::*;

pub trait ToSole<L> {
    fn sole(&self) -> Sole<L>;
}

impl ToSole<String> for str {
    fn sole(&self) -> Sole<String> {
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
