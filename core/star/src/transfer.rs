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

#[derive(Gate, Back, Debug)]
pub struct Transfer<T> {
    pub source: Hub<T>,
    pub target: Leaf<T>,
}

impl<T> Act for Transfer<T>
where
    T: 'static + Gather,
{
    async fn act(&self) -> node::Action {
        let value = self.source.base().await?;
        self.target.write(|x| *x = value).await?;
        acted()
    }
}

pub trait MakeTransfer<T> {
    fn transfer(&self, target: impl Into<Leaf<T>>) -> Result<Hub<()>>;
}

impl<T: 'static + Gather> MakeTransfer<T> for Hub<T> {
    fn transfer(&self, target: impl Into<Leaf<T>>) -> Result<Hub<()>> {
        Transfer {
            source: self.clone(),
            target: target.into(),
        }
        .hub()
    }
}
