use std::ffi::c_void;
#[repr(u32)]
pub enum ShaderType {
    VertexShader = gl::VERTEX_SHADER,
    FragmentShader = gl::FRAGMENT_SHADER,
}

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub unsafe fn from_str(source: &str, shader_type: ShaderType) -> Result<Self, String> {
        let id = unsafe { gl::CreateShader(shader_type as u32) };
        let c_shader_source = std::ffi::CString::new(source).map_err(|e| e.to_string())?;

        gl::ShaderSource(
            id,
            1,
            &(c_shader_source.as_ptr() as *const i8) as *const *const i8,
            std::ptr::null(),
        );
        gl::CompileShader(id);
        check_shader_compile_errors(id)?;
        Ok(Shader { id })
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id) }
    }
}

pub struct ShaderProgram {
    pub id: u32,
}

impl ShaderProgram {
    pub unsafe fn new() -> Self {
        let id = gl::CreateProgram();
        ShaderProgram { id }
    }

    pub unsafe fn attach_shader(&self, shader: Shader) -> () {
        gl::AttachShader(self.id, shader.id);
    }

    pub unsafe fn link_program(&self) -> Result<(), String> {
        gl::LinkProgram(self.id);
        check_shader_link_errors(self.id)?;
        Ok(())
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id);
    }

    unsafe fn get_uniform_location(&self, name: &str) -> Result<i32, String> {
        let cstr = std::ffi::CString::new(name).unwrap();
        let location = gl::GetUniformLocation(self.id, cstr.as_ptr() as *const i8);
        if location == -1 {
            Err(format!("Failed to find location of uniform {}", name))
        } else {
            Ok(location)
        }
    }

    pub unsafe fn set_uniform_f64(&self, name: &str, value: f32) -> Result<(), String> {
        let location = self.get_uniform_location(name)?;
        self.use_program();
        gl::Uniform1f(location, value);
        check_gl_error()?;
        Ok(())
    }
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
            (std::mem::size_of::<f32>() * 6) as i32, // stride 0 defaults to width of each vertex without additional data
            0 as *const c_void,
        );

        // Enable the attribute.
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1, // we want to bind this attribute to position 0
            3, // each vertex is three floats long
            gl::FLOAT,
            gl::FALSE, // do not normalize data points between [-1.0, 1.0]
            (std::mem::size_of::<f32>() * 6) as i32, // stride 0 defaults to width of each vertex without additional data
            (std::mem::size_of::<f32>() * 3) as *const c_void,
        );
        // Enable the attribute.
        gl::EnableVertexAttribArray(1);

        // unbind buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
    vao
}

unsafe fn check_shader_link_errors(shader: u32) -> Result<(), String> {
    let mut success: i32 = 0;
    gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success as *mut i32);
    if success == gl::FALSE as i32 {
        let mut log_length: i32 = 0;
        gl::GetProgramiv(shader, gl::INFO_LOG_LENGTH, &mut log_length as *mut i32);

        let mut buff = Vec::<u8>::with_capacity(log_length as usize);
        gl::GetProgramInfoLog(
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
