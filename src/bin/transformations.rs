use glam::f32::{Vec3, Mat4};
use std::ffi::CString;
use glfw::{Action, Context, Key};
use opengl_book_examples::shaders::{Shader, ShaderProgram, ShaderType};
use std::path::Path;
use std::ptr;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4,1));
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
    gl::load_with(|symbol| match window.get_proc_address(symbol) {
        Some(f) => f as *const _,
        None => ptr::null(),
    });

    let vertex_program: &Path = Path::new("src/shaders/vertex/transformation.vert");
    let fragment_program: &Path = Path::new("src/shaders/fragment/texture.frag");

    // let mut trans = Mat4::IDENTITY;
    // trans = trans * Mat4::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), std::f32::consts::PI / 2.0);
    // trans = trans * Mat4::from_scale(Vec3::new(0.5, 0.5, 0.5));

    let vertex_shader_id = match Shader::load_shader(ShaderType::Vertex, vertex_program) {
        Ok(id) => id,
        Err(e) => {
            panic!("{}", e.to_string())
        }
    };
    let fragment_shader_id = match Shader::load_shader(ShaderType::Fragment, fragment_program) {
        Ok(id) => id,
        Err(e) => {
            panic!("{}", e.to_string())
        }
    };

    let shader_program: ShaderProgram = ShaderProgram::new();
    match shader_program.build(&[vertex_shader_id, fragment_shader_id]) {
        Err(e) => {
            panic!("{}", e.to_string())
        },
        _ => {}
    };

    let (vao, texture1, texture2) = unsafe {
        let (mut internal_vao, mut internal_texture1, mut internal_texture2) = (0, 0, 0);

        let vertices: [f32; 32] = [
            // positions     // colors      // texture coords
            0.5,  0.5, 0.0,  1.0, 0.0, 0.0, 1.0, 1.0,   // top right
            0.5, -0.5, 0.0,  0.0, 1.0, 0.0, 1.0, 0.0,   // bottom right
            -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,   // bottom left
            -0.5,  0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0    // top left
        ];
        let indices: [u32; 6] = [
            0,1,3,
            1,2,3
        ];

        // Texture configuration
        gl::GenTextures(1, &mut internal_texture1);
        gl::BindTexture(gl::TEXTURE_2D, internal_texture1);
        // set the texture wrapping/filtering options (on the currently bound texture object)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        // load and generate the texture
        let img_source: String = "src/textures/container.jpg".to_string();
        let img = match image::open(img_source) {
            Ok(img) => img.rotate180(),
            Err(e) => {
                panic!("Failed to load image: {}", e.to_string())
            }
        };

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            img.width() as i32,
            img.height() as i32,
            0,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            img.as_bytes().as_ptr().cast()
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // Texture configuration
        gl::GenTextures(1, &mut internal_texture2);
        gl::BindTexture(gl::TEXTURE_2D, internal_texture2);
        // set the texture wrapping/filtering options (on the currently bound texture object)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        // load and generate the texture
        let img_source2: String = "src/textures/awesomeface.png".to_string();
        let img2 = match image::open(img_source2) {
            Ok(img) => img.flipv(),
            Err(e) => {
                panic!("Failed to load image: {}", e.to_string())
            }
        };

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            img2.width() as i32,
            img2.height() as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img2.as_bytes().as_ptr().cast()
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // Linking Vertex Attributes
        let (mut vbo, mut ebo) = (0, 0);
        gl::GenVertexArrays(1, &mut internal_vao);
        gl::BindVertexArray(internal_vao);

        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<f32>()) as isize,
            vertices.as_ptr().cast(),
            gl::STATIC_DRAW
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * size_of::<u32>()) as isize,
            indices.as_ptr().cast(),
            gl::STATIC_DRAW
        );

        // Position attribute
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            8 * size_of::<f32>() as i32,
            ptr::null()
        );
        gl::EnableVertexAttribArray(0);

        // Color attribute
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            8 * size_of::<f32>() as i32,
            (3 * size_of::<f32>()) as *const _
        );
        gl::EnableVertexAttribArray(1);

        // Texture attribute
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            8 * size_of::<f32>() as i32,
            (6 * size_of::<f32>()) as *const _
        );
        gl::EnableVertexAttribArray(2);

        (internal_vao, internal_texture1, internal_texture2)
    };

    let tex1: CString = CString::new("texture1").unwrap();
    let tex2: CString = CString::new("texture2").unwrap();

    unsafe { gl::UseProgram(shader_program.shader_program_id); }
    shader_program.set_int(&tex1, 0);
    shader_program.set_int(&tex2, 1);

    // let name = CString::new("transform").unwrap();
    // unsafe {
    //     let location = gl::GetUniformLocation(shader_program.shader_program_id, name.as_ptr());
    //     gl::UniformMatrix4fv(location, 1, gl::FALSE, &trans.to_cols_array()[0]);
    // };

    while !window.should_close() {
        // Input
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event, &shader_program);
        }

        // Rendering
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            let mut trans = Mat4::IDENTITY;
            let time = unsafe {
                glfw::ffi::glfwGetTime() as f32
            };
            trans = trans * Mat4::from_translation(Vec3::new(0.5, -0.5, 0.0));
            trans = trans * Mat4::from_rotation_z(time);

            let name = CString::new("transform").unwrap();
            let location = gl::GetUniformLocation(shader_program.shader_program_id, name.as_ptr());
            gl::UniformMatrix4fv(location, 1, gl::FALSE, &trans.to_cols_array()[0]);
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

            trans = Mat4::IDENTITY;
            trans = trans * Mat4::from_translation(Vec3::new(-0.5, 0.5, 0.0));
            let scale_amount: f32 = f32::sin(glfw::ffi::glfwGetTime() as f32);
            trans = trans * Mat4::from_scale(Vec3::new(scale_amount, scale_amount, scale_amount));
            gl::UniformMatrix4fv(location, 1, gl::FALSE, &trans.to_cols_array()[0]);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

            gl::BindVertexArray(0);
        }

        // Check call events and swap the buffers
        glfw.poll_events();
        window.swap_buffers();
    }
}

pub fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent, shader_program: &ShaderProgram) {
    match event {
        glfw::WindowEvent::FramebufferSize(width, height) => {
            unsafe { gl::Viewport(0, 0, width, height) }
        }
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        glfw::WindowEvent::Key(Key::Up, _, Action::Press, _) => {
            let visibility: CString = CString::new("visibility").unwrap();
            let mut value = shader_program.get_float(&visibility);
            value += 0.1;
            if value <= 1.0 {
                shader_program.set_float(visibility, value);
            }
        }
        glfw::WindowEvent::Key(Key::Down, _, Action::Press, _) => {
            let visibility: CString = CString::new("visibility").unwrap();
            let mut value = shader_program.get_float(&visibility);
            value -= 0.1;
            if value >= 0.0 {
                shader_program.set_float(visibility, value);
            }
        }
        _ => {}
    }
}
