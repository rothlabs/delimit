use super::*;

#[derive(Builder, Make!)]
#[builder(pattern = "owned")]
pub struct Demo {
    #[builder(default = "20")]
    curves: i32,
    #[builder(default = "5000.0")]
    duration: f64,
    #[builder(default = "300")]
    width: u32,
    #[builder(default = "150")]
    height: u32,
}

impl Demo {
    pub fn start(self) {
        spawn_local(async move {self.run().await.unwrap()})
    }
    pub async fn run(&self) -> dom::Result<()> {
        let window = Window::new()?;
        let doc = window.document()?;
        let tick = 0.leaf();
        let _nurbs = self.nurbs(&doc, &tick).await?;
        let start = doc.time()?;
        let mut last = start;
        loop {
            let time = window.request_animation_frame().await?.as_f64().result()?;
            if time - start > self.duration {
                break;
            }
            if time - last > 1000. / 60. {
                last = time;
                tick.write(|x| *x += 1).await?;
            }
        }
        Ok(())
    }
    pub async fn nurbs(
        &self,
        doc: &Document,
        tick: impl Into<Hub<i32>>,
    ) -> dom::Result<Node<Nurbs>> {
        let canvas = doc.body()?.stem("canvas")?.canvas()?;
        canvas.set_size(self.width, self.height);
        let gpu = canvas.gpu()?;
        let tick = &tick.into();
        let vert = gpu.vertex_shader(PARTICLES)?;
        let frag = gpu.fragment_shader(PARTICLES_FRAG)?;
        let prog = gpu
            .program(vert, frag)?
            .out("out_pos")
            .out("out_vel")
            .out_type(WGLRC::SEPARATE_ATTRIBS)
            .node()?;
        let order = 16;
        let point_count = order * self.curves;
        let mut point_array = vec![];
        for _ in 0..point_count {
            point_array.push(random_float());
            point_array.push(random_float());
        }
        let mut vel_array = vec![];
        for _ in 0..point_count {
            vel_array.push(random_float() * 0.01);
            vel_array.push(random_float() * 0.01);
        }
        let pos_buff0 = gpu.buffer()?;
        pos_buff0.writer().array(point_array).node()?.act().await?;
        let vel_buff0 = gpu.buffer()?;
        vel_buff0.writer().array(vel_array).node()?.act().await?;
        let pos0 = pos_buff0.attribute().size(2).stride(8).node()?;
        let vel0 = vel_buff0.attribute().size(2).stride(8).index(1).node()?;
        let vao0 = gpu.vao()?;
        let vao_writer0 = vao0.writer().attributes(vec![pos0, vel0]).apex()?;
        let tfo0 = gpu.tfo()?.buffer(&pos_buff0).buffer(&vel_buff0).make()?;
        let pos_buff1 = gpu.buffer()?;
        pos_buff1
            .writer()
            .array(point_count * 8)
            .node()?
            .act()
            .await?;
        let vel_buff1 = gpu.buffer()?;
        vel_buff1
            .writer()
            .array(point_count * 8)
            .node()?
            .act()
            .await?;
        let pos1 = pos_buff1.attribute().size(2).stride(8).node()?;
        let vel1 = vel_buff1.attribute().size(2).stride(8).index(1).node()?;
        let vao1 = gpu.vao()?;
        let vao_writer1 = vao1.writer().attributes(vec![pos1, vel1]).apex()?;
        let tfo1 = gpu.tfo()?.buffer(pos_buff1).buffer(vel_buff1).make()?;
        let draw0 = gpu
            .draw_arrays(prog.clone())
            .mode(WGLRC::POINTS)
            .stem(tick)
            .stem(vao_writer0)
            .vao(vao0)
            .tfo(tfo1)
            .count(point_count)
            .instances(1)
            .rasterizer_discard(true)
            .node()?;
        let draw1 = gpu
            .draw_arrays(prog)
            .mode(WGLRC::POINTS)
            .stem(tick)
            .stem(vao_writer1)
            .vao(vao1)
            .tfo(tfo0)
            .count(point_count)
            .instances(1)
            .node()?;

        let seg_count = 1000;
        let vertex = gpu.vertex_shader(NURBS)?;
        let fragment = gpu.fragment_shader(shader::basic::FRAGMENT_EMPTY)?;
        let program = gpu
            .program(vertex, fragment)?
            .out("position0")
            .out("position1")
            .out("position2")
            .out("position3")
            .node()?;
        let mut curve_array = vec![];
        let curve_count = point_count / order;
        for _ in 0..curve_count {
            curve_array.push(order as f32);
            curve_array.extend(vec![
                0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 1., 1., 1., 1., 1.,
                1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.,
            ]);
        }
        let nurbs_buff = gpu.buffer()?;
        nurbs_buff.writer().array(curve_array).node()?.act().await?;
        #[rustfmt::skip]
        let attribs = vec![
            nurbs_buff.attribute().size(1).stride(132).divisor(1).node()?,
            nurbs_buff.attribute().size(4).stride(132).offset(4).divisor(1).index(1).node()?,
            nurbs_buff.attribute().size(4).stride(132).offset(20).divisor(1).index(2).node()?,
            nurbs_buff.attribute().size(4).stride(132).offset(36).divisor(1).index(3).node()?,
            nurbs_buff.attribute().size(4).stride(132).offset(52).divisor(1).index(4).node()?,
            nurbs_buff.attribute().size(4).stride(132).offset(68).divisor(1).index(5).node()?,
            nurbs_buff.attribute().size(4).stride(132).offset(84).divisor(1).index(6).node()?,
            nurbs_buff.attribute().size(4).stride(132).offset(100).divisor(1).index(7).node()?,
            nurbs_buff.attribute().size(4).stride(132).offset(116).divisor(1).index(8).node()?,
        ];
        let vao = gpu.vao()?;
        let vao_writer = vao.writer().attributes(attribs).apex()?;
        let basis_buf = gpu.buffer()?;
        basis_buf
            .writer()
            .array(4 * order * seg_count * curve_count)
            .node()?
            .act()
            .await?;
        let tfo = gpu.tfo()?.buffer(&basis_buf).make()?;
        let basis_draw = gpu
            .draw_arrays(program)
            .mode(WGLRC::POINTS)
            // .stem(tick) // enable this to calculate basis on every tick
            .stem(vao_writer)
            .vao(vao)
            .tfo(tfo)
            .count(seg_count)
            .instances(curve_count)
            .rasterizer_discard(true)
            .node()?;

        let vert = gpu.vertex_shader(CURVE)?;
        let frag = gpu.fragment_shader(CURVE_FRAG)?;
        let prog = gpu.program(vert, frag)?.node()?;
        #[rustfmt::skip]
        let attribs = vec![
            pos_buff0.attribute().size(4).stride(16).divisor(1).node()?,
            pos_buff0.attribute().size(4).stride(16).offset(16).index(1).divisor(1).node()?,
            pos_buff0.attribute().size(4).stride(16).offset(32).index(2).divisor(1).node()?,
            pos_buff0.attribute().size(4).stride(16).offset(48).index(3).divisor(1).node()?,
            pos_buff0.attribute().size(4).stride(16).offset(64).index(4).divisor(1).node()?,
            pos_buff0.attribute().size(4).stride(16).offset(80).index(5).divisor(1).node()?,
            pos_buff0.attribute().size(4).stride(16).offset(96).index(6).divisor(1).node()?,
            pos_buff0.attribute().size(4).stride(16).offset(112).index(7).divisor(1).node()?,
            basis_buf.attribute().size(4).stride(64).offset(0).index(8).node()?,
            basis_buf.attribute().size(4).stride(64).offset(16).index(9).node()?,
            basis_buf.attribute().size(4).stride(64).offset(32).index(10).node()?,
            basis_buf.attribute().size(4).stride(64).offset(48).index(11).node()?,
        ];
        let vao = gpu.vao()?;
        let vao_writer = vao.writer().attributes(attribs).apex()?;
        let curve_draw = gpu
            .draw_arrays(prog)
            .mode(WGLRC::POINTS)
            .stem(tick)
            .stem(vao_writer)
            .vao(vao)
            .count(seg_count)
            .instances(curve_count)
            .node()?;

        let nurbs = NurbsBuilder::default()
            .gl(gpu)
            .draw0(draw0)
            .draw1(draw1)
            .basis(basis_draw)
            .curve(curve_draw)
            .tick(tick)
            .node()?;
        nurbs.act().await?;
        Ok(nurbs)
    }
}

#[derive(Builder, Debug, Unit!)]
#[builder(pattern = "owned", setter(into))]
pub struct Nurbs {
    gl: Gpu,
    tick: Hub<i32>,
    draw0: Node<DrawArrays>,
    draw1: Node<DrawArrays>,
    basis: Node<DrawArrays>,
    curve: Node<DrawArrays>,
}

impl Act for Nurbs {
    fn backed(&mut self, back: &Back) -> graph::Result<()> {
        self.draw0.back(back)?;
        self.draw1.back(back)?;
        self.basis.back(back)?;
        self.curve.back(back)?;
        self.tick.back(back)
    }
    async fn act(&self) -> graph::Result<()> {
        if self.tick.base().await? % 2 == 0 {
            self.draw0.act().await?;
        } else {
            self.gl.clear();
            self.draw1.act().await?;
            self.basis.act().await?;
            self.curve.act().await?;
        }
        Ok(())
    }
}

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
    gl_PointSize = 2.;
}";

pub const PARTICLES_FRAG: &str = r"#version 300 es
precision highp float;
layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(0.2, 0.2, 0.8, 1);
}";

pub const CURVE: &str = r"#version 300 es
layout(location = 0) in vec4 c0;
layout(location = 1) in vec4 c1;
layout(location = 2) in vec4 c2;
layout(location = 3) in vec4 c3;
layout(location = 4) in vec4 c4;
layout(location = 5) in vec4 c5;
layout(location = 6) in vec4 c6;
layout(location = 7) in vec4 c7;
layout(location = 8) in vec4 bA;
layout(location = 9) in vec4 bB;
layout(location = 10) in vec4 bC;
layout(location = 11) in vec4 bD;
void main() {
    vec2 out_pos = vec2(0., 0.);
    out_pos.x =  c0.x*bA[0] + c1.x*bA[1] + c2.x*bA[2] + c3.x*bA[3];
    out_pos.x += c4.x*bB[0] + c5.x*bB[1] + c6.x*bB[2] + c7.x*bB[3];
    out_pos.x += c0.z*bC[0] + c1.z*bC[1] + c2.z*bC[2] + c3.z*bC[3];
    out_pos.x += c4.z*bD[0] + c5.z*bD[1] + c6.z*bD[2] + c7.z*bD[3];
    out_pos.y =  c0.y*bA[0] + c1.y*bA[1] + c2.y*bA[2] + c3.y*bA[3];
    out_pos.y += c4.y*bB[0] + c5.y*bB[1] + c6.y*bB[2] + c7.y*bB[3];
    out_pos.y += c0.w*bC[0] + c1.w*bC[1] + c2.w*bC[2] + c3.w*bC[3];
    out_pos.y += c4.w*bD[0] + c5.w*bD[1] + c6.w*bD[2] + c7.w*bD[3];
    gl_Position = vec4(out_pos.x, out_pos.y, 0., 1.);
    gl_PointSize = 1.;
}";

// out_pos.x =  c0.x*bA[0] + c1.x*bA[1] + c2.x*bA[2] + c3.x*bA[3];
// out_pos.x += c0.z*bB[0] + c1.z*bB[1] + c2.z*bB[2] + c3.z*bB[3];
// out_pos.x += c4.x*bC[0] + c5.x*bC[1] + c6.x*bC[2] + c7.x*bC[3];
// out_pos.x += c4.z*bD[0] + c5.z*bD[1] + c6.z*bD[2] + c7.z*bD[3];
// out_pos.y =  c0.y*bA[0] + c1.y*bA[1] + c2.y*bA[2] + c3.y*bA[3];
// out_pos.y += c0.w*bB[0] + c1.w*bB[1] + c2.w*bB[2] + c4.w*bB[3];
// out_pos.y += c4.y*bC[0] + c5.y*bC[1] + c6.y*bC[2] + c7.y*bC[3];
// out_pos.y += c4.w*bD[0] + c5.w*bD[1] + c6.w*bD[2] + c7.w*bD[3];

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
layout(location = 3) in vec4 knots2;
layout(location = 4) in vec4 knots3;
layout(location = 5) in vec4 knots4;
layout(location = 6) in vec4 knots5;
layout(location = 7) in vec4 knots6;
layout(location = 8) in vec4 knots7;
out vec4 position0;
out vec4 position1;
out vec4 position2;
out vec4 position3;

// max order
const int order_max = 16;
// max knots
const int knot_max = 32;

void main() {
    int knot_index = knot_max - int(order) - 1;
    float u = float(gl_VertexID) / 1000.0;
    float[knot_max] knots = float[knot_max](
        knots0[0], knots0[1], knots0[2], knots0[3], 
        knots1[0], knots1[1], knots1[2], knots1[3],
        knots2[0], knots2[1], knots2[2], knots2[3],
        knots3[0], knots3[1], knots3[2], knots3[3],
        knots4[0], knots4[1], knots4[2], knots4[3], 
        knots5[0], knots5[1], knots5[2], knots5[3],
        knots6[0], knots6[1], knots6[2], knots6[3],
        knots7[0], knots7[1], knots7[2], knots7[3]
    );
    float[order_max] pos = float[order_max](0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 1.);
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
    position1 = vec4(pos[4], pos[5], pos[6], pos[7]);
    position2 = vec4(pos[8], pos[9], pos[10], pos[11]);
    position3 = vec4(pos[12], pos[13], pos[14], pos[15]);
}";

// for _ in 0..100 {
//     tick.write(|x| *x += 1).await.unwrap();
//     TimeoutFuture::new(16).await;
// }
