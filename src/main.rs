extern crate gl;
extern crate sdl2;

use learnopengl::*;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const ENABLE_POLYGON_MODE: bool = false;

unsafe fn pre_render() {
    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
}

fn main() {
    let program_start_time = std::time::Instant::now();
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
    let verts: [f32; 32] = [
        // loc            // color        // texture coords
         0.5,  0.5, 0.0,  1.0, 0.0, 0.0,  1.0, 1.0,
         0.5, -0.5, 0.0,  0.0, 1.0, 0.0,  1.0, 0.0,
        -0.5, -0.5, 0.0,  0.0, 0.0, 1.0,  0.0, 0.0,
        -0.5,  0.5, 0.0,  1.0, 1.0, 0.0,  0.0, 1.0,
    ];

    #[rustfmt::skip]
    let indices: [u32; 6] = [
        0, 1, 3,
        1, 2, 3,
    ];

    let vao = unsafe { create_vao(&verts, &indices) };

    let texture = unsafe { Texture::new("container.jpg").unwrap() };
    let texture_smile = unsafe { Texture::new("awesomeface.png").unwrap() };

    if ENABLE_POLYGON_MODE {
        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE) }
    }

    unsafe { check_gl_error().unwrap() };

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        unsafe {
            pre_render();
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture_smile.id);
            shader_program.use_program();
            shader_program.set_uniform_i32("texture2", 1).unwrap();
            let trans1 = glam::Mat4::from_scale_rotation_translation(
                glam::vec3(0.5, 0.5, 0.5),
                glam::Quat::from_rotation_z(f32::to_radians(
                    program_start_time.elapsed().as_secs_f32() * 90.0,
                )),
                glam::vec3(0.5, -0.5, 0.0),
            );
            shader_program
                .set_uniform_mat4("transform", &trans1)
                .unwrap();
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
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
