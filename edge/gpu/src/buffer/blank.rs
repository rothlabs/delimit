use super::*;

#[derive(Builder, Back, Gate, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Blank {
    #[back(skip)]
    core: Core,
    #[builder(setter(each(name = "root")))]
    roots: Vec<BlankRoot>,
    // root: Hub<Grc<Buffer>>,
    // #[builder(default, setter(each(name = "mul", into)))]
    // muls: Vec<Hub<u32>>,
    // #[builder(default, setter(each(name = "div", into)))]
    // divs: Vec<Hub<u32>>,
    #[back(skip)]
    #[builder(default = "BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST")]
    usage: BufferUsages,
}

impl Solve for Blank {
    type Base = Grc<Buffer>;
    async fn solve(&self) -> graph::Result<Hub<Grc<Buffer>>> {
        let mut total: u64 = 0;
        for root in &self.roots {
            let mut size = root.buffer.base().await?.size();
            for mul in &root.muls {
                size *= mul.base().await? as u64;
            }
            for div in &root.divs {
                size /= div.base().await? as u64;
            }
            total += size;
        }
        Ok(self.core.buffer(total).usage(self.usage).make()?.into())
    }
}

impl BlankBuilder {
    pub fn map_read(self) -> graph::Result<Hub<Grc<Buffer>>> {
        self.usage(BufferUsages::MAP_READ | BufferUsages::COPY_DST)
            .hub()
    }
}

#[derive(Builder, Debug)]
#[builder(build_fn(error = "graph::Error"))]
pub struct BlankRoot {
    buffer: Hub<Grc<Buffer>>,
    #[builder(default, setter(each(name = "mul", into)))]
    muls: Vec<Hub<u32>>,
    #[builder(default, setter(each(name = "div", into)))]
    divs: Vec<Hub<u32>>,
}

impl Backed for BlankRoot {
    fn backed(&self, back: &Back) -> graph::Result<Self>
        where
            Self: Sized {
        Ok(Self {
            buffer: self.buffer.clone(),
            muls: self.muls.backed(back)?,
            divs: self.divs.backed(back)?,
        })
    }
}

// impl BackIt for Root {
//     fn back(&mut self, back: &Back) -> graph::Result<()> {
//         self.muls.back(back)?;
//         self.divs.back(back)
//     }
// }