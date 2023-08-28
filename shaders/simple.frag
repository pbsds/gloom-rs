#version 430 core

out vec4 color;

// Uniform variables that need to be updated in the rendering loop
uniform layout(location=1) float time;


void main()
{
    float time_dump = time; // just use `time`somewhere so that it doesn't crash
    vec4 color_1 = vec4(0.6f, 0.1f, 0.2f, 1.0f);
    vec4 color_2 = vec4(1.0f, 1.0f, 1.0f, 1.0f);
    
    // Checkerboard
    //int size = 40;
    //int condition = int(mod(floor(gl_FragCoord.x/size),2)==mod(floor(gl_FragCoord.y/size),2));
    
    
    // Circle
    //int amplitude = 100*100;
    //int size = 200;
    //int centerX=1000;
    //int centerY=750;
    //int condition = int((gl_FragCoord.x-centerX)*(gl_FragCoord.x-centerX)
    //+ (gl_FragCoord.y-centerY)*(gl_FragCoord.y-centerY) < size*size);

    // Color changing over time
    //float condition = 0.5 * (1.0 + sin(time));  // to have a factor between 0 and 1

    // Sine wave
    int thickness = 20;
    int positionY = 750;
    int amplitude = 100;
    float frequency = 0.01;
    int condition = int(gl_FragCoord.y<(positionY+sin(time)*amplitude*sin(frequency*gl_FragCoord.x+time)))
    * int(gl_FragCoord.y>(positionY+sin(time)*amplitude*sin(frequency*gl_FragCoord.x+time)-thickness)) ; 

    // Spiral 
    //int centerX = 1000;
    //int centerY = 750;
    //vec2 uv = (gl_FragCoord.xy - vec2(centerX,centerY)); // Centered coordinates
    //float angle = atan(uv.y, uv.x); // Angle of rotation
    //float radius = length(uv);  // Radius from the center
    //float spiralStart = 0.0;    // minimum radius
    //float spiralEnd = 2000.0;      // maximum radius
    //float tightness = 10.0;    // Related to the number of loops (not very clear)
    //float numBranches = 5.0;    // Number of branches
    //float spiral = mod(numBranches*angle +  tightness * sqrt(sqrt(radius))*sin(time/20), 2.0 * 3.14159265358979323846);  // Magic
    //int condition = int(radius >= spiralStart && radius <= spiralEnd && spiral >= 0.0 && spiral <= 1);  // Magic


    color = condition*color_1+(1-condition)*color_2;
    
}
