extern crate gl;
extern crate sdl2;

mod lib;

use lib::*;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const ENABLE_POLYGON_MODE: bool = false;

unsafe fn draw(shader_program: &ShaderProgram, vbo: u32) {
    shader_program.use_program();
    gl::BindVertexArray(vbo);
    gl::DrawArrays(
        gl::TRIANGLES,
        0,
        3, /* must match number of verts in vbo */
    );
}

unsafe fn pre_render() {
    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
}

fn main() {
    let program_start = std::time::Instant::now();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("LEARNOPENGL", SCREEN_WIDTH, SCREEN_HEIGHT)
        //.resizable()
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    assert_eq!(gl_attr.context_profile(), sdl2::video::GLProfile::Core);
    assert_eq!(gl_attr.context_version(), (3, 3));

    unsafe {
        gl::Viewport(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    }

    // load shaders
    let vertex_shader_source =
        std::str::from_utf8(include_bytes!("./shaders/vertex_shader.glsl")).unwrap();
    let vertex_shader =
        unsafe { Shader::from_str(vertex_shader_source, ShaderType::VertexShader).unwrap() };

    let fragment_shader_source =
        std::str::from_utf8(include_bytes!("./shaders/fragment_shader.glsl")).unwrap();
    let fragment_shader =
        unsafe { Shader::from_str(fragment_shader_source, ShaderType::FragmentShader).unwrap() };

    // link shaders
    let shader_program = unsafe { ShaderProgram::new() };
    unsafe {
        shader_program.attach_shader(vertex_shader);
        shader_program.attach_shader(fragment_shader);
        shader_program
            .link_program()
            .expect("Shader linking failed");
    };

    #[rustfmt::skip]
    // create verts
    let verts1: [f32; 18] = [
        // loc           // color
        0.5, -0.5, 0.0,   1.0, 0.0, 0.0,
        -0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
        0.0, 0.5, 0.0,  0.0, 0.0, 1.0
    ];

    let vao = unsafe { create_vbo(&verts1) };

    if ENABLE_POLYGON_MODE {
        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE) }
    }

    unsafe { check_gl_error().unwrap() };

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        unsafe {
            let millis = program_start.elapsed().as_secs_f64() as f64;
            let green_value = (millis.sin() / 2.0) + 0.5;
            shader_program
                .set_uniform_f64("greenValue", green_value as f32)
                .expect("Failed to set uniform");
            pre_render();
            draw(&shader_program, vao);
        };

        window.gl_swap_window();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Window {
                    timestamp: _,
                    window_id: _,
                    win_event,
                } => {
                    if let sdl2::event::WindowEvent::Resized(width, height) = win_event {
                        unsafe {
                            gl::Viewport(0, 0, width, height);
                            println!("Resized to {width}, {height}")
                        }
                    }
                }
                sdl2::event::Event::Quit { .. } => break 'running,
                sdl2::event::Event::KeyDown {
                    timestamp: _,
                    window_id: _,
                    keycode,
                    scancode: _,
                    keymod: _,
                    repeat: _,
                } => match keycode {
                    Some(sdl2::keyboard::Keycode::Escape) => break 'running,
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
