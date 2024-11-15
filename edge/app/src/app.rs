use super::*;

#[derive(Builder, Gate, Back, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct App {
    displays: Leaf<Vec<Display>>,
    #[builder(default)]
    drawings: Leaf<Vec<Hub<Mutation>>>,
}

impl Act for App {
    async fn act(&self) -> graph::Result<()> {
        let displays = self.displays.base()?;
        if let Some(main) = displays.first() {
            let view = mech_view(main).unwrap();
            let entries = test::draw_nurbs_surface(&view).await.unwrap().base().await?;
            let passes = vec![Pass::Render(render::Pass { entries })];
            view.port.passes(passes).unwrap();
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
