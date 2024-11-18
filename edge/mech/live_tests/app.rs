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
            let steps = tests::draw_nurbs_surface(&view).await?;
            let pass = port.pass(steps).hub()?;
            let commands = star::vector().field(pass).hub()?;
            let transfer = commands.transfer(&port.commands)?;
            transfer.depend().await?;
            self.command_writers.write(|x| x.push(transfer)).await?;
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


// impl Act for App {
//     async fn act(&self) -> node::Action {
//         self.action().await
//         // match self.action().await {
//         //     Ok(_) => acted(),
//         //     Err(err) => panic!("App Error: {err}")
//         // }
//     }
// }

// impl App {
//     async fn action(&self) -> node::Action {
//         let displays = self.displays.base()?;
//         if let Some(main) = displays.last() {
//             let port = &main.viewport;
//             let view = mech_view(main)?;
//             let steps = tests::draw_nurbs_surface(&view).await?;
//             let pass = port.pass(steps).hub()?;
//             let commands = star::vector().field(pass).hub()?;
//             let transfer = commands.transfer(&port.commands)?;
//             transfer.depend().await?;
//             self.command_writers.write(|x| x.push(transfer)).await?;
//         }
//         acted()
//     }
// }