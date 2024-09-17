use crate::prelude::*;
use crate::log;

impl Sim {
    pub async fn particles(&self, tick: impl Into<Hub<i32>>) -> Result<ParticlesBuilder> {
        let tick = tick.into();
        let vert = self.gpu.vertex_shader(PARTICLES)?;
        let frag = self.gpu.fragment_shader(PARTICLES_FRAG)?;
        let prog = self.gpu.program(vert, frag)?.out("out_pos").out("out_vel").make()?;
        let point_count = 8;
        let mut point_array = vec![];
        for _ in 0..point_count {
            point_array.push(random_float());
            point_array.push(random_float());
            point_array.push(random_float() * 0.002);
            point_array.push(random_float() * 0.002);
        }
        let cp_buff = self.gpu.buffer()?;
        cp_buff.writer().array(point_array).make()?.act().await?;
        let pos0 = cp_buff.attribute().size(2).stride(16).make()?;
        let vel0 = cp_buff.attribute().size(2).stride(16).offset(8).index(1).make()?;
        let vao0 = self.gpu.vao()?.attributes(vec![pos0, vel0]).make()?;
        let tfo0 = self.gpu.tfo()?.buffer(&cp_buff).make()?;
        let buffer1 = self.gpu.buffer()?;
        buffer1.writer().array(point_count * 16 * 2).make()?.act().await?;
        let pos1 = buffer1.attribute().size(2).stride(16).make()?;
        let vel1 = buffer1.attribute().size(2).stride(16).offset(8).index(1).make()?;
        let vao1 = self.gpu.vao()?.attributes(vec![pos1, vel1]).make()?;
        let tfo1 = self.gpu.tfo()?.buffer(buffer1).make()?;
        let draw0 = self.gpu
            .draw_arrays(prog.clone())
            .mode(WGLRC::POINTS)
            .vao(vao0)
            .tfo(tfo1)
            .count(point_count)
            .instances(1)
            .tick(&tick)
            .make()?;
        let draw1 = self.gpu
            .draw_arrays(prog)
            .mode(WGLRC::POINTS)
            .vao(vao1)
            .tfo(tfo0)
            .count(point_count)
            .instances(1)
            .tick(&tick)
            .make()?;

        let seg_count = 80;
        let vertex = self.gpu.vertex_shader(NURBS)?;
        let fragment = self.gpu.fragment_shader(shader::basic::FRAGMENT_EMPTY)?;
        let program = self.gpu
            .program(vertex, fragment)?
            .out("position0")
            .make()?;
        let mut curve_array = vec![];
        let order = 4;
        let curve_count = point_count / order;
        for _ in 0..curve_count {
            curve_array.push(order as f32);
            curve_array.extend(vec![0., 0., 0., 0.,  1., 1., 1., 1.]);
        }
        let nurbs_buff = self.gpu.buffer()?;
        let _ = nurbs_buff.writer().array(curve_array).make()?.act().await?;
        let attribs = vec![
            nurbs_buff.attribute().size(1).stride(36).divisor(1).make()?,
            nurbs_buff.attribute().size(4).stride(36).offset(4).divisor(1).index(1).make()?,
            nurbs_buff.attribute().size(4).stride(36).offset(20).divisor(1).index(2).make()?,
            // buffer.attribute().size(4).stride(68).offset(36).index(3).make()?,
            // buffer.attribute().size(4).stride(68).offset(52).index(4).make()?,
        ];
        let vao = self.gpu.vao()?.attributes(attribs).make()?;
        let basis_buf = self.gpu.buffer()?;
        let _ = basis_buf.writer().array(4 * (order * seg_count * curve_count)).make()?.act().await?;
        let tfo = self.gpu.tfo()?.buffer(&basis_buf).make()?;
        let basis_draw = self.gpu
            .draw_arrays(program)
            .mode(WGLRC::POINTS)
            .vao(vao)
            .tfo(tfo)
            .rasterizer_discard(true)
            .count(seg_count)
            .instances(curve_count)
            .tick(&tick)
            .make()?;
        // let reader = basis_buf.reader().size(order * seg_count * curve_count + 3)
        // .draw(basis_draw.clone())
        // .make()?;

        let vert = self.gpu.vertex_shader(CURVE)?;
        let frag = self.gpu.fragment_shader(CURVE_FRAG)?;
        let prog = self.gpu.program(vert, frag)?.make()?;
        let attribs = vec![
            cp_buff.attribute().size(2).stride(16).divisor(1).make()?,
            cp_buff.attribute().size(2).stride(16).offset(16).index(1).divisor(1).make()?,
            cp_buff.attribute().size(2).stride(16).offset(32).index(2).divisor(1).make()?,
            cp_buff.attribute().size(2).stride(16).offset(48).index(3).divisor(1).make()?,
            basis_buf.attribute().size(4).stride(16).index(4).make()?,
        ];
        let vao = self.gpu.vao()?.attributes(attribs).make()?;
        let curve_draw = self.gpu
            .draw_arrays(prog)
            .mode(WGLRC::POINTS)
            .vao(vao)
            .count(seg_count) //////////////////////////////////////////
            .instances(curve_count)
            .tick(&tick)
            .make()?;

        let particles = ParticlesBuilder::default().draw0(draw0).draw1(draw1).basis(basis_draw)
        .curve(curve_draw)
        // .reader(reader)
        .tick(tick);
        Ok(particles)
    }
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into), build_fn(error = "graph::Error"))]
pub struct Particles {
    // gpu: Gpu,
    tick: Hub<i32>,
    draw0: Node<DrawArrays>,
    draw1: Node<DrawArrays>,
    basis: Node<DrawArrays>,
    curve: Node<DrawArrays>,
    // reader: Hub<Vf32>,
}

impl ParticlesBuilder {
    pub fn make(self) -> Result<Node<Particles>> {
        let mut part = self.build()?;
        Node::make(|back|{
            part.tick = part.tick.backed(back)?;
            Ok(part)
        })
    }
}

impl Act for Particles {
    async fn act(&self) -> Result<()> {
        if self.tick.base().await? % 2 == 0{
            self.draw0.act().await?;
        } else {
            self.draw1.act().await?;
        }
        self.basis.act().await?;
        self.curve.act().await?;
        // console_log!("basis: {:?}", self.reader.base().await?);
        Ok(())
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
    // out_pos = mod(out_pos + 1.0, 2.0) - 1.0;
    out_vel = vel;
    if(out_pos.x < -1. || out_pos.x > 1.) {
        out_vel.x = -out_vel.x;
    }
    if(out_pos.y < -1. || out_pos.y > 1.) {
        out_vel.y = -out_vel.y;
    }
    gl_Position = vec4(out_pos.x, out_pos.y, 0., 1.);
    gl_PointSize = 6.;
}";

pub const PARTICLES_FRAG: &str = r"#version 300 es
precision highp float;
layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(0.2, 0.2, 0.8, 1);
}";

pub const CURVE: &str = r"#version 300 es
layout(location = 0) in vec2 c0;
layout(location = 1) in vec2 c1;
layout(location = 2) in vec2 c2;
layout(location = 3) in vec2 c3;
layout(location = 4) in vec4 b;
void main() {
    vec2 out_pos = vec2(0., 0.);
    out_pos.x = c0.x*b[0] + c1.x*b[1] + c2.x*b[2] + c3.x*b[3];
    out_pos.y = c0.y*b[0] + c1.y*b[1] + c2.y*b[2] + c3.y*b[3];
    gl_Position = vec4(out_pos.x, out_pos.y, 0., 1.);
    gl_PointSize = 4.;
}";

pub const CURVE_FRAG: &str = r"#version 300 es
precision highp float;
layout(location = 0) out vec4 outColor;
void main() {
    outColor = vec4(0.8, 0.2, 0.2, 1);
}";


pub const NURBS: &str = r"#version 300 es
layout(location = 0) in float order;
layout(location = 1) in vec4 knots0;
layout(location = 2) in vec4 knots1;
// layout(location = 3) in vec4 knots2;
// layout(location = 4) in vec4 knots3;

out vec4 position0;
// out vec4 position1;

// max order
const int order_max = 4;
// max knots
const int knot_max = 8;

void main() {
    int knot_index = knot_max - int(order) - 1;
    float u = float(gl_VertexID) / 50.0;
    float[knot_max] knots = float[knot_max](
        knots0[0], knots0[1], knots0[2], knots0[3], 
        knots1[0], knots1[1], knots1[2], knots1[3]
            // knots2[0], knots2[1], knots2[2], knots2[3],
            // knots3[0], knots3[1], knots3[2], knots3[3]
    );
        // float[order_max] pos = float[order_max](0., 0., 0., 0., 0., 0., 0., 1.);
    float[order_max] pos = float[order_max](0., 0., 0., 1.);
    for (int deg = 1; deg < int(order); deg++) {
        for (int i = 0; i < deg + 1; i++) {
            int b0 = order_max - 1 - deg + i;
            int b1 = b0 + 1;
            int k0 = knot_index + i; 
            int k1 = k0 + 1;
            float ps = 0.0;
                // float vel = 0.0;
            if(pos[b0] > 0.0){ // piecewise part of b-spline basis N?
                float distance = knots[k0] - knots[k0 - deg];
                ps += pos[b0] * (u - knots[k0 - deg]) / distance; // Part A of recursive N
                    // vel += basis.0[b0] * deg as f32 / distance;
            }
            if(b1 < order_max && pos[b1] > 0.0){ // piecewise part of b-spline basis N?
                float distance = knots[k1] - knots[k1 - deg];
                ps += pos[b1] * (knots[k1] - u) / distance; // Part B of recursive N
                    // vel -= basis.0[b1] * deg as f32 / distance;
            } 
            pos[b0] = ps; 
                // velocity[b0] = vel;
        }
    }
    position0 = vec4(pos[0], pos[1], pos[2], pos[3]);
    // position1 = vec4(pos[4], pos[5], pos[6], pos[7]);
}";