pub const VERTEX: &str = r"#version 300 es
precision highp float;
layout(location = 0) in vec4 a_position;

void main() {
    gl_Position = a_position;
}
";

pub const FRAGMENT_RED: &str = r"#version 300 es
precision highp float;
layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(1, 0.2, 0.2, 1);
}
";

pub const FRAGMENT_GREEN: &str = r"#version 300 es
precision highp float;
layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(0.2, 1, 0.2, 1);
}
";

pub const VERTEX_TEX: &str = r"#version 300 es
layout(location = 0) in vec4 aPosition;
layout(location = 1) in vec2 aTexCoord;

out vec2 vTexCoord;
void main()
{
	vTexCoord = aTexCoord;
    gl_Position = aPosition;
}
";

pub const FRAGMENT_TEX: &str = r"#version 300 es
precision mediump float;

in vec2 vTexCoord;

uniform sampler2D uSampler;

out vec4 fragColor;

void main()
{
    fragColor = texture(uSampler, vTexCoord);
}
";

pub const VERTEX_FEEDBACK: &str = r"#version 300 es
out float output0;
out float output1;

void main() {
    output0 = 1.0;
    output1 = 2.0;
}
";

pub const FRAGMENT_EMPTY: &str = r"#version 300 es
void main() {}
";

pub const NURBS: &str = r"#version 300 es
layout(location = 0) in float order;
layout(location = 1) in vec4 knots0;
layout(location = 2) in vec4 knots1;
layout(location = 3) in vec4 knots2;
layout(location = 4) in vec4 knots3;

out vec4 position0;
out vec4 position1;

// max order
const int order_max = 8;
// max knots
const int knot_max = 16;

void main() {
    int knot_index = knot_max - int(order) - 1;
    float u = 0.4;
    float[knot_max] knots = float[knot_max](
        knots0[0], knots0[1], knots0[2], knots0[3], 
        knots1[0], knots1[1], knots1[2], knots1[3],
        knots2[0], knots2[1], knots2[2], knots2[3],
        knots3[0], knots3[1], knots3[2], knots3[3]
    );
    float[order_max] pos = float[order_max](0., 0., 0., 0., 0., 0., 0., 1.);
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
}";