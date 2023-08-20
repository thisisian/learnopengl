extern crate gl;
extern crate sdl2;

use learnopengl::*;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const ENABLE_POLYGON_MODE: bool = false;

unsafe fn pre_render() {
    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
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
    sdl_context.mouse().show_cursor(false);
    sdl_context.mouse().set_relative_mouse_mode(true);

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    assert_eq!(gl_attr.context_profile(), sdl2::video::GLProfile::Core);
    assert_eq!(gl_attr.context_version(), (3, 3));

    unsafe {
        gl::Viewport(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    }

    // load shaders
    let box_shader_program = {
        let vertex_shader_source =
            std::str::from_utf8(include_bytes!("./shaders/vertex_shader.glsl")).unwrap();
        let vertex_shader =
            unsafe { Shader::from_str(vertex_shader_source, ShaderType::VertexShader).unwrap() };

        let fragment_shader_source =
            std::str::from_utf8(include_bytes!("./shaders/fragment_shader.glsl")).unwrap();
        let fragment_shader = unsafe {
            Shader::from_str(fragment_shader_source, ShaderType::FragmentShader).unwrap()
        };

        // link shaders
        let shader_program = unsafe { ShaderProgram::new() };
        unsafe {
            shader_program.attach_shader(vertex_shader);
            shader_program.attach_shader(fragment_shader);
            shader_program
                .link_program()
                .expect("Shader linking failed");
        };
        shader_program
    };

    let light_shader_program = {
        let vertex_shader_source =
            std::str::from_utf8(include_bytes!("./shaders/vertex_shader_light.glsl")).unwrap();
        let vertex_shader =
            unsafe { Shader::from_str(vertex_shader_source, ShaderType::VertexShader).unwrap() };

        let fragment_shader_source =
            std::str::from_utf8(include_bytes!("./shaders/fragment_shader_light.glsl")).unwrap();
        let fragment_shader = unsafe {
            Shader::from_str(fragment_shader_source, ShaderType::FragmentShader).unwrap()
        };

        // link shaders
        let shader_program = unsafe { ShaderProgram::new() };
        unsafe {
            shader_program.attach_shader(vertex_shader);
            shader_program.attach_shader(fragment_shader);
            shader_program
                .link_program()
                .expect("Shader linking failed");
        };
        shader_program
    };

    #[rustfmt::skip]
    let cube_verts: [f32; 216] = [
        // loca            // normals
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
         0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,

        -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,
         0.5, -0.5,  0.5,  0.0,  0.0, 1.0,
         0.5,  0.5,  0.5,  0.0,  0.0, 1.0,
         0.5,  0.5,  0.5,  0.0,  0.0, 1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,

         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,
         0.5,  0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5,  0.5,  1.0,  0.0,  0.0,
         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
         0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0
    ];

    let vao = unsafe { create_vao(&cube_verts) };
    let vao_light = unsafe { create_vao(&cube_verts) };

    if ENABLE_POLYGON_MODE {
        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE) }
    }

    unsafe { check_gl_error().unwrap() };

    let light_position = glam::vec3(1.2, 1.0, 2.0);
    let mut projection: glam::Mat4;

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let mut camera = Camera::new();
    camera.set_position(glam::vec3(0.0, 0.0, 3.0));
    let mut keyboard = Keyboard::new();
    let mut current_frame: std::time::Instant;
    let mut last_frame = std::time::Instant::now();
    let mut delta_time: std::time::Duration;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        current_frame = std::time::Instant::now();
        delta_time = current_frame - last_frame;
        last_frame = current_frame;
        unsafe {
            pre_render();
            box_shader_program.use_program();
        };

        projection =
            glam::Mat4::perspective_rh(f32::to_radians(camera.zoom), 800.0 / 600.0, 0.1, 100.0);
        let view = camera.get_view_matrix();

        // Drawing center cube
        unsafe {
            box_shader_program.set_uniform_mat4("view", &view).unwrap();
            box_shader_program
                .set_uniform_mat4("projection", &projection)
                .unwrap();
            box_shader_program
                .set_uniform_vec3("objectColor", 1.0, 0.5, 0.31)
                .unwrap();
            box_shader_program
                .set_uniform_vec3("lightColor", 1.0, 1.0, 1.0)
                .unwrap();
        };
        unsafe { gl::BindVertexArray(vao) };
        let model = glam::Mat4::IDENTITY;
        unsafe {
            box_shader_program
                .set_uniform_mat4("model", &model)
                .unwrap()
        };
        unsafe {
            box_shader_program
                .set_uniform_vec3(
                    "lightPos",
                    light_position.x,
                    light_position.y,
                    light_position.z,
                )
                .unwrap()
        };
        unsafe {
            box_shader_program
                .set_uniform_vec3(
                    "viewPos",
                    camera.position.x,
                    camera.position.y,
                    camera.position.z,
                )
                .unwrap()
        };
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 36) };

        // Drawing light source cube
        unsafe {
            light_shader_program.use_program();
            light_shader_program
                .set_uniform_mat4("view", &view)
                .unwrap();
            light_shader_program
                .set_uniform_mat4("projection", &projection)
                .unwrap();
        };
        unsafe { gl::BindVertexArray(vao_light) };
        let model = glam::Mat4::from_scale_rotation_translation(
            glam::vec3(0.2, 0.2, 0.2),
            glam::Quat::IDENTITY,
            light_position,
        );
        unsafe {
            light_shader_program
                .set_uniform_mat4("model", &model)
                .unwrap()
        };
        unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 36) };

        // clean up
        unsafe { gl::BindVertexArray(0) };
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
                sdl2::event::Event::MouseWheel {
                    timestamp: _,
                    window_id: _,
                    which: _,
                    x: _,
                    y,
                    direction: _,
                } => {
                    camera.process_mouse_scroll(y);
                }
                sdl2::event::Event::MouseMotion {
                    timestamp: _,
                    window_id: _,
                    which: _,
                    mousestate: _,
                    x: _,
                    y: _,
                    xrel,
                    yrel,
                } => {
                    camera.process_mouse_movement(xrel, yrel);
                }
                sdl2::event::Event::KeyDown {
                    timestamp: _,
                    window_id: _,
                    keycode,
                    scancode: _,
                    keymod: _,
                    repeat: _,
                } => match keycode {
                    Some(sdl2::keyboard::Keycode::Escape) => break 'running,
                    Some(sdl2::keyboard::Keycode::W) => {
                        keyboard.w = true;
                    }
                    Some(sdl2::keyboard::Keycode::A) => {
                        keyboard.a = true;
                    }
                    Some(sdl2::keyboard::Keycode::S) => {
                        keyboard.s = true;
                    }
                    Some(sdl2::keyboard::Keycode::D) => {
                        keyboard.d = true;
                    }
                    _ => {}
                },
                sdl2::event::Event::KeyUp {
                    timestamp: _,
                    window_id: _,
                    keycode,
                    scancode: _,
                    keymod: _,
                    repeat: _,
                } => match keycode {
                    Some(sdl2::keyboard::Keycode::W) => {
                        keyboard.w = false;
                    }
                    Some(sdl2::keyboard::Keycode::A) => {
                        keyboard.a = false;
                    }
                    Some(sdl2::keyboard::Keycode::S) => {
                        keyboard.s = false;
                    }
                    Some(sdl2::keyboard::Keycode::D) => {
                        keyboard.d = false;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if keyboard.w {
            camera.process_keyboard(CameraDirection::Forward, delta_time)
        }
        if keyboard.a {
            camera.process_keyboard(CameraDirection::Left, delta_time)
        }
        if keyboard.s {
            camera.process_keyboard(CameraDirection::Backward, delta_time)
        }
        if keyboard.d {
            camera.process_keyboard(CameraDirection::Right, delta_time)
        }
    }
}
