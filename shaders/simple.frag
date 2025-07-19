#version 460 core
layout(binding = 0) uniform sampler2D t;
in vec3 vColor;
in vec2 vUv;
out vec4 color;
void main() {
    color = vec4(vUv, 0, 1.0f);
}