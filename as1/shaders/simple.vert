#version 430 core

in layout(location=0) vec3 position;
uniform mat3 mvp;

void main()
{
    gl_Position = vec4(mvp * position, 1.0f);
}