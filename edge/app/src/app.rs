use super::*;

type CommandTransfer = Hub<Transfer<Grc<Vec<Command>>>>;

#[derive(Builder, BuildGate, Back, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct App {
    displays: Leaf<Vec<Display>>,
    #[builder(default)]
    command_writers: Leaf<Vec<CommandTransfer>>,
}

impl Act for App {
    async fn act(&self) -> node::Action {
        let displays = self.displays.base()?;
        if let Some(main) = displays.last() {
            let port = &main.viewport;
            let mech = Mech::new(port)?;
            let view = View::new(mech, port.clone())?;
            let action = test::draw_nurbs_surface(&view).await?;
            let commands = gpu::action::flat().action(action).hub()?;
            let transfer = commands.transfer(&port.commands)?;
            transfer.depend().await?;
            self.command_writers.write(|x| x.push(transfer)).await?;
        }
        acted()
    }
}

// let writer = star::Transfer{source: commands, target: &port.commands}.hub()?;
// let writer = star::transfer(commands, &port.commands)?;
