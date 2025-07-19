#version 460 core
layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 iColor;
layout (location = 2) in vec2 uv;
layout (location = 0) uniform float val;
// layout (location = 0) uniform mat4 P;
// layout (location = 1) uniform mat4 V;
// layout (location = 2) uniform mat4 M;
out vec3 vColor;
out vec2 vUv;
void main() {
    // gl_Position = P * V * M * vec4(pos.xyz, 1.0f);
    gl_Position = vec4(pos.x, pos.y + val, pos.z, 1.0f);
    vColor = iColor;
    vUv = uv;
}
