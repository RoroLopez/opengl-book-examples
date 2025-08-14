use std::ffi::CString;
use std::ptr;
use gl::types::GLsizei;
use glfw::{Action, Context, Key};
use glfw::ffi::glfwGetTime;

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;
    out vec3 ourColor;
    void main()
    {
        gl_Position = vec4(aPos, 1.0);
        ourColor = aColor;
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    in vec3 ourColor;
    // uniform vec4 ourColor;
    void main()
    {
        FragColor = vec4(ourColor, 1.0);
        // FragColor = ourColor; // uniform example
    }
"#;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3,3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) =
        glfw.create_window(800, 600, "Shaders exercise", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);
    window.make_current();

    // GLAD OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Experiments
    unsafe {
        let mut nr_attributes = 0;
        gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut nr_attributes);
        println!("Maximum number of vertex attributes supported: {}", nr_attributes);
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
            let mut vertex_info_log: Vec<u8> = Vec::with_capacity(512);
            gl::GetShaderInfoLog(
                vertex_shader,
                512,
                ptr::null_mut(),
                vertex_info_log.as_mut_ptr().cast()
            );
            println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&vertex_info_log));
        }

        // Fragment shader compilation and error checking
        fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        let mut success_frag = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success_frag);
        if success_frag == 0 {
            let mut frag_info_log: Vec<u8> = Vec::with_capacity(512);
            gl::GetShaderInfoLog(
                fragment_shader,
                512,
                ptr::null_mut(),
                frag_info_log.as_mut_ptr().cast()
            );
            println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&frag_info_log));
        }

        // Shader program compilation and error checking
        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        let mut success_shader = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success_shader);
        if success_shader == 0 {
            let mut shader_info_log: Vec<u8> = Vec::with_capacity(512);
            gl::GetProgramInfoLog(
                shader_program,
                512,
                ptr::null_mut(),
                shader_info_log.as_mut_ptr().cast()
            );
            println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", String::from_utf8_lossy(&shader_info_log))
        }
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        let vertices: [f32; 18] = [
            // position      // colors
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
             0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
             0.0,  0.5, 0.0, 0.0, 0.0, 1.0
        ];

        let original_vertices: [f32; 9] = [
            0.5, -0.5, 0.0,
            -0.5, -0.5, 0.0,
            0.0,  0.5, 0.0
        ];

        // Linking Vertex Attributes
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            // size_of_val(&vertices) as isize, // use only when size of value is not known at compile time
            (vertices.len() * size_of::<f32>()) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW
        );

        // Position attribute
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * size_of::<f32>() as GLsizei,
            ptr::null()
        );
        gl::EnableVertexAttribArray(0);

        // Color attribute
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * size_of::<f32>() as GLsizei,
            (3 * size_of::<f32>()) as *const _
        );
        gl::EnableVertexAttribArray(1);
    }

    while !window.should_close() {
        // Input
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        // Rendering
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);

            // update the uniform color
            // let time_value = glfwGetTime();
            // let green_value = (time_value.sin() / 2.0 + 0.5) as f32;
            // let our_color = CString::new("ourColor").unwrap();
            // let vertex_color_location = gl::GetUniformLocation(shader_program, our_color.as_ptr());
            // gl::Uniform4f(vertex_color_location, 0.0, green_value, 0.0, 1.0);

            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            // gl::BindVertexArray(0);
        }

        // Check call events and swap the buffers
        glfw.poll_events();
        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::FramebufferSize(width, height) => {
            unsafe { gl::Viewport(0, 0, width, height) }
        }
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}