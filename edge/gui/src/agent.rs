use super::*;

pub struct Agent {
    pub gfx: Gfx,
}

impl Agent {
    pub async fn run(self, mut queue: broadcast::Receiver<Grc<Window>>) {
        loop {
            if let Err(err) = self.step(&mut queue).await {
                println!("gui::agent::Error: {:?}", err);
            }
        }
    }
    async fn step(&self, queue: &mut broadcast::Receiver<Grc<Window>>) -> Result<()> {
        let window = queue.recv().await?;
        self.gfx.display(window).await?;
        Ok(())
    }
}
