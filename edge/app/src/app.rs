use super::*;

#[derive(Builder, Gate, Back, Debug)]
#[builder(setter(into), pattern = "owned")]
pub struct App {
    displays: Leaf<Vec<Display>>,
    #[builder(default)]
    drawings: Leaf<Vec<Hub<Mutation>>>,
}

impl Act for App {
    async fn act(&self) -> graph::Result<()> {
        let displays = self.displays.base()?;

        println!("displays changed");
        Ok(())
    }
}