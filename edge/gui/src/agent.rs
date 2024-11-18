use super::*;

pub struct Agent {
    pub gfx: Gfx,
}

impl Agent {
    pub async fn run(self, mut queue: broadcast::Receiver<Post>) {
        loop {
            if let Err(err) = self.step(&mut queue).await {
                println!("gui::agent::Error: {:?}", err);
            }
        }
    }
    async fn step(&self, queue: &mut broadcast::Receiver<Post>) -> Result<()> {
        let post = queue.recv().await?;
        #[allow(irrefutable_let_patterns)]
        if let Post::Window(window) = post {
            self.gfx.display(window).await?;
        }
        Ok(())
    }
}
