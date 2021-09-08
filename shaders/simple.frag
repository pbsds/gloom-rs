#version 430 core

out vec4 color;

void main()
{
    //color = vec4(0.0f, 1.0f, 0.0f, 1.0f);
    if(gl_FragCoord.x % 2 ==0)
        if(gl_FragCoord.y % 2 == 0)
            color = vec4(0.0f, 0.0f, 0.0f, 1.0);
            
        else
            color = vec4(1.0f, 1.0f, 1.0f, 1.0);
    else
        if(gl_FragCoord.y % 2 == 0)
            
            color = vec4(1.0f, 1.0f, 1.0f, 1.0);
            
        else
            color = vec4(0.0f, 0.0f, 0.0f, 1.0);
}

