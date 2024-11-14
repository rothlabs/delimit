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
        // let drawings = self.drawings.base()?;
        // if drawings.is_empty() {
        if let Some(main) = displays.first() {
            let view = make_view(main).unwrap();
            let drawing = test::draw_nurbs_surface(&view).await.unwrap();
            // drawing.base().await?;
            self.drawings.clone().write(|x| x.push(drawing)).await?;
        }
        // }
        Ok(())
    }
}

fn make_view(display: &Display) -> Result<View> {
    let port = display.viewport.clone();
    let mech = Mech::new(port.clone())?;
    let view = View::new(mech, port)?;
    Ok(view)
}
