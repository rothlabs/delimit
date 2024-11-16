use star::MakeTransfer;

use super::*;

#[derive(Builder, BuildGate, Back, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct App {
    displays: Leaf<Vec<Display>>,
    #[builder(default)]
    command_writers: Leaf<Vec<Hub<()>>>,
}

impl Act for App {
    async fn act(&self) -> node::Action {
        let displays = self.displays.base()?;
        if let Some(main) = displays.last() {
            let port = &main.viewport;
            let view = mech_view(main)?;
            let steps = test::draw_nurbs_surface(&view).await?;
            let command = port.pass(steps).hub()?;
            let commands = star::vector().field(command).hub()?;
            let transfer = commands.transfer(&port.commands)?;
            transfer.depend().await?;
            self.command_writers.write(|x| x.push(transfer)).await?;
            println!("new window");
        }
        acted()
    }
}

fn mech_view(display: &Display) -> Result<View> {
    let port = display.viewport.clone();
    let mech = Mech::new(port.clone())?;
    let view = View::new(mech, port)?;
    Ok(view)
}


    // let writer = star::Transfer{source: commands, target: &port.commands}.hub()?;
    // let writer = star::transfer(commands, &port.commands)?;