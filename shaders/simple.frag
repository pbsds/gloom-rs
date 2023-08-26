#version 430 core

out vec4 color;

void main()
{
    
    int size = 40;
    if (mod(floor(gl_FragCoord.x/size),2)==mod(floor(gl_FragCoord.y/size),2))
         color = vec4(0.8f, 0.0f, 0.8f, 1.0f);
    else
        color = vec4(1.0f, 0.87f, 0.32f, 1.0f);
    //color = vec4(1.0f,1.0f,1.0f, ((mod(gl_FragCoord.x/40.0f,2))*(mod(gl_FragCoord.y/40.0f,2)))*1.0f);
    //vec2 uv = gl_FragCoord.xy / gl_Resolution.xy - 0.5;
    //color = vec4(vec3(step(uv.x * uv.y, 0.)), 1.);
}
