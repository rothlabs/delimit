use std::marker::PhantomData;

use super::*;

// pub fn transfer<T>(source: Hub<T>, target: impl Into<Leaf<T>>) -> Result<Hub<()>>
// where
//     T: 'static + Gather,
// {
//     Transfer {
//         source,
//         target: target.into(),
//     }
//     .hub()
// }

#[derive(Debug, Clone, Default)]
pub struct Transfer<T> {
    kind: PhantomData<T>,
}

#[derive(Gate, Back, Debug)]
pub struct TransferUnit<T> {
    pub source: Hub<T>,
    pub target: Leaf<T>,
}

impl<T> Solve for TransferUnit<T>
where
    T: 'static + Gather,
{
    type Base = Transfer<T>;
    async fn solve(&self) -> node::Result<Transfer<T>> {
        let value = self.source.base().await?;
        self.target.write(|x| *x = value).await?;
        Ok(Transfer{kind: PhantomData}.into())
    }
}

pub trait MakeTransfer<T> {
    fn transfer(&self, target: impl Into<Leaf<T>>) -> Hub<Transfer<T>>;
}

impl<T: 'static + Gather> MakeTransfer<T> for Hub<T> {
    fn transfer(&self, target: impl Into<Leaf<T>>) -> Hub<Transfer<T>> {
        TransferUnit {
            source: self.clone(),
            target: target.into(),
        }
        .hub()
    }
}
