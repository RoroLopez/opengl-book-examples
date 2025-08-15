use glfw::{Context};

use std::ffi::{CStr, CString};
use std::{ptr};
use gl::types::{GLsizei};
use opengl_book_examples::utils::handle_window_event;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main()
    {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main()
    {
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3,3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true)); 

    let (mut window, events) = glfw.create_window(800, 600, "Hello, this is MY window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.make_current();

    // GLAD OpenGL function pointers
    // gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    gl::load_with(|symbol| match window.get_proc_address(symbol) {
        Some(f) => f as *const _,
        None => std::ptr::null(),
    });

    // Exploring...
    unsafe {
        let version = gl::GetString(gl::VERSION);
        if !version.is_null() {
            let version_cstr = CStr::from_ptr(version as *const i8);
            println!("OpenGL version: {}", version_cstr.to_str().unwrap());
        } else {
            println!("glGetString(GL_VERSION) returned null");
        }
    }

    let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
    let (mut vertex_shader, mut fragment_shader, mut shader_program) = (0, 0, 0);
    let c_str_vert = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
    let c_str_frag = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
    unsafe {
        // Vertex shader compilation and error checking
        vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        let mut success_vertex = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success_vertex);
        if success_vertex == 0 {
            let mut vertex_info_log: Vec<u8> = Vec::with_capacity(1024);
            gl::GetShaderInfoLog(
                vertex_shader,
                1024,
                ptr::null_mut(),
                vertex_info_log.as_mut_ptr().cast()
            );
            println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&vertex_info_log).unwrap())
        }

        // Fragment shader compilation and error checking
        fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        let mut success_frag = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success_frag);
        if success_frag == 0 {
            let mut frag_info_log: Vec<u8> = Vec::with_capacity(1024);
            gl::GetShaderInfoLog(
                vertex_shader,
                1024,
                ptr::null_mut(),
                frag_info_log.as_mut_ptr().cast()
            );
            println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&frag_info_log).unwrap())
        }

        // Shader program compilation and error checking
        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        let mut success_shader = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success_shader);
        if success_shader == 0 {
            let mut shader_info_log: Vec<u8> = Vec::with_capacity(1024);
            gl::GetProgramInfoLog(
                vertex_shader,
                1024,
                ptr::null_mut(),
                shader_info_log.as_mut_ptr().cast()
            );
            println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&shader_info_log).unwrap())
        }

        let vertices: [f32; 9] = [
            -0.5, -0.5, 0.0,
            0.5, -0.5, 0.0,
            0.0,  0.5, 0.0
        ];

        let t_vertices: [f32; 36] = [
            -0.2, 0.6, 0.0,
            -0.2, 0.5, 0.0,
            0.3, 0.5, 0.0,
            0.3, 0.6, 0.0,
            0.3, 0.5, 0.0,
            -0.2, 0.6, 0.0,
            0.0, 0.0, 0.0,
            0.0, 0.5, 0.0,
            0.1, 0.0, 0.0,
            0.0, 0.5, 0.0,
            0.1, 0.0, 0.0,
            0.1, 0.5, 0.0
        ];

        let rectangle_vertices: [f32; 12] = [
            0.5,  0.5, 0.0,  // top right
            0.5, -0.5, 0.0,  // bottom right
            -0.5, -0.5, 0.0,  // bottom left
            -0.5,  0.5, 0.0   // top left
        ];
        let indices: [u32; 6] = [
            0,1,3,
            1,2,3
        ];

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        // Linking Vertex Attributes
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            // size_of_val(&vertices) as isize, // use only when size of value is not known at compile time
            (rectangle_vertices.len() * size_of::<f32>()) as isize,
            rectangle_vertices.as_ptr().cast(),
            gl::STATIC_DRAW
        );

        // for rendering rectangle ONLY
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * size_of::<u32>()) as isize,
            indices.as_ptr().cast(),
            gl::STATIC_DRAW
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * size_of::<f32>() as GLsizei,
            ptr::null()
        );
        gl::EnableVertexAttribArray(0);
    }

    let mut wireframe_mode: bool = false;
    while !window.should_close() {
        // println!("{:?}",window.get_size());

        // Input
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event, &mut wireframe_mode);
        }

        // Rendering
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            // gl::DrawArrays(gl::TRIANGLES, 0, 12);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);
        }

        // Check call events and swap the buffers
        glfw.poll_events();
        window.swap_buffers();
    }
}
