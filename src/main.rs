extern crate gl;
extern crate sdl2;

mod lib;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const ENABLE_POLYGON_MODE: bool = false;

unsafe fn render(shader_program: u32, vbo: u32) {
    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
    gl::UseProgram(shader_program);
    gl::BindVertexArray(vbo);
    gl::DrawArrays(gl::TRIANGLES, 0, 3);
}

fn gl_enum_to_error(err: gl::types::GLenum) -> String {
    match err {
        gl::INVALID_ENUM => "GL_INVALID_ENUM".to_owned(),
        gl::INVALID_OPERATION => "GL_INVALID_OPERATION".to_owned(),
        gl::STACK_OVERFLOW => "GL_STACK_OVERFLOW".to_owned(),
        gl::STACK_UNDERFLOW => "GL_STACK_UNDERFLOW".to_owned(),
        gl::OUT_OF_MEMORY => "GL_OUT_OF_MEMORY".to_owned(),
        gl::INVALID_FRAMEBUFFER_OPERATION => "GL_INVALID_FR".to_owned(),
        gl::CONTEXT_LOST => "GL_CONTEXT_LOST".to_owned(),
        _ => format!("Unknown error code {}", err).to_owned(),
    }
}

unsafe fn check_gl_error() -> Result<(), String> {
    let err = gl::GetError();
    if err != 0 {
        Err(gl_enum_to_error(err))
    } else {
        Ok(())
    }
}

unsafe fn check_shader_link_errors(shader: u32) {
    let mut success: i32 = 0;
    gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success as *mut i32);
    if success == gl::FALSE as i32 {
        println!("FAILED TO COMPILE!");
        let mut log_length: i32 = 0;
        gl::GetProgramiv(shader, gl::INFO_LOG_LENGTH, &mut log_length as *mut i32);

        let mut buff = Vec::<u8>::with_capacity(log_length as usize);
        println!("{log_length}");
        gl::GetProgramInfoLog(
            shader,
            log_length,
            std::ptr::null_mut(),
            buff.as_ptr() as *mut i8,
        );
        buff.set_len((log_length) as usize);

        println!("{log_length}");
        let c_string = std::ffi::CString::from_vec_with_nul(buff).unwrap();
        println!("{}", c_string.to_str().unwrap());
    } else {
        println!("SUCCESS!");
    }
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
    let vertex_shader =
        unsafe { lib::create_shader(vertex_shader_source, gl::VERTEX_SHADER).unwrap() };

    let fragment_shader_source = r#"
            #version 330 core
            out vec4 FragColor;

            void main()
            {
                FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
            }"#;

    let fragment_shader =
        unsafe { lib::create_shader(fragment_shader_source, gl::FRAGMENT_SHADER).unwrap() };

    // link shaders
    let shader_program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        check_shader_link_errors(shader_program);

        // These are no longer needed now that we've linked them.
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    // create verts
    let verts: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    let mut vbo: u32 = 0;
    let mut vao = 0;
    unsafe {
        // Initialize vbo and vao
        gl::GenVertexArrays(1, &mut vao as *mut u32);
        gl::GenBuffers(1, &mut vbo as *mut u32);
    }
    unsafe {
        // These steps need to be done in this order
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // Initialize data in the buffer
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (verts.len() * std::mem::size_of::<f32>()) as isize,
            verts.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW, // data will not cange often
        );

        // Configure attributes.
        // Describe the kind of data we're passing to location 0 (index zero of vao?).
        gl::VertexAttribPointer(
            0, // we want to bind this attribute to position 0
            3, // each vertex is three floats long
            gl::FLOAT,
            gl::FALSE, // do not normalize data points between [-1.0, 1.0]
            0,         // stride 0 defaults to width of each vertex without additional data
            std::ptr::null(),
        );
        // Enable the attribute.
        gl::EnableVertexAttribArray(0);

        // unbind buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    if ENABLE_POLYGON_MODE {
        unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE) }
    }

    unsafe { check_gl_error().unwrap() };

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        unsafe { render(shader_program, vao) };

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
