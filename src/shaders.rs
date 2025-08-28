/// Common structure for loading shaders.
pub mod shaders {
    use std::ffi::{CStr, CString};
    use std::fs::File;
    use std::io::{Error, ErrorKind, Read};
    use std::path::{Path};
    use std::{ptr};
    use gl::types::GLint;

    #[derive(Debug)]
    pub enum ShaderType {
        Vertex,
        Fragment
    }

    pub struct Shader {
        shader_program_id: u32,
        shader_program: CString
    }

    pub struct ShaderProgram {
        pub shader_program_id: u32
    }

    impl Shader {
        /// Load the shaders directly by specifying the file path to the shaders. Will try to compile
        /// and check the status of the compilation if it was successful, it will return the `id` for
        /// the shader loaded.
        pub fn load_shader<P: AsRef<Path>>(shader_type: ShaderType, file_path: P) -> Result<u32, Error> {
            let path: &Path = file_path.as_ref();
            let mut reader: File = match File::open(path) {
                Ok(f) => f,
                Err(e) => {
                    panic!("Error trying to open file {:?}: {}", path.file_name(), e.to_string())
                }
            };
            let mut content: String = String::new();
            match reader.read_to_string(&mut content) {
                Err(e) => {
                    panic!("Error trying to read file's content: {}", e.to_string())
                },
                _ => {}
            };

            let mut shader: Shader = Shader {
                shader_program_id: 0,
                shader_program: CString::new(content.as_bytes())?
            };

            // Create the respective shader based on ShaderType
            match shader_type {
                ShaderType::Vertex => {
                    shader.shader_program_id = shader.create_shader(&shader_type);
                },
                ShaderType::Fragment => {
                    shader.shader_program_id = shader.create_shader(&shader_type);
                }
            };

            // Set the shader source to the object
            shader.set_source_code();
            // Compile shader
            shader.compile_shader();
            // Check if compilation was successful
            let success = shader.get_shader_compilation_status();
            if success == 0 {
                let error_message: String =
                    format!("ERROR::SHADER::{:?}::COMPILATION_FAILED\n{:?}",
                            shader_type, String::from_utf8_lossy(&shader.get_shader_info_log()));
                let error: Error = Error::new(ErrorKind::Other, error_message);
                return Err(error);
            }

            Ok(shader.shader_program_id)
        }

