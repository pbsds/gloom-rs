#version 430 core

out vec4 color;

vec4 create_checker(in float x, in float y, in float z) {
    float checkSize = 1.8;
    float modResult = mod(
        floor(checkSize * x) +
        floor(checkSize * y) + 
        floor(checkSize * z), 2.0
    );
    float result = max(sign(modResult), 0.0);
    return vec4(result, result, result, 1.0f);
}

in vec3 fade;

void main()
{
    color = vec4(0.5f, 0.0f, 0.0f, 1.0f);
    // color = create_checker(gl_FragCoord.x, gl_FragCoord.y, gl_FragCoord.z); // comment out to enable checkers
}

