use super::*;

type CommandTransfer = Hub<Transfer<Grc<Vec<gpu::flat::Command>>>>;

#[derive(Debug, Gate, Back)]
pub struct App {
    pub displays: Leaf<Vec<Display>>,
    pub command_transfers: Leaf<Vec<CommandTransfer>>,
}

impl App {
    pub fn new(displays: Leaf<Vec<Display>>) -> Self {
        Self {
            displays,
            command_transfers: Leaf::default(),
        }
    }
}

impl Act for App {
    async fn act(&self) -> node::Action {
        let displays = self.displays.base()?;
        if let Some(display) = displays.last() {
            let port = &display.viewport;
            let mech = Mech::new(&port.gpu);
            let medium = mech.medium(&port.targets);
            let view = medium.view(port.size.clone());
            let action = tests::draw_nurbs_surface(&view)?;
            let commands = gpu::command::Sort::new(vec![action]).hub();
            let transfer = commands.transfer(&port.commands);
            transfer.depend().await?;
            self.command_transfers.write(|x| x.push(transfer)).await?;
        }
        acted()
    }
}

// fn mech_view(display: &Display) -> Result<View> {
//     let port = display.viewport.clone();
//     let mech = Mech::new(port.clone())?;
//     let view = View::new(mech, port)?;
//     Ok(view)
// }

// let view = mech_view(main)?;

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
