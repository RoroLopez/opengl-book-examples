use std::ffi::CStr;
use std::path::Path;
use std::ptr;
use glam::f32::{Vec3, Mat4};
use glfw::{Context};
use opengl_book_examples::shaders::{Shader, ShaderProgram, ShaderType};
use opengl_book_examples::utils::{handle_window_event};

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
    gl::load_with(|symbol| match window.get_proc_address(symbol) {
        Some(f) => f as *const _,
        None => ptr::null(),
    });

    let vertex_program: &Path = Path::new("src/shaders/vertex/coordinate_systems.vert");
    let fragment_program: &Path = Path::new("src/shaders/fragment/coordinate_system.frag");

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

        let vertices: [f32; 20] = [
            // positions     // texture coords
            0.5,  0.5, 0.0,  1.0, 1.0,   // top right
            0.5, -0.5, 0.0,  1.0, 0.0,   // bottom right
            -0.5, -0.5, 0.0, 0.0, 0.0,   // bottom left
            -0.5,  0.5, 0.0, 0.0, 1.0    // top left
        ];
        let indices: [u32; 6] = [
            0,1,3,
            1,2,3
        ];

        let cube_vertices: [f32; 180] = [
            -0.5, -0.5, -0.5,  0.0, 0.0,
            0.5, -0.5, -0.5,  1.0, 0.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5,  0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 0.0,

            -0.5, -0.5,  0.5,  0.0, 0.0,
            0.5, -0.5,  0.5,  1.0, 0.0,
            0.5,  0.5,  0.5,  1.0, 1.0,
            0.5,  0.5,  0.5,  1.0, 1.0,
            -0.5,  0.5,  0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,

            -0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5,  0.5,  1.0, 0.0,

            0.5,  0.5,  0.5,  1.0, 0.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            0.5, -0.5, -0.5,  0.0, 1.0,
            0.5, -0.5, -0.5,  0.0, 1.0,
            0.5, -0.5,  0.5,  0.0, 0.0,
            0.5,  0.5,  0.5,  1.0, 0.0,

            -0.5, -0.5, -0.5,  0.0, 1.0,
            0.5, -0.5, -0.5,  1.0, 1.0,
            0.5, -0.5,  0.5,  1.0, 0.0,
            0.5, -0.5,  0.5,  1.0, 0.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,

            -0.5,  0.5, -0.5,  0.0, 1.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            0.5,  0.5,  0.5,  1.0, 0.0,
            0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5, -0.5,  0.0, 1.0
        ];

        // Linking Vertex Attributes
        let (mut vbo, mut ebo) = (0, 0);
        gl::GenVertexArrays(1, &mut internal_vao);
        gl::BindVertexArray(internal_vao);

        gl::GenBuffers(1, &mut vbo);
        // gl::GenBuffers(1, &mut ebo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (cube_vertices.len() * size_of::<f32>()) as isize,
            cube_vertices.as_ptr().cast(),
            gl::STATIC_DRAW
        );

        // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        // gl::BufferData(
        //     gl::ELEMENT_ARRAY_BUFFER,
        //     (indices.len() * size_of::<u32>()) as isize,
        //     indices.as_ptr().cast(),
        //     gl::STATIC_DRAW
        // );

        // Position attribute
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            5 * size_of::<f32>() as i32,
            ptr::null()
        );
        gl::EnableVertexAttribArray(0);

        // Texture attribute
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            5 * size_of::<f32>() as i32,
            (3 * size_of::<f32>()) as *const _
        );
        gl::EnableVertexAttribArray(1);

        (internal_vao, internal_texture1, internal_texture2)
    };

    let tex1: &CStr = c"texture1";
    let tex2: &CStr = c"texture2";

    shader_program.use_program();
    shader_program.set_int(tex1, 0);
    shader_program.set_int(tex2, 1);

    unsafe { gl::Enable(gl::DEPTH_TEST); }

    let mut wireframe_mode = false;

    let cube_positions: [Vec3; 10] = [
        Vec3::new( 0.0,  0.0,  0.0),
        Vec3::new( 2.0,  5.0, -15.0),
        Vec3::new(-1.5, -2.2, -2.5),
        Vec3::new(-3.8, -2.0, -12.3),
        Vec3::new( 2.4, -0.4, -3.5),
        Vec3::new(-1.7,  3.0, -7.5),
        Vec3::new( 1.3, -2.0, -2.5),
        Vec3::new( 1.5,  2.0, -2.5),
        Vec3::new( 1.5,  0.2, -1.5),
        Vec3::new(-1.3,  1.0, -1.5)
    ];

    // main loop
    while !window.should_close() {
        // Input
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event, &mut wireframe_mode);
        }

        // let model_matrix: Mat4 = Mat4::IDENTITY * Mat4::from_rotation_x(-45.0);
        // let model_matrix = unsafe {
        //     let angle = 50.0f32.to_radians() * glfw::ffi::glfwGetTime() as f32;
        //     Mat4::IDENTITY * Mat4::from_axis_angle(Vec3::new(0.5, 1.0, 0.0).normalize(), angle)
        // };
        let view_matrix: Mat4 = Mat4::IDENTITY * Mat4::from_translation(Vec3::new(0.0, 0.0, -3.0));
        let projection_matrix: Mat4 = Mat4::perspective_rh_gl(45.0f32.to_radians(), 800.0 / 600.0, 0.1, 100.0);

        // Rendering
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            // let model_cstr: &CStr = c"model";
            // let model_location = gl::GetUniformLocation(shader_program.shader_program_id, model_cstr.as_ptr());
            // gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &model_matrix.to_cols_array()[0]);

            let view_cstr: &CStr = c"view";
            let model_location = gl::GetUniformLocation(shader_program.shader_program_id, view_cstr.as_ptr());
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &view_matrix.to_cols_array()[0]);

            let projection_cstr: &CStr = c"projection";
            let model_location = gl::GetUniformLocation(shader_program.shader_program_id, projection_cstr.as_ptr());
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &projection_matrix.to_cols_array()[0]);

            gl::BindVertexArray(vao);
            for (i, v) in cube_positions.iter().enumerate() {
                let mut angle: f32 = 20.0f32.to_radians() * i as f32;
                if i % 3 == 0 {
                    angle = 20.0f32.to_radians() * (0.5f32 + i as f32) * glfw::ffi::glfwGetTime() as f32;
                }
                let model_matrix = Mat4::IDENTITY * Mat4::from_translation(*v) * Mat4::from_axis_angle(Vec3::new(1.0, 0.3, 0.5).normalize(), angle);

                let model_cstr: &CStr = c"model";
                let model_location = gl::GetUniformLocation(shader_program.shader_program_id, model_cstr.as_ptr());
                gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &model_matrix.to_cols_array()[0]);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
            // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);

        }

        // Check call events and swap the buffers
        glfw.poll_events();
        window.swap_buffers();
    }
}
