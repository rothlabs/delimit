pub const VERTEX: &str = r"#version 300 es
layout(location = 0) in vec4 a_position;

void main() {
    gl_Position = a_position;
}
";

pub const FRAGMENT: &str = r"#version 300 es
precision highp float;

layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(1, 0, 0.5, 1);
}
";
