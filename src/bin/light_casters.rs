use std::ffi::CStr;
use std::path::Path;
use std::ptr;
use glam::{Mat4, Vec3, Vec4};
use glfw::{Context};
use opengl_book_examples::camera::camera::Camera;
use opengl_book_examples::common::common::{handle_window_event, process_input};
use opengl_book_examples::flashlight::flashlight::FlashLight;
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
        None => std::ptr::null(),
    });

    // Shader setup
    let cube_object_vert_shader = match Shader::load_shader(ShaderType::Vertex, Path::new("src/shaders/vertex/specular_diffuse.vert")) {
        Ok(id) => id,
        Err(e) => {
            panic!("{}", e.to_string())
        }
    };
    let cube_object_frag_shader = match Shader::load_shader(ShaderType::Fragment, Path::new("src/shaders/fragment/spotlight.frag")) {
        Ok(id) => id,
        Err(e) => {
            panic!("{}", e.to_string())
        }
    };
    let lamp_vert_shader = match Shader::load_shader(ShaderType::Vertex, Path::new("src/shaders/vertex/light_source.vert")) {
        Ok(id) => id,
        Err(e) => {
            panic!("{}", e.to_string())
        }
    };
    let lamp_frag_shader = match Shader::load_shader(ShaderType::Fragment, Path::new("src/shaders/fragment/light_source.frag")) {
        Ok(id) => id,
        Err(e) => {
            panic!("{}", e.to_string())
        }
    };
    let cube_object: ShaderProgram = ShaderProgram::new();
    match cube_object.build(&[cube_object_vert_shader, cube_object_frag_shader]) {
        Err(e) => {
            panic!("{}", e.to_string())
        },
        _ => {}
    };
    // let lamp: ShaderProgram = ShaderProgram::new();
    // match lamp.build(&[lamp_vert_shader, lamp_frag_shader]) {
    //     Err(e) => {
    //         panic!("{}", e.to_string())
    //     },
    //     _ => {}
    // };

    // Texture setup
    let texture1 = match Texture::load_texture(Path::new("src/textures/wooden-container-with-metal-frame.png"), true) {
        Ok(id) => id,
        Err(e) => {
            panic!("{}", e.to_string())
        }
    };
    let texture2 = match Texture::load_texture(Path::new("src/textures/steel-frame.png"), true) {
        Ok(id) => id,
        Err(e) => {
            panic!("{}", e.to_string())
        }
    };

    let (cube_vao, light_vao) = unsafe {
        let mut internal_vao = 0;
        let mut internal_light_vao = 0;

        let cube_vertices: [f32; 288] = [
            // positions       // normals        // texture coords
            -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 0.0,
            0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 0.0,
            0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 1.0,
            0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 1.0,
            -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 0.0,

            -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,   0.0, 0.0,
            0.5, -0.5,  0.5,  0.0,  0.0, 1.0,   1.0, 0.0,
            0.5,  0.5,  0.5,  0.0,  0.0, 1.0,   1.0, 1.0,
            0.5,  0.5,  0.5,  0.0,  0.0, 1.0,   1.0, 1.0,
            -0.5,  0.5,  0.5,  0.0,  0.0, 1.0,   0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,   0.0, 0.0,

            -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0, 0.0,
            -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0, 1.0,
            -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0, 1.0,
            -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0, 1.0,
            -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0, 0.0,
            -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0, 0.0,

            0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0, 0.0,
            0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0, 1.0,
            0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0, 1.0,
            0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0, 1.0,
            0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0, 0.0,
            0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0, 0.0,

            -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0, 1.0,
            0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0, 1.0,
            0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0, 0.0,
            0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0, 0.0,
            -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0, 0.0,
            -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0, 1.0,

            -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0, 1.0,
            0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0, 1.0,
            0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0, 0.0,
            0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0, 0.0,
            -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0, 0.0,
            -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0, 1.0
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
            8 * size_of::<f32>() as i32,
            ptr::null()
        );
        gl::EnableVertexAttribArray(0);

        // Normal attribute
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

        gl::GenVertexArrays(1, &mut internal_light_vao);
        gl::BindVertexArray(internal_light_vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            8 * size_of::<f32>() as i32,
            ptr::null()
        );
        gl::EnableVertexAttribArray(0);


        (internal_vao, internal_light_vao)
    };
    unsafe { gl::Enable(gl::DEPTH_TEST); }

    let mut delta_time: f32 = 0.0; // Time between the current frame and last frame
    let mut last_frame: f32 = 0.0; // Time of last frame
    let mut last_x: f32 = 400.0;
    let mut last_y: f32 = 300.0;
    let mut first_mouse = true;

    // Camera setup
    let mut camera: Camera = Camera::new(
        Vec3::new(0.0, 1.0, 3.0),
        Vec3::new(0.0, 1.0, 0.0),
        true
    );
    let mut flashlight: FlashLight = FlashLight::new(true);

    let mut light_position = Vec3::new(1.2, 1.0, 2.0);
    let mut lamp_color = Vec3::new(1.0, 1.0, 1.0);

    let mut wireframe_mode = false;
    cube_object.use_program();
    cube_object.set_int(c"material.diffuse", 0);
    cube_object.set_int(c"material.specular", 1);

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

    // MAIN LOOP
    while !window.should_close() {
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
                &mut flashlight,
                &mut wireframe_mode
            );
        }
        process_input(&window, &mut camera, delta_time);

        let view_matrix = camera.get_view_matrix();
        let projection_matrix: Mat4 = Mat4::perspective_rh_gl(camera.zoom.to_radians(), 800.0 / 600.0, 0.1, 100.0);

        // Rendering
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            cube_object.use_program();
            // cube_object.set_vec3(c"lightPos", &light_position.to_array());
            cube_object.set_float(c"material.shininess", 32.0);

            // let light_direction = Vec3::new(-0.2, -1.0, -0.3);
            // let light_direction = view_matrix * Vec4::new(-0.2, -1.0, -0.3, 0.0);
            // cube_object.set_vec3(c"light.direction", &light_direction.to_array());
            cube_object.set_float(c"light.cutOff", 12.5f32.to_radians().cos());
            cube_object.set_float(c"light.outerCutOff", 17.5f32.to_radians().cos());
            cube_object.set_vec3(c"light.ambient", &[0.2, 0.2, 0.2]);
            cube_object.set_vec3(c"light.diffuse", &[0.5, 0.5, 0.5]);
            cube_object.set_vec3(c"light.specular", &[1.0, 1.0, 1.0]);
            cube_object.set_float(c"light.constant", 1.0);
            cube_object.set_float(c"light.linear", 0.09);
            cube_object.set_float(c"light.quadratic", 0.032);


            let model_matrix = Mat4::IDENTITY;
            let model_location = gl::GetUniformLocation(cube_object.shader_program_id, c"model".as_ptr());
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &model_matrix.to_cols_array()[0]);

            let model_location = gl::GetUniformLocation(cube_object.shader_program_id, c"view".as_ptr());
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &view_matrix.to_cols_array()[0]);

            let model_location = gl::GetUniformLocation(cube_object.shader_program_id, c"projection".as_ptr());
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &projection_matrix.to_cols_array()[0]);

            gl::BindVertexArray(cube_vao);
            for (i, v) in cube_positions.iter().enumerate() {
                let angle = 20.0 * i as f32;
                let model_matrix = Mat4::IDENTITY * Mat4::from_translation(*v) * Mat4::from_axis_angle(Vec3::new(1.0, 0.3, 0.5).normalize(), angle);

                let model_cstr: &CStr = c"model";
                let model_location = gl::GetUniformLocation(cube_object.shader_program_id, model_cstr.as_ptr());
                gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &model_matrix.to_cols_array()[0]);

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
            gl::BindVertexArray(0);

            // lamp.use_program();
            // lamp.set_vec3(c"lightColorSource", &lamp_color.to_array());
            //
            // let model_matrix = Mat4::IDENTITY * Mat4::from_translation(light_position) * Mat4::from_scale(Vec3::new(0.2, 0.2, 0.2));
            // let model_location = gl::GetUniformLocation(lamp.shader_program_id, c"model".as_ptr());
            // gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &model_matrix.to_cols_array()[0]);
            //
            // let model_location = gl::GetUniformLocation(lamp.shader_program_id, c"view".as_ptr());
            // gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &view_matrix.to_cols_array()[0]);
            //
            // let model_location = gl::GetUniformLocation(lamp.shader_program_id, c"projection".as_ptr());
            // gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &projection_matrix.to_cols_array()[0]);
            //
            // gl::BindVertexArray(light_vao);
            // gl::DrawArrays(gl::TRIANGLES, 0, 36);
            // gl::BindVertexArray(0);
        }

        // Check call events and swap the buffers
        glfw.poll_events();
        window.swap_buffers();
    }
}
