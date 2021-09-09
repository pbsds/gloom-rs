extern crate nalgebra_glm as glm;
use std::{ mem, ptr, os::raw::c_void };
use std::thread;
use std::sync::{Mutex, Arc, RwLock};

mod shader;
mod util;

use glutin::event::{Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;

const SCREEN_W: u32 = 800;
const SCREEN_H: u32 = 600;

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //
// The names should be pretty self explanatory
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()



// == // Modify and complete the function below for the first task
// unsafe fn FUNCTION_NAME(ARGUMENT_NAME: &Vec<f32>, ARGUMENT_NAME: &Vec<u32>) -> u32 { }
unsafe fn triangle_vao(vertices: &Vec<f32>, indices: &Vec<u32>) -> u32 {

    // Variables used for binding
    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    let vertex_index: u32 = 0;
    let mut index_buffer_id: u32 = 0;

    // Bind triangle_vao
    gl::GenVertexArrays(1, &mut vao);
    assert_ne!(vao, 0); // make sure 0 is not returned
    gl::BindVertexArray(vao);

    // Buffers for vertice coordinates
    gl::GenBuffers(1, &mut vbo);
    assert_ne!(vbo, 0); // make sure 0 is not returned
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);


    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(vertices),
        pointer_to_array(vertices),
        gl::STATIC_DRAW);

    let num_of_components = 3;
    gl::VertexAttribPointer(
        vertex_index,
        num_of_components,
        gl::FLOAT,
        gl::FALSE,
        0,
        ptr::null());

    gl::EnableVertexAttribArray(vertex_index);

    // setup vertex buffer object
    gl::GenBuffers(1, &mut index_buffer_id);
    assert_ne!(index_buffer_id, 0); // make sure 0 is not returned
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer_id);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        byte_size_of_array(indices),
        pointer_to_array(indices),
        gl::STATIC_DRAW);

    return vao
}

unsafe fn draw_scene(count: usize) {
    gl::FrontFace(gl::CW);
    gl::DrawElements(gl::TRIANGLES, count as i32, gl::UNSIGNED_INT, ptr::null()); // TRIANGLE_STRIP can be used to easier build up geometry

}

