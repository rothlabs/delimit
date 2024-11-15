use super::*;

#[derive(Builder, Gate, Back, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct App {
    displays: Leaf<Vec<Display>>,
    #[builder(default)]
    command_writer: Leaf<Hub<()>>,
}

impl Act for App {
    async fn act(&self) -> graph::Result<()> {
        println!("app act");
        let displays = self.displays.base()?;
        if let Some(main) = displays.first() {
            let port = &main.viewport;
            let view = mech_view(main).unwrap();
            let steps = test::draw_nurbs_surface(&view).await.unwrap();
            let command = port.pass(steps).hub()?;
            let commands = star::vector().field(command).hub().unwrap();
            let writer = star::writer(commands).target(&port.commands).hub()?;
            writer.base().await.unwrap();
            self.command_writer.write(|x| *x = writer).await.unwrap();
            println!("set viewport passes");
        }
        Ok(())
    }
}

fn mech_view(display: &Display) -> Result<View> {
    let port = display.viewport.clone();
    let mech = Mech::new(port.clone())?;
    let view = View::new(mech, port)?;
    Ok(view)
}
