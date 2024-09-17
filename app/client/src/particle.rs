// use web_sys::js_sys::Math::random;

use web_sys::js_sys::Math::random;

use crate::prelude::*;
use crate::log;

impl Sim {
    pub async fn particles(&self, tick: impl Into<Hub<i32>>) -> Result<ParticlesBuilder> {
        let tick = tick.into();
        let vert = self.gpu.vertex_shader(PARTICLES)?;
        let frag = self.gpu.fragment_shader(PARTICLES_FRAG)?;
        let prog = self.gpu.program(vert, frag)?.out("out_pos").out("out_vel").make()?;
        // #[rustfmt::skip]
        // let mut points: Vec<f32> = vec![
        //     // pos        vel
        //     0., -1.0,   0.01, 0.01,
        //     0., -0.5,   -0.01, 0.01,
        //     0.,  0.0,   0.01, -0.01,
        //     0.,  0.5,   -0.01, -0.01,
        //     0.,  1.0,   0.01, 0.01,
        // ];
        let count = 20;
        let mut points = vec![];
        for _ in 0..count {
            points.push(random_float());
            points.push(random_float());
            points.push(random_float() * 0.01);
            points.push(random_float() * 0.01);
        }
        let buffer0 = self.gpu.buffer()?;
        buffer0.writer().array(points).make()?.act().await?;
        let pos0 = buffer0.attribute().size(2).stride(16).make()?;
        let vel0 = buffer0.attribute().size(2).stride(16).offset(8).index(1).make()?;
        let vao0 = self.gpu.vao()?.attributes(vec![pos0, vel0]).make()?;
        let tfo0 = self.gpu.tfo()?.buffer(buffer0).make()?;
        let buffer1 = self.gpu.buffer()?;
        buffer1.writer().array(count * 16).make()?.act().await?;
        let pos1 = buffer1.attribute().size(2).stride(16).make()?;
        let vel1 = buffer1.attribute().size(2).stride(16).offset(8).index(1).make()?;
        let vao1 = self.gpu.vao()?.attributes(vec![pos1, vel1]).make()?;
        let tfo1 = self.gpu.tfo()?.buffer(buffer1).make()?;
        let draw0 = self.gpu
            .draw_arrays(prog.clone())
            .mode(WGLRC::POINTS)
            .vao(vao0)
            .tfo(tfo1)
            .count(count)
            .tick(&tick)
            .make()?;
        let draw1 = self.gpu
            .draw_arrays(prog)
            .mode(WGLRC::POINTS)
            .vao(vao1)
            .tfo(tfo0)
            .count(count)
            .tick(tick)
            .step(1)
            .make()?;
        let particles = ParticlesBuilder::default().draw0(draw0).draw1(draw1);
        Ok(particles)
    }
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into), build_fn(error = "graph::Error"))]
pub struct Particles {
    // gpu: Gpu,
    // tick: Hub<i32>,
    draw0: Node<DrawArrays>,
    draw1: Node<DrawArrays>,
}

impl ParticlesBuilder {
    pub fn make(self) -> Result<Node<Particles>> {
        let mut part = self.build()?;
        Node::make(|back|{
            // part.tick = part.tick.backed(back)?;
            Ok(part)
        })
    }
}

impl Act for Particles {
    async fn act(&self) -> Result<()> {
        // console_log!("tick: {}", self.tick.base().await?);
        // if self.tick.base().await? % 2 == 0 {
            self.draw0.act().await?;
        // } else {
            self.draw1.act().await
        // }
    }
}

impl Reckon for Particles {}

pub const PARTICLES: &str = r"#version 300 es
layout(location = 0) in vec2 pos;
layout(location = 1) in vec2 vel;

out vec2 out_pos;
out vec2 out_vel;

void main() {
    out_pos = pos + vel;
    out_pos = mod(out_pos + 1.0, 2.0) - 1.0;
    out_vel = vel;
    gl_Position = vec4(out_pos.x, out_pos.y, 0., 1.);
    gl_PointSize = 6.;
}";

pub const PARTICLES_FRAG: &str = r"#version 300 es
precision highp float;
layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(0.2, 0.2, 0.8, 1);
}";