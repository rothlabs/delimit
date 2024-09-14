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
