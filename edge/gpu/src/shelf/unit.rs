use super::*;

#[derive(Debug, Gate, Back)]
pub struct Grant {
    pub size: Hub<u32>,
    #[back(skip)]
    pub buffer: Leaf<Grc<Buffer>>,
    #[back(skip)]
    pub chunks: Leaf<Vec<Chunk>>,
}

impl Solve for Grant {
    type Base = u32;
    async fn solve(&self) -> node::Result<u32> {
        let max = (self.buffer.base()?.size() / 4) as u32;
        let size = self.size.base().await?;
        let mut i = 0;
        let mut start = 0;
        let mut end = size;
        let grant = self.chunks.write_passive(|chunks| {
            while let Some(chunk) = chunks.get(i) {
                if end < chunk.start {
                    break;
                } else {
                    start = chunk.end;
                    end = chunk.end + size;
                }
                i += 1;
            }
            if end > max {
                // TODO: make bigger buffer and copy to it
                panic!("buffer full!")
            }
            let grant = Leaf::new(start);
            let chunk = Chunk {
                // grant: grant.clone(),
                start,
                end,
            };
            chunks.insert(i, chunk);
            grant
        })?;
        Ok(grant.into())
    }
}
