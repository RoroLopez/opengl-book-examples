use std::ffi::CStr;
use std::path::Path;
use std::ptr;
use glam::f32::{Vec3, Mat4};
use glfw::Context;
use opengl_book_examples::camera::camera::Camera;
use opengl_book_examples::common::common::{handle_window_event, process_input};
use opengl_book_examples::shaders::shaders::{Shader, ShaderProgram, ShaderType};
use opengl_book_examples::textures::textures::Texture;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3,3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) =
        glfw.create_window(800, 600, "Shaders exercise", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

    // OpenGL state setup
    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);

    // set the cursor at the middle of the screen
    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // GLAD OpenGL function pointers
    gl::load_with(|symbol| match window.get_proc_address(symbol) {
        Some(f) => f as *const _,
        None => ptr::null(),
    });

    // Shader setup
    let vertex_shader_id = match Shader::load_shader(ShaderType::Vertex, Path::new("src/shaders/vertex/coordinate_systems.vert")) {
        Ok(id) => id,
        Err(e) => {
            panic!("{}", e.to_string())
        }
    };
    let fragment_shader_id = match Shader::load_shader(ShaderType::Fragment, Path::new("src/shaders/fragment/coordinate_system.frag")) {
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

    // Texture setup
    let texture1 = match Texture::load_texture(Path::new("src/textures/container.jpg"), false) {
        Ok(id) => id,
        Err(e) => {
            panic!("{}", e.to_string())
        }
    };
    let texture2 = match Texture::load_texture(Path::new("src/textures/calamardo.jpg"), false) {
        Ok(id) => id,
        Err(e) => {
            panic!("{}", e.to_string())
        }
    };

    // VAO setup
    let vao = unsafe {
        let mut internal_vao = 0;

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
        let mut vbo = 0;
        gl::GenVertexArrays(1, &mut internal_vao);
        gl::BindVertexArray(internal_vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (cube_vertices.len() * size_of::<f32>()) as isize,
            cube_vertices.as_ptr().cast(),
            gl::STATIC_DRAW
        );

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

        internal_vao
    };

    shader_program.use_program();
    shader_program.set_int(c"texture1", 0);
    shader_program.set_int(c"texture2", 1);

    unsafe { gl::Enable(gl::DEPTH_TEST); }

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

    let mut delta_time: f32 = 0.0; // Time between current frame and last frame
    let mut last_frame: f32 = 0.0; // Time of last frame
    let mut last_x: f32 = 400.0;
    let mut last_y: f32 = 300.0;

    // Camera setup
    let mut camera: Camera = Camera::new(
        Vec3::new(0.0, 0.0, 3.0),
        Vec3::new(0.0, 1.0, 0.0),
        true
    );

    let mut wireframe_mode = false;
    let mut first_mouse = true;
    // main loop
    while !window.should_close() {
        println!("Camera position: {}", camera.position);
        // Input
        let current_frame: f32 = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(
                &mut window,
                event,
                &mut first_mouse,
                &mut last_x,
                &mut last_y,
                &mut camera,
                &mut wireframe_mode
            );
        }
        process_input(&window, &mut camera, delta_time);

        // Rendering
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            let view_matrix = camera.get_view_matrix();
            let projection_matrix: Mat4 = Mat4::perspective_rh_gl(camera.zoom.to_radians(), 800.0 / 600.0, 0.1, 100.0);

            let view_cstr: &CStr = c"view";
            let model_location = gl::GetUniformLocation(shader_program.shader_program_id, view_cstr.as_ptr());
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &view_matrix.to_cols_array()[0]);

            let projection_cstr: &CStr = c"projection";
            let model_location = gl::GetUniformLocation(shader_program.shader_program_id, projection_cstr.as_ptr());
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &projection_matrix.to_cols_array()[0]);

            gl::BindVertexArray(vao);
            for (i, v) in cube_positions.iter().enumerate() {
                let radius = 10.0;
                let angle: f32 = 10.0f32.to_radians() * (0.5f32 + i as f32) * glfw::ffi::glfwGetTime() as f32;
                // let angle: f32 = 20.0f32.to_radians() * i as f32;
                let rot_x = v.x * angle.cos() - v.z * angle.sin();
                // let rot_z = v.y * angle.cos() - v.z * angle.sin();
                let rot_z = v.x * angle.sin() + v.z * angle.cos();
                let v_rotated = Vec3::new(rot_x, v.y, rot_z).normalize();
                let model_matrix =
                    Mat4::IDENTITY * Mat4::from_translation(*v)
                    * Mat4::from_axis_angle(Vec3::new(0.0, 1.0, 0.0).normalize(), angle)
                    * Mat4::from_translation(-v_rotated)
                    * Mat4::from_translation(*v);

                let model_cstr: &CStr = c"model";
                let model_location = gl::GetUniformLocation(shader_program.shader_program_id, model_cstr.as_ptr());
                gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &model_matrix.to_cols_array()[0]);

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
            gl::BindVertexArray(0);

        }

        // Check call events and swap the buffers
        glfw.poll_events();
        window.swap_buffers();
    }
}
