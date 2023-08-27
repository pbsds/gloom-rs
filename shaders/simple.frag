#version 430 core

out vec4 color;
uniform layout(location=1) float time; // needs to be updated in the rendering loop i think

void main()
{
    vec4 color_1 = vec4(0.6f, 0.1f, 0.2f, 1.0f);
    vec4 color_2 = vec4(1.0f, 1.0f, 1.0f, 1.0f);
    
    // Checkerboard
    //int size = 40;
    //int condition = int(mod(floor(gl_FragCoord.x/size),2)==mod(floor(gl_FragCoord.y/size),2));
    
    
    // Circle
    //int size = 200;
    //int centerX=1000;
    //int centerY=750;
    //int condition = int((gl_FragCoord.x-centerX)*(gl_FragCoord.x-centerX)
    //+ (gl_FragCoord.y-centerY)*(gl_FragCoord.y-centerY) < size*size);

    // Color changing over time
    float condition = 0.5 * (1.0 + sin(time));  // to have a factor between 0 and 1


    color = condition*color_1+(1-condition)*color_2;
    
}