        fn create_shader(&self, shader_type: &ShaderType) -> u32 {
            match shader_type {
                ShaderType::Vertex => {
                    unsafe { gl::CreateShader(gl::VERTEX_SHADER) }
                },
                ShaderType::Fragment => {
                    unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) }
                }
            }
        }

        fn set_source_code(&self) {
            unsafe {
                gl::ShaderSource(
                    self.shader_program_id,
                    1,
                    &self.shader_program.as_ptr(),
                    ptr::null()
                );
            }
        }

        fn compile_shader(&self) {
            unsafe {
                gl::CompileShader(self.shader_program_id);
            }
        }

        fn get_shader_compilation_status(&self) -> u32 {
            let mut success: GLint = 0;
            unsafe {
                gl::GetShaderiv(self.shader_program_id, gl::COMPILE_STATUS, &mut success);
            }
            success as u32
        }

        fn get_shader_info_log(&self) -> Vec<u8> {
            let mut needed_len = 0;
            unsafe { gl::GetShaderiv(self.shader_program_id, gl::INFO_LOG_LENGTH, &mut needed_len); }
            let mut info_log: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
            let mut len_written = 0_i32;
            unsafe {
                gl::GetShaderInfoLog(
                    self.shader_program_id,
                    info_log.capacity().try_into().unwrap(),
                    &mut len_written,
                    info_log.as_mut_ptr().cast()
                );
                info_log.set_len(len_written.try_into().unwrap());
            }
            info_log
        }
    }

    impl ShaderProgram {
        /// New instance of the `ShaderProgram`
        pub fn new() -> Self {
            ShaderProgram { shader_program_id: Self::create_shader() }
        }

        /// Will build the shader program and link the shader program for the given list of shaders.
        /// It will attach them in the order the list of shaders is given. If successful, `ShaderProgram`
        /// is ready to use, otherwise it will return an `Error` and you can check the error message.
        /// It also deletes the given shaders after successful linking.
        pub fn build(&self, shaders: &[u32]) -> Result<(), Error> {
            for shader in shaders {
                self.attach_shader(*shader);
            }
            self.link_program();
            let success = self.get_program_link_status();
            if success == 0 {
                let error_message: String =
                    format!("ERROR::SHADER::Program::COMPILATION_FAILED\n{:?}",
                            String::from_utf8_lossy(&self.get_program_info_log()));
                let error: Error = Error::new(ErrorKind::Other, error_message);
                return Err(error);
            }
            for shader in shaders {
                self.delete_shader(*shader);
            }
            Ok(())
        }

        /// Uses the shader assuming no error was raised during build
        pub fn use_program(&self) {
            unsafe {
                gl::UseProgram(self.shader_program_id);
            }
        }

        pub fn set_bool(&self, name: &CStr, value: bool) {
            unsafe {
                gl::Uniform1i(
                    gl::GetUniformLocation(self.shader_program_id, name.as_ptr()),
                    value.into())
            }
        }

        pub fn set_int(&self, name: &CStr, value: u32) {
            unsafe {
                gl::Uniform1i(
                    gl::GetUniformLocation(self.shader_program_id, name.as_ptr()),
                    value as GLint)
            }
        }

        pub fn get_int(&self, name: &CStr) -> i32 {
            unsafe {
                let location: i32 = gl::GetUniformLocation(self.shader_program_id, name.as_ptr());
                let mut value: i32 = 0;
                gl::GetUniformiv(self.shader_program_id, location, &mut value);
                value
            }
        }

        pub fn set_float(&self, name: &CStr, value: f32) {
            unsafe {
                gl::Uniform1f(
                    gl::GetUniformLocation(self.shader_program_id, name.as_ptr()),
                    value)
            }
        }

        pub fn get_float(&self, name: &CStr) -> f32 {
            unsafe {
                let location: i32 = gl::GetUniformLocation(self.shader_program_id, name.as_ptr());
                let mut value: f32 = 0.0;
                gl::GetUniformfv(self.shader_program_id, location, &mut value);
                value
            }
        }

        pub fn set_float4(&self, name: &CStr, values: (f32, f32, f32, f32)) {
            unsafe {
                gl::Uniform4f(
                    gl::GetUniformLocation(self.shader_program_id, name.as_ptr()),
                    values.0, values.1, values.2, values.3)
            }
        }

        fn delete_shader(&self, shader: u32) {
            unsafe { gl::DeleteShader(shader); }
        }

        fn attach_shader(&self, shader: u32) {
            unsafe { gl::AttachShader(self.shader_program_id, shader); }
        }

        fn get_program_info_log(&self) -> Vec<u8> {
            let mut needed_len = 0;
            unsafe { gl::GetProgramiv(self.shader_program_id, gl::INFO_LOG_LENGTH, &mut needed_len); }
            let mut info_log: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
            let mut len_written = 0_i32;
            unsafe {
                gl::GetProgramInfoLog(
                    self.shader_program_id,
                    info_log.capacity().try_into().unwrap(),
                    &mut len_written,
                    info_log.as_mut_ptr().cast()
                );
                info_log.set_len(len_written.try_into().unwrap());
            }
            info_log
        }

        fn get_program_link_status(&self) -> u32 {
            let mut success: GLint = 0;
            unsafe {
                gl::GetProgramiv(self.shader_program_id, gl::LINK_STATUS, &mut success);
            }
            success as u32
        }

        fn link_program(&self) {
            unsafe { gl::LinkProgram(self.shader_program_id); }
        }

        fn create_shader() -> u32 {
            unsafe { gl::CreateProgram() }
        }
    }
}