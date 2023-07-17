use std::env;
use std::ffi::c_void;

#[repr(u32)]
pub enum ShaderType {
    VertexShader = gl::VERTEX_SHADER,
    FragmentShader = gl::FRAGMENT_SHADER,
}

pub struct Texture {
    pub id: u32,
}

impl Texture {
    pub unsafe fn new(name: &str) -> Result<Self, String> {
        let path = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("textures")
            .join(name);

        if !path.exists() {
            return Err(format!("Texture file not found: {}", path.display()));
        }

        let img = image::io::Reader::open(path)
            .map_err(|x| x.to_string())?
            .decode()
            .map_err(|x| x.to_string())?
            .flipv();

        let mut id = 0;
        gl::GenTextures(1, &mut id);
        gl::BindTexture(gl::TEXTURE_2D, id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        let rgb8 = img.into_rgb8();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            rgb8.width() as i32,
            rgb8.height() as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            rgb8.as_ptr() as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        gl::BindTexture(gl::TEXTURE_2D, 0);
        Ok(Texture { id })
    }

    pub unsafe fn new_rgba(name: &str) -> Result<Self, String> {
        let path = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("textures")
            .join(name);

        if !path.exists() {
            return Err(format!("Texture file not found: {}", path.display()));
        }

        let img = image::io::Reader::open(path)
            .map_err(|x| x.to_string())?
            .decode()
            .map_err(|x| x.to_string())?
            .flipv();

        let mut id = 0;
        gl::GenTextures(1, &mut id);
        gl::BindTexture(gl::TEXTURE_2D, id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        let rgba8 = img.into_rgba8();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            rgba8.width() as i32,
            rgba8.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            rgba8.as_ptr() as *const c_void,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        gl::BindTexture(gl::TEXTURE_2D, 0);
        Ok(Texture { id })
    }
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

    pub unsafe fn set_uniform_i32(&self, name: &str, value: i32) -> Result<(), String> {
        let location = self.get_uniform_location(name)?;
        self.use_program();
        gl::Uniform1i(location, value);
        check_gl_error()?;
        Ok(())
    }

    pub unsafe fn set_uniform_f32(&self, name: &str, value: f32) -> Result<(), String> {
        let location = self.get_uniform_location(name)?;
        self.use_program();
        gl::Uniform1f(location, value);
        check_gl_error()?;
        Ok(())
    }

    pub unsafe fn set_uniform_mat4(&self, name: &str, value: &glam::Mat4) -> Result<(), String> {
        let location = self.get_uniform_location(name)?;
        self.use_program();
        gl::UniformMatrix4fv(location, 1, gl::FALSE, &value.to_cols_array()[0]);
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

// VAO with a vertex attribute and a color attribute
pub unsafe fn create_vao(verts: &[f32]) -> u32 {
    let mut vbo = 0;
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
        // Describe the kind of data we're passing to location 0
        gl::VertexAttribPointer(
            0, // we want to bind this attribute to position 0
            3, // each vertex is three floats long
            gl::FLOAT,
            gl::FALSE, // do not normalize data points between [-1.0, 1.0]
            (std::mem::size_of::<f32>() * 5) as i32, // stride 0 defaults to width of each vertex without additional data
            0 as *const c_void,
        );

        // Enable the attribute.
        gl::EnableVertexAttribArray(0);

        // texture coordinate attributes
        gl::VertexAttribPointer(
            1, // we want to bind this attribute to position 1
            2, // each vertex is two floats long
            gl::FLOAT,
            gl::FALSE, // do not normalize data points between [-1.0, 1.0]
            (std::mem::size_of::<f32>() * 5) as i32, // stride 0 defaults to width of each vertex without additional data
            (std::mem::size_of::<f32>() * 3) as *const c_void, // first data starts at 3rd float value
        );
        // Enable the attribute.
        gl::EnableVertexAttribArray(1);

        // unbind buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
    vao
}

pub unsafe fn create_vao_indices(verts: &[f32], indices: &[u32]) -> u32 {
    let mut vbo = 0;
    let mut vao = 0;
    let mut ebo = 0;
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

        // Creating the EBO buffer
        gl::GenBuffers(1, &mut ebo as *mut u32);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as isize,
            indices.as_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );

        // Configure attributes.
        // Describe the kind of data we're passing to location 0
        gl::VertexAttribPointer(
            0, // we want to bind this attribute to position 0
            3, // each vertex is three floats long
            gl::FLOAT,
            gl::FALSE, // do not normalize data points between [-1.0, 1.0]
            (std::mem::size_of::<f32>() * 8) as i32, // stride 0 defaults to width of each vertex without additional data
            0 as *const c_void,
        );

        // Enable the attribute.
        gl::EnableVertexAttribArray(0);

        // color attributes
        gl::VertexAttribPointer(
            1, // we want to bind this attribute to position 1
            3, // each vertex is three floats long
            gl::FLOAT,
            gl::FALSE, // do not normalize data points between [-1.0, 1.0]
            (std::mem::size_of::<f32>() * 8) as i32, // stride 0 defaults to width of each vertex without additional data
            (std::mem::size_of::<f32>() * 3) as *const c_void, // first data starts at 3rd float value
        );
        // Enable the attribute.
        gl::EnableVertexAttribArray(1);

        // texture coordinate attributes
        gl::VertexAttribPointer(
            2, // we want to bind this attribute to position 2
            2, // each vertex is two floats long
            gl::FLOAT,
            gl::FALSE, // do not normalize data points between [-1.0, 1.0]
            (std::mem::size_of::<f32>() * 8) as i32, // stride 0 defaults to width of each vertex without additional data
            (std::mem::size_of::<f32>() * 6) as *const c_void,
        );
        // Enable the attribute.
        gl::EnableVertexAttribArray(2);

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

pub struct Camera {
    position: glam::Vec3,
    front: glam::Vec3,
    up: glam::Vec3,
    right: glam::Vec3,
    world_up: glam::Vec3,
    yaw: f32,
    pitch: f32,
    movement_speed: f32,
    mouse_sensitivity: f32,
    pub zoom: f32,
}

pub enum CameraDirection {
    Forward,
    Backward,
    Left,
    Right,
}

impl Camera {
    pub fn new() -> Self {
        let mut camera = Camera {
            position: glam::vec3(0.0, 0.0, -3.0),
            front: glam::vec3(0.0, 0.0, 0.0),
            up: glam::vec3(0.0, 1.0, 0.0),
            right: glam::vec3(1.0, 0.0, 0.0),
            world_up: glam::vec3(0.0, 1.0, 0.0),
            yaw: -90.0,
            pitch: 0.0,
            movement_speed: 2.5,
            mouse_sensitivity: 0.1,
            zoom: 45.0,
        };
        camera.update_camera_vectors();
        camera
    }

    pub fn get_view_matrix(&self) -> glam::Mat4 {
        glam::Mat4::look_at_rh(self.position, self.position + self.front, self.up)
    }

    pub fn process_keyboard(
        &mut self,
        camera_direction: CameraDirection,
        delta_time: std::time::Duration,
    ) {
        let velocity = delta_time.as_secs_f32() * self.movement_speed;
        match camera_direction {
            CameraDirection::Forward => self.position += self.front * velocity,
            CameraDirection::Backward => self.position -= self.front * velocity,
            CameraDirection::Left => self.position -= self.right * velocity,
            CameraDirection::Right => self.position += self.right * velocity,
        }
    }

    pub fn process_mouse_movement(&mut self, x_offset: i32, y_offset: i32) {
        self.yaw += x_offset as f32 * self.mouse_sensitivity;
        self.pitch = (self.pitch - y_offset as f32 * self.mouse_sensitivity).clamp(-89.9, 89.9);
        self.update_camera_vectors();
    }

    fn update_camera_vectors(&mut self) {
        self.front = glam::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        )
        .normalize();
        self.right = self.front.cross(self.world_up).normalize();
        self.up = self.right.cross(self.front).normalize();
    }

    pub fn process_mouse_scroll(&mut self, y_offset: i32) {
        self.zoom = (self.zoom - y_offset as f32).clamp(1.0, 45.0);
    }
}
