precision highp float;

const float TWO_PI = 6.283185307179586476925286766559;

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec2 a_size;
layout(location = 2) in vec4 a_uv;
layout(location = 3) in vec4 a_color;

out vec2 v_uv;
out vec4 v_color;

layout(std140) uniform vertex {
    mat4 ortho;
};

// UV Layout: xmin xmax ymin ymax
// ymin and ymax are swapped below because OpenGL reads images from bottom row to top row, but
// they're stored top to bottom on upload, so this corrects that.
vec4 uv_lut[4] = vec4[4](
    vec4(1.0, 0.0, 1.0, 0.0),  // left bottom
    vec4(1.0, 0.0, 0.0, 1.0),  // left top
    vec4(0.0, 1.0, 1.0, 0.0),  // right bottom
    vec4(0.0, 1.0, 0.0, 1.0)); // right top

vec2 size_lut[4] = vec2[4](
    vec2(0.0, 1.0),  // left top
    vec2(0.0, 0.0),  // right top
    vec2(1.0, 1.0),  // left bottom
    vec2(1.0, 0.0)); // right bottom

void main() {
    vec4 temp = a_uv * uv_lut[gl_VertexID];
    v_uv = vec2(temp.x + temp.y, temp.z + temp.w);
    v_color = a_color;

    vec3 size = vec3(a_size * size_lut[gl_VertexID], 0.0);
    vec3 pos = a_pos + size;
    gl_Position = ortho * vec4(pos, 1.0);
}