#version 430 core

in vec3 position;

void main()
{
    gl_Position = vec4(position.x * -1.0f, position.y * -1.0f, position.z, 1.0f);
}