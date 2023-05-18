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

pub unsafe fn check_gl_error() -> Result<(), String> {
    let err = gl::GetError();
    if err != 0 {
        Err(gl_enum_to_error(err))
    } else {
        Ok(())
    }
}

unsafe fn check_shader_compile_errors(shader: u32) -> Result<(), String> {
    let mut success: i32 = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success as *mut i32);
    if success == gl::FALSE as i32 {
        let mut log_length: i32 = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length as *mut i32);

        let mut buff = Vec::<u8>::with_capacity(log_length as usize);
        gl::GetShaderInfoLog(
            shader,
            log_length,
            std::ptr::null_mut(),
            buff.as_ptr() as *mut i8,
        );
        buff.set_len((log_length) as usize);

        let c_string = std::ffi::CString::from_vec_with_nul(buff).unwrap();
        Err(c_string.to_str().map_err(|err| err.to_string())?.to_owned())
    } else {
        Ok(())
    }
}

// Remember, we need to deallocate our created shaders!
pub unsafe fn create_shader(source: &str, shader_type: gl::types::GLenum) -> Result<u32, String> {
    let vertex_shader = unsafe { gl::CreateShader(shader_type) };
    let vertex_shader_source = std::ffi::CString::new(source).map_err(|e| e.to_string())?;

    unsafe {
        gl::ShaderSource(
            vertex_shader,
            1,
            &(vertex_shader_source.as_ptr() as *const i8) as *const *const i8,
            std::ptr::null(),
        );
        gl::CompileShader(vertex_shader);
        check_shader_compile_errors(vertex_shader)?;
    }
    Ok(vertex_shader)
}

pub unsafe fn create_vbo(verts: &[f32]) -> u32 {
    let mut vbo: u32 = 0;
    let mut vao = 0;
    unsafe {
        // Initialize vbo and vao
        gl::GenVertexArrays(1, &mut vao as *mut u32);
        gl::GenBuffers(1, &mut vbo as *mut u32);
        // These steps need to be done in this order
        // Bind the vertex array
        gl::BindVertexArray(vao);
        // Binding the array buffer, this associates this VBO with this VAO
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // Initialize data in the buffer
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (verts.len() * std::mem::size_of::<f32>()) as isize,
            verts.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW, // data will not change often
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
    vao
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
    }
}

pub unsafe fn link_shader_program<T: IntoIterator<Item = u32> + Copy>(shaders: &T) -> u32 {
    let shader_program = gl::CreateProgram();
    for shader in *shaders {
        gl::AttachShader(shader_program, shader);
    }
    gl::LinkProgram(shader_program);
    check_shader_link_errors(shader_program);

    shader_program
}