fn main() {

    // Triangles
    // let v1: Vec<f32> = vec![-0.6, -0.6, 0.0, 0.6, -0.6, 0.0, 0.0, 0.6, 0.0];
    // let v1: Vec<f32> = vec![-1.0, -1.0, 0.0, -1.0, 1.0, 0.0, -0.5, 0.5, 0.0, -0.6, -0.6, 0.0, 0.6, -0.6, 0.0, 0.0, 0.6, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
    // let v2: Vec<f32> = vec![0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.5, 0.0, 0.0 ];

    // const VERTICES: [Vertex; 3] =
    // [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    // let mut indices: Vec<u32> = vec![];
    // let mut indice: u32 = 0;
    // for i in 0..v1.len() {
    //     indices.push(indice);
    //     indice += 1;
    // }

    // println!("Result is:{}", v1.len());

    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(SCREEN_W, SCREEN_H));
    let cb = glutin::ContextBuilder::new()
        .with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    // Uncomment these if you want to use the mouse for controls, but want it to be confined to the screen and/or invisible.
    // windowed_context.window().set_cursor_grab(true).expect("failed to grab cursor");
    // windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers. This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE); //need to disable this to make mirroring to work, havent found a work around
            //edit: By using gl::frontface we change the direction it is drawed.
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!("{}: {}", util::get_gl_string(gl::VENDOR), util::get_gl_string(gl::RENDERER));
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!("GLSL\t: {}", util::get_gl_string(gl::SHADING_LANGUAGE_VERSION));
        }

        // == // Set up your VAO here

        //TASK 1

        //triangles data

        //1 triangle
        /* let vertices: Vec<f32> = vec![-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0];
        let indices: Vec<u32> = vec![0, 1, 2]; */

        //5 triangles, in this case the z axis will always be 0
        /* let vertices: Vec<f32> = vec![
            -1.0, -1.0, 0.0,  // 0
            -1.0, 1.0, 0.0,   // 1
            -0.5, 0.0, 0.0,   // 2
            0.0, 1.0, 0.0,    // 3
            1.0, 1.0, 0.0,    // 4
            0.0, -1.0, 0.0,   // 5
            0.0, 0.5, 0.0,    // 6
            0.0, -0.5, 0.0,   // 7
            0.5, 0.0, 0.0,     // 8
            1.0, -1.0, 0.0    // 9
        ]; */
        //remember to draw the correct order. right corner->top-> left corner
        /* let indices: Vec<u32> = vec![
            5, 2, 0,
            1, 2, 3,
            9, 8, 5,
            3, 8, 4,
            7, 8, 6,
            6, 2, 7
        ]; */


        //TASK 2

        //task 2a
        // == // Set up your vao here
        // unsafe {
        //     let draw_vao: u32 = 0;
        //     gl::BindVertexArray(draw_vao);
        //     let vao = triangle_vao(& v1, & indices);
        // }

        /* let vertices: Vec<f32> = vec![
            0.6, -0.8, 1.0,  // 0
            0.0, 0.4, 0.0,   // 1
            -0.8, -0.2, 1.0   // 2
        ];

        let indices: Vec<u32> = vec![
            0, 1, 2
        ]; */

        //task2b


        /* let indices: Vec<u32> = vec![
            2, 1, 0
        ];*/

        //Task 2d
        //changing the simple.vert files positions

        let vertices: Vec<f32> = vec![
            0.6, -0.2, 0.0,  // 0
            0.4, 0.4, 0.0,   // 1
            -0.6, -0.2, 0.0   // 2
        ];

        let indices: Vec<u32> = vec![
            0, 1, 2
        ];


        //generating the vao_id to the triangle that are getting drawed.
        let vao_id = unsafe{ triangle_vao(& vertices, & indices) };

        // Basic usage of shader helper:
        // The example code below returns a shader object, which contains the field `.program_id`.
        // The snippet is not enough to do the assignment, and will need to be modified (outside of
        // just using the correct path), but it only needs to be called once
        //
        //     shader::ShaderBuilder::new()
        //        .attach_file("./path/to/shader.file")
        //        .link();
        //this returns the unsafe function to the shader variable
        let shader = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.vert")
                .attach_file("./shaders/simple.frag")
                .link()
                .activate() //assignment says activate it, but doesnt seemed to be needed. this only runs the useProgram function
        };


        // Dissable to use custom frag shader*
        // unsafe {
        //     gl::UseProgram(0);
        // }

        // Used to demonstrate keyboard handling -- feel free to remove
        let mut _arbitrary_number = 0.0;

        let first_frame_time = std::time::Instant::now();
        let mut last_frame_time = first_frame_time;
        // The main rendering loop
        loop {
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(last_frame_time).as_secs_f32();
            last_frame_time = now;

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        VirtualKeyCode::A => {
                            _arbitrary_number += delta_time;
                        },
                        VirtualKeyCode::D => {
                            _arbitrary_number -= delta_time;
                        },


                        _ => { }
                    }
                }
            }
            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {



                *delta = (0.0, 0.0);
            }

            unsafe {
                gl::ClearColor(0.76862745, 0.71372549, 0.94901961, 1.0); // moon raker, full opacity
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                // Issue the necessary commands to draw your scene here

                draw_scene(vertices.len());
                gl::FrontFace(gl::CW); //CCW for counter clockwise, CW for Clockwise
                //draw the elements mode: triangle, number of points/count: lenght of the indices, type and void* indices

            }

            context.swap_buffers().unwrap();
        }
    });

    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events get handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent { event: WindowEvent::KeyboardInput {
                input: KeyboardInput { state: key_state, virtual_keycode: Some(keycode), .. }, .. }, .. } => {

                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        },
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle escape separately
                match keycode {
                    Escape => {
                        *control_flow = ControlFlow::Exit;
                    },
                    Q => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => { }
                }
            },
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            },
            _ => { }
        }
    });
}
