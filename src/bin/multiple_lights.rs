use std::collections::HashMap;
use std::ffi::{CStr, CString};
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
        glfw.create_window(800, 600, "Multiple lights", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

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
    let cube_object_vert_shader = match Shader::load_shader(ShaderType::Vertex, Path::new("src/shaders/vertex/lights.vert")) {
        Ok(id) => id,
        Err(e) => {
            panic!("{}", e.to_string())
        }
    };
    let cube_object_frag_shader = match Shader::load_shader(ShaderType::Fragment, Path::new("src/shaders/fragment/lights.frag")) {
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
    let cube: ShaderProgram = ShaderProgram::new();
    match cube.build(&[cube_object_vert_shader, cube_object_frag_shader]) {
        Err(e) => {
            panic!("{}", e.to_string())
        },
        _ => {}
    };
    let lamp: ShaderProgram = ShaderProgram::new();
    match lamp.build(&[lamp_vert_shader, lamp_frag_shader]) {
        Err(e) => {
            panic!("{}", e.to_string())
        },
        _ => {}
    };

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
    let mut first_mouse: bool = true;

    // Camera setup
    let mut camera: Camera = Camera::new(
        Vec3::new(0.0, 1.0, 3.0),
        Vec3::new(0.0, 1.0, 0.0),
        true
    );
    let mut flashlight: FlashLight = FlashLight::new(true);
    
    let mut lamp_color: Vec3 = Vec3::new(1.0, 1.0, 1.0);

    let mut wireframe_mode: bool = false;
    cube.use_program();
    cube.set_int(c"material.diffuse", 0);
    cube.set_int(c"material.specular", 1);

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
    let light_positions: [Vec3; 4] = [
        Vec3::new(0.7, 0.2, 1.0),
        Vec3::new(2.3, -1.3, -4.0),
        Vec3::new(-4.0, 2.0, -9.0),
        Vec3::new(0.0, 0.0, -3.0),
    ];

    let mut point_light_properties: HashMap<&str, Vec<Vec<f32>>> = HashMap::new();

    // properties setup
    point_light_properties.insert("ambient", vec![vec![0.1, 0.1, 0.1], vec![0.1, 0.1, 0.1], vec![0.1, 0.1, 0.1], vec![0.1, 0.1, 0.1]]);
    point_light_properties.insert("diffuse", vec![vec![1.0, 0.65, 0.0], vec![0.0, 1.0, 0.0], vec![1.0, 0.0, 0.0], vec![0.0, 0.0, 1.0]]);
    // point_light_properties.insert("diffuse", vec![vec![1.0, 0.65, 0.1]; 4]);
    point_light_properties.insert("specular", vec![vec![1.0, 1.0, 1.0]; 4]);
    point_light_properties.insert("constant", vec![vec![1.0]; 4]);
    point_light_properties.insert("linear", vec![vec![0.09]; 4]);
    point_light_properties.insert("quadratic", vec![vec![0.032]; 4]);

    // MAIN LOOP
    while !window.should_close() {
        // println!("Camera position: {}", camera.position);
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

            cube.use_program();
            // uniform material setup
            cube.set_float(c"material.shininess", 32.0);

            // global light setup
            cube.set_vec3(c"dirLight.direction", &[-0.2, -1.0, -0.3]);
            cube.set_vec3(c"dirLight.ambient", &[0.2, 0.2, 0.2]);
            cube.set_vec3(c"dirLight.diffuse", &[0.5, 0.5, 0.5]);
            cube.set_vec3(c"dirLight.specular", &[1.0, 1.0, 1.0]);

            // spotlight setup (just flashlight)
            let light = flashlight.get_light();

            cube.set_float(c"spotLight.cutOff", 12.5f32.to_radians().cos());
            cube.set_float(c"spotLight.outerCutOff", 17.5f32.to_radians().cos());
            cube.set_vec3(c"spotLight.ambient", &[0.0, 0.0, 0.0]);
            cube.set_vec3(c"spotLight.diffuse", &light);
            cube.set_vec3(c"spotLight.specular", &light);
            cube.set_float(c"spotLight.constant", 1.0);
            cube.set_float(c"spotLight.linear", 0.09);
            cube.set_float(c"spotLight.quadratic", 0.032);

            // point lights properties setup
            let point_light_fields = vec!["ambient", "diffuse", "specular", "constant", "linear", "quadratic"];

            // point lights setup
            for field in &point_light_fields {
                for i in 0..4 {
                    let mut point_light_name = format!("pointLights[{i}].");
                    point_light_name.push_str(field);
                    let c_str = CString::new(point_light_name).unwrap();
                    if *field == "ambient" || *field == "diffuse" || *field == "specular" {
                        let property = point_light_properties.get(field).unwrap();
                        cube.set_vec3(&*c_str, &property[i]);
                    } else {
                        let property = point_light_properties.get(field).unwrap();
                        cube.set_float(&*c_str, property[i][0]);
                    }
                }
            }

            for i in 0..4 {
                let light_position_name = format!("lightPositions[{i}]");
                let light_in_view_coords: Vec4 = camera.get_view_matrix() * Vec4::new(light_positions[i].x, light_positions[i].y, light_positions[i].z, 1.0);
                let c_str = CString::new(light_position_name).unwrap();
                cube.set_vec3(&*c_str, &light_in_view_coords.to_array());
            }

            let model_matrix = Mat4::IDENTITY;
            let model_location = gl::GetUniformLocation(cube.shader_program_id, c"model".as_ptr());
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &model_matrix.to_cols_array()[0]);

            let model_location = gl::GetUniformLocation(cube.shader_program_id, c"view".as_ptr());
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &view_matrix.to_cols_array()[0]);

            let model_location = gl::GetUniformLocation(cube.shader_program_id, c"projection".as_ptr());
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &projection_matrix.to_cols_array()[0]);

            gl::BindVertexArray(cube_vao);
            for (i, v) in cube_positions.iter().enumerate() {
                let angle = 20.0 * i as f32;
                let model_matrix = Mat4::IDENTITY * Mat4::from_translation(*v) * Mat4::from_axis_angle(Vec3::new(1.0, 0.3, 0.5).normalize(), angle);

                let model_cstr: &CStr = c"model";
                let model_location = gl::GetUniformLocation(cube.shader_program_id, model_cstr.as_ptr());
                gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &model_matrix.to_cols_array()[0]);

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
            gl::BindVertexArray(0);

            lamp.use_program();
            let model_location = gl::GetUniformLocation(lamp.shader_program_id, c"view".as_ptr());
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &view_matrix.to_cols_array()[0]);

            let model_location = gl::GetUniformLocation(lamp.shader_program_id, c"projection".as_ptr());
            gl::UniformMatrix4fv(model_location, 1, gl::FALSE, &projection_matrix.to_cols_array()[0]);


            gl::BindVertexArray(light_vao);
            for (i, v) in light_positions.iter().enumerate() {
                let light = point_light_properties.get("diffuse").unwrap();
                lamp.set_vec3(c"lightColorSource", &light[i]);
                let model_matrix: Mat4 = Mat4::IDENTITY * Mat4::from_translation(*v) * Mat4::from_scale(Vec3::new(0.2, 0.2, 0.2));

                let model_cstr: &CStr = c"model";
                let model_location = gl::GetUniformLocation(lamp.shader_program_id, model_cstr.as_ptr());
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
