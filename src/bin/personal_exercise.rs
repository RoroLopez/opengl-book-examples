use std::path::Path;
use std::ptr;
use glam::{Mat4, Vec3};
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
    let fragment_shader_id = match Shader::load_shader(ShaderType::Fragment, Path::new("src/shaders/fragment/personal_exercise.frag")) {
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
    let gravel_floor = match Texture::load_texture(Path::new("src/textures/gravel-concrete.jpg"), false) {
        Ok(id) => id,
        Err(e) => {
            panic!("{}", e.to_string())
        }
    };

    let vao = unsafe {
        let mut internal_vao = 0;

        let vertices: [f32; 30] = [
            -0.5, -0.5, -0.5,  0.0, 0.0,
            0.5, -0.5, -0.5,  1.0, 0.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5,  0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 0.0,
        ];

        let indices: [u32; 6] = [
            0,1,3,
            1,2,3
        ];

        // Linking Vertex Attributes
        // let (mut vbo, mut ebo) = (0, 0);
        let mut vbo = 0;
        gl::GenVertexArrays(1, &mut internal_vao);
        gl::BindVertexArray(internal_vao);

        gl::GenBuffers(1, &mut vbo);
        // gl::GenBuffers(1, &mut ebo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<f32>()) as isize,
            vertices.as_ptr().cast(),
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

        internal_vao
    };

    shader_program.use_program();
    shader_program.set_int(c"texture1", 0);

    unsafe { gl::Enable(gl::DEPTH_TEST); }

    let floor: [Vec3; 12] = [
        Vec3::new( 0.0,  -0.25,  -0.5),
        Vec3::new( 1.0,  -0.25,  -0.5),
        Vec3::new( -1.0,  -0.25,  -0.5),

        Vec3::new( 1.0,  -0.25,  0.5),
        Vec3::new( -1.0,  -0.25,  0.5),
        Vec3::new( 0.0,  -0.25,  0.5),

        Vec3::new( 0.0,  -0.25,  1.5),
        Vec3::new( 1.0,  -0.25,  1.5),
        Vec3::new( -1.0,  -0.25,  1.5),

        Vec3::new( 0.0,  -0.25,  2.5),
        Vec3::new( 1.0,  -0.25,  2.5),
        Vec3::new( -1.0,  -0.25,  2.5),
    ];

    let mut delta_time: f32 = 0.0; // Time between current frame and last frame
    let mut last_frame: f32 = 0.0; // Time of last frame
    let mut last_x: f32 = 400.0;
    let mut last_y: f32 = 300.0;

    // Camera setup
    let mut camera: Camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
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

        let view_matrix = camera.get_view_matrix();
        let projection_matrix: Mat4 = Mat4::perspective_rh_gl(camera.zoom.to_radians(), 800.0 / 600.0, 0.1, 100.0);

        // Render floor
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindTexture(gl::TEXTURE_2D, gravel_floor);

            let model_location = gl::GetUniformLocation(shader_program.shader_program_id, c"view".as_ptr());
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &view_matrix.to_cols_array()[0]);

            let model_location = gl::GetUniformLocation(shader_program.shader_program_id, c"projection".as_ptr());
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &projection_matrix.to_cols_array()[0]);

            gl::BindVertexArray(vao);
            for v in floor {
                let model_matrix = Mat4::IDENTITY * Mat4::from_translation(v) * Mat4::from_rotation_x(-90.0f32.to_radians());
                let model_location = gl::GetUniformLocation(shader_program.shader_program_id, c"model".as_ptr());
                gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &model_matrix.to_cols_array()[0]);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
            // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
        }

        // Check call events and swap the buffers
        glfw.poll_events();
        window.swap_buffers();
    }
}