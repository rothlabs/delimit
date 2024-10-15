use super::*;

pub struct Nurbs {
    gpu: Gpu,
}

impl Nurbs {
    async fn dispatcher(&self) -> gpu::Result<Hub<()>> {
        let shader = self.gpu.shader(include_wgsl!("shader/nurbs.wgsl"));
        let order = 3;
        let count = 64;
        let size = 4 * count as u64;
        let config = self.gpu.buffer_uniform(&[order, count]);
        let knots = self.gpu.buffer(size).storage_copy()?;
        let weights = self.gpu.buffer(size).storage_copy()?;
        let basis = self.gpu.buffer(size).storage_copy()?;
        let stage = self.gpu.buffer(size).map_read()?;
        let config_entry = self.gpu.uniform().entry(0)?.compute()?;
        let knots_entry = self.gpu.storage(false).entry(1)?.compute()?;
        let weights_entry = self.gpu.storage(false).entry(2)?.compute()?;
        let basis_entry = self.gpu.storage(false).entry(3)?.compute()?;
        let bind_layout = self
            .gpu
            .bind_layout(&[config_entry, knots_entry, weights_entry, basis_entry])
            .make()?;
        let bind = self
            .gpu
            .bind()
            .layout(&bind_layout)
            .entry(0, &config)
            .entry(1, &knots)
            .entry(2, &weights)
            .entry(3, &basis)
            .make()?;
        let pipe_layout = self.gpu.pipe_layout(&[&bind_layout]).make()?;
        let pipe = shader.compute("main").layout(&pipe_layout).make()?;
        let dispatcher = self
            .gpu
            .dispatcher()
            .pipe(pipe)
            .bind(bind)
            .count(count)
            .stage((basis.inner(), stage.inner()))
            .hub();
        dispatcher.map_err(|err| gpu::Error::Any(anyhow!("{err}")))
    }
}

impl Solve for Nurbs {
    type Base = ();
    async fn solve(&self) -> graph::Result<Hub<()>> {
        self.dispatcher()
            .await
            .map_err(|err| graph::Error::Any(anyhow!("{err}")))
    }
}

// impl Nurbs {
//     async fn dispatcher(&self) -> gpu::Result<Hub<()>> {
//         let shader = self.gpu.shader(include_wgsl!("shader/nurbs.wgsl"));
//         let order = 3;
//         let count = 64;
//         let size = 4 * count as u64;
//         let config = self.gpu.buffer_uniform(&[order, count]);
//         let knots = self.gpu.buffer(size).storage_copy()?;
//         let weights = self.gpu.buffer(size).storage_copy()?;
//         let basis = self.gpu.buffer(size).storage_copy()?;
//         let stage = self.gpu.buffer(size).map_read()?;
//         let config_entry = self.gpu.uniform().entry(0)?.compute()?;
//         let knots_entry = self.gpu.storage(false).entry(1)?.compute()?;
//         let weights_entry = self.gpu.storage(false).entry(2)?.compute()?;
//         let basis_entry = self.gpu.storage(false).entry(3)?.compute()?;
//         let bind_layout = self.gpu.bind_layout(&[config_entry, knots_entry, weights_entry, basis_entry]).make()?;
//         let bind = self.gpu
//             .bind()
//             .layout(&bind_layout)
//             .entry(0, &config)
//             .entry(1, &knots)
//             .entry(2, &weights)
//             .entry(3, &basis)
//             .make()?;
//         let pipe_layout = self.gpu.pipe_layout(&[&bind_layout]).make()?;
//         let pipe = shader.compute("main").layout(&pipe_layout).make()?;
//         let dispatcher = self.gpu
//             .dispatcher()
//             .pipe(pipe)
//             .bind(bind)
//             .count(count)
//             .stage((basis.inner(), stage.inner()))
//             .hub();
//         dispatcher.map_err(|err| gpu::Error::Any(anyhow!("{err}")))
//     }
// }
