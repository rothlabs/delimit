use super::*;

#[derive(Builder, BuildGate, Back, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into, strip_option))]
pub struct Node {
    #[back(skip)]
    // #[builder(default)]
    target: Target,
    codec: Hub<Vec<Step>>,
}

impl Solve for Node {
    type Base = Command;
    async fn solve(&self) -> node::Result<Self::Base> {
        let target = self.target.clone();
        let steps = self.codec.base().await?;
        let pass = Command::Render(render::Pass { steps, target });
        Ok(pass.into())
    }
}

// #[derive(Builder, Gate, Back, Debug)]
// #[builder(pattern = "owned")]
// #[builder(setter(into, strip_option))]
// pub struct Node {
//     #[builder(default, setter(each(name = "part", into)))]
//     parts: Vec<Hub<Vec<Step>>>,
// }

// impl Solve for Node {
//     type Base = crate::Command;
//     async fn solve(&self) -> graph::Result<Hub<Self::Base>> {
//         let mut entries = vec![];
//         for part in &self.parts {
//             entries.extend(part.base().await?);
//         }
//         let pass = crate::Command::Render(render::Pass { steps: entries });
//         Ok(pass.into())
//     }
// }
