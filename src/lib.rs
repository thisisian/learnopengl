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
