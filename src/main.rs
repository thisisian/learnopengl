extern crate gl;
extern crate sdl2;

mod lib;

use lib::*;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const ENABLE_POLYGON_MODE: bool = false;

unsafe fn draw(shader_program: u32, vbo: u32) {
    gl::UseProgram(shader_program);
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
    let vertex_shader_source = r#"
                #version 330 core
                layout (location = 0) in vec3 aPos;

                void main()
                {
                    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
                }
            "#;
    let vertex_shader = unsafe { create_shader(vertex_shader_source, gl::VERTEX_SHADER).unwrap() };

    let fragment_shader_orange_source = r#"
            #version 330 core
            out vec4 FragColor;

            void main()
            {
                FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
            }"#;

    let fragment_shader_yellow_source = r#"
            #version 330 core
            out vec4 FragColor;

            void main()
            {
                FragColor = vec4(1.0f, 1.0f, 0.2f, 1.0f);
            }"#;

    let fragment_shader_orange =
        unsafe { create_shader(fragment_shader_orange_source, gl::FRAGMENT_SHADER).unwrap() };

    let fragment_shader_yellow =
        unsafe { create_shader(fragment_shader_yellow_source, gl::FRAGMENT_SHADER).unwrap() };

    // link shaders
    let shader_program_orange =
        unsafe { link_shader_program(&[vertex_shader, fragment_shader_orange]) };
    let shader_program_yellow =
        unsafe { link_shader_program(&[vertex_shader, fragment_shader_yellow]) };

    // These are no longer needed now that we've linked them.
    unsafe {
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader_orange);
        gl::DeleteShader(fragment_shader_yellow);
    }

    #[rustfmt::skip]
    // create verts
    let verts1: [f32; 9] = [
        -0.5, 0.0, 0.0,
        0.0, -0.5, 0.0,
        0.0, 0.5, 0.0,
    ];

    #[rustfmt::skip]
    // create verts
    let verts2: [f32; 9] = [
        0.5, 0.0, 0.0,
        0.0, 0.5, 0.0,
        0.0, -0.5, 0.0,
    ];

    let vao1 = unsafe { create_vbo(&verts1) };
    let vao2 = unsafe { create_vbo(&verts2) };

    if ENABLE_POLYGON_MODE {
        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE) }
    }

    unsafe { check_gl_error().unwrap() };

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        unsafe {
            pre_render();
            draw(shader_program_orange, vao1);
            draw(shader_program_yellow, vao2);
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
