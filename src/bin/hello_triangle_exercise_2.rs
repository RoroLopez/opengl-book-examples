use glfw::{Context};

use std::ffi::{CString};
use std::{ptr};
use gl::types::{GLsizei};
use opengl_book_examples::common::common::handle_window_event_original;

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
        FragColor = vec4(1.0f, 1.0f, 0.0f, 1.0f);
    }
"#;

const YELLOW_FRAGMENT_SHADER_SOURCE: &str = r#"
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
    gl::load_with(|symbol| match window.get_proc_address(symbol) {
        Some(f) => f as *const _,
        None => std::ptr::null(),
    });

    let c_str_vert = CString::new(VERTEX_SHADER_SOURCE.as_bytes()).unwrap();
    let c_str_frag = CString::new(FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();
    let c_str_yellow_frag = CString::new(YELLOW_FRAGMENT_SHADER_SOURCE.as_bytes()).unwrap();

    let mut vaos = [0; 2];
    let (mut shader_program1, mut shader_program2) = (0, 0);
    unsafe {
        let (mut vertex_shader, mut fragment_shader, mut yellow_fragment_shader) = (0, 0, 0);
        // Vertex shader compilation and error checking
        vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
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

        success = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut frag_info_log: Vec<u8> = Vec::with_capacity(1024);
            gl::GetShaderInfoLog(
                fragment_shader,
                1024,
                ptr::null_mut(),
                frag_info_log.as_mut_ptr().cast()
            );
            println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&frag_info_log).unwrap())
        }

        // Yellow fragment shader compilation and error checking
        yellow_fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(yellow_fragment_shader, 1, &c_str_yellow_frag.as_ptr(), ptr::null());
        gl::CompileShader(yellow_fragment_shader);

        success= 0;
        gl::GetShaderiv(yellow_fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut frag_info_log: Vec<u8> = Vec::with_capacity(1024);
            gl::GetShaderInfoLog(
                yellow_fragment_shader,
                1024,
                ptr::null_mut(),
                frag_info_log.as_mut_ptr().cast()
            );
            println!("ERROR::SHADER::YELLOW_FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&frag_info_log).unwrap())
        }

        // Shader program compilation and error checking
        shader_program1 = gl::CreateProgram();
        gl::AttachShader(shader_program1, vertex_shader);
        gl::AttachShader(shader_program1, fragment_shader);
        gl::LinkProgram(shader_program1);

        let mut success_shader = 0;
        gl::GetProgramiv(shader_program1, gl::LINK_STATUS, &mut success_shader);
        if success_shader == 0 {
            let mut shader_info_log: Vec<u8> = Vec::with_capacity(1024);
            gl::GetProgramInfoLog(
                shader_program1,
                1024,
                ptr::null_mut(),
                shader_info_log.as_mut_ptr().cast()
            );
            println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&shader_info_log).unwrap())
        }

        // Second shader program compilation and error checking
        shader_program2 = gl::CreateProgram();
        gl::AttachShader(shader_program2, vertex_shader);
        gl::AttachShader(shader_program2, yellow_fragment_shader);
        gl::LinkProgram(shader_program2);

        success_shader = 0;
        gl::GetProgramiv(shader_program2, gl::LINK_STATUS, &mut success_shader);
        if success_shader == 0 {
            let mut shader_info_log: Vec<u8> = Vec::with_capacity(1024);
            gl::GetProgramInfoLog(
                shader_program2,
                1024,
                ptr::null_mut(),
                shader_info_log.as_mut_ptr().cast()
            );
            println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&shader_info_log).unwrap())
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
        gl::DeleteShader(yellow_fragment_shader);

        let t_upper_vertices: [f32; 18] = [
            -0.2, 0.6, 0.0,
            -0.2, 0.5, 0.0,
            0.3, 0.5, 0.0,
            0.0, 0.5, 0.0,
            0.1, 0.0, 0.0,
            0.1, 0.5, 0.0
        ];

        let t_lower_vertices: [f32; 18] = [
            0.0, 0.0, 0.0,
            0.0, 0.5, 0.0,
            0.1, 0.0, 0.0,
            0.3, 0.6, 0.0,
            0.3, 0.5, 0.0,
            -0.2, 0.6, 0.0
        ];

        let t_upper_vertices2: [f32; 18] = [
            -0.2, 0.6, 0.0,
            -0.2, 0.5, 0.0,
            0.3, 0.5, 0.0,
            0.3, 0.6, 0.0,
            0.3, 0.5, 0.0,
            -0.2, 0.6, 0.0
        ];
        let t_lower_vertices2: [f32; 18] = [
            0.0, 0.0, 0.0,
            0.0, 0.5, 0.0,
            0.1, 0.0, 0.0,
            0.0, 0.5, 0.0,
            0.1, 0.0, 0.0,
            0.1, 0.5, 0.0
        ];

        let mut vbos = [0; 2];

        gl::GenVertexArrays(2, vaos.as_mut_ptr());
        gl::GenBuffers(2, vbos.as_mut_ptr());

        // First part of T
        gl::BindVertexArray(vaos[0]);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbos[0]);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (t_upper_vertices.len() * size_of::<f32>()) as isize,
            t_upper_vertices.as_ptr().cast(),
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

        //Second part of T
        gl::BindVertexArray(vaos[1]);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbos[1]);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (t_lower_vertices.len() * size_of::<f32>()) as isize,
            t_lower_vertices.as_ptr().cast(),
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
        // Input
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event_original(&mut window, event, &mut wireframe_mode);
        }

        // Rendering
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            //Upper part of T
            gl::UseProgram(shader_program1);
            gl::BindVertexArray(vaos[0]);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);

            gl::UseProgram(shader_program2);
            //Lower part of T
            gl::BindVertexArray(vaos[1]);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            // gl::BindVertexArray(0);
        }

        // Check call events and swap the buffers
        glfw.poll_events();
        window.swap_buffers();
    }
}
