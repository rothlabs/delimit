use super::*;

#[derive(Builder, Back, BuildGate, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Blank {
    #[back(skip)]
    core: Core,
    size: Hub<u32>,
    #[back(skip)]
    #[builder(default = "BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST")]
    usage: BufferUsages,
    #[builder(default)]
    #[back(skip)]
    label: String,
}

impl Solve for Blank {
    type Base = Grc<Buffer>;
    async fn solve(&self) -> node::Result<Grc<Buffer>> {
        let size = self.size.base().await?;
        // let label = self.label.base().await?;
        println!("make buffer: {}, {}", self.label, size);
        let buffer = self
            .core
            .buffer(size as u64 * 4)
            .label(&self.label)
            .usage(self.usage)
            .make()?;
        Ok(buffer.into())
    }
}

impl BlankBuilder {
    pub fn map_read(self) -> Result<Hub<Grc<Buffer>>> {
        Ok(self
            .usage(BufferUsages::MAP_READ | BufferUsages::COPY_DST)
            .hub()?)
    }
}

// #[derive(Builder, Debug)]
// #[builder(build_fn(error = "graph::Error"))]
// pub struct BlankRoot {
//     buffer: Hub<Grc<Buffer>>,
//     #[builder(default, setter(each(name = "mul", into)))]
//     muls: Vec<Hub<u32>>,
//     #[builder(default, setter(each(name = "div", into)))]
//     divs: Vec<Hub<u32>>,
// }

// impl Backed for BlankRoot {
//     fn backed(&self, back: &Back) -> graph::Result<Self>
//         where
//             Self: Sized {
//         Ok(Self {
//             buffer: self.buffer.clone(),
//             muls: self.muls.backed(back)?,
//             divs: self.divs.backed(back)?,
//         })
//     }
// }

// root: Hub<Grc<Buffer>>,
// #[builder(default, setter(each(name = "mul", into)))]
// muls: Vec<Hub<u32>>,
// #[builder(default, setter(each(name = "div", into)))]
// divs: Vec<Hub<u32>>,

// impl BackIt for Root {
//     fn back(&mut self, back: &Back) -> graph::Result<()> {
//         self.muls.back(back)?;
//         self.divs.back(back)
//     }
// }
