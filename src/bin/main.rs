use glfw::{Context};
use opengl_book_examples::utils::{handle_window_event};

/// This file is the basis for all the exercises of OpenGL as well as the loop for processing
/// any user input. The main loop consists always of taking Input, Rendering and Check call events
/// with swap buffers for smoother FPS transitions
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
        None => std::ptr::null(),
    });

    let mut wireframe_mode = false;
    while !window.should_close() {
        // Input
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event, &mut wireframe_mode);
        }

        // Rendering
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

        }

        // Check call events and swap the buffers
        glfw.poll_events();
        window.swap_buffers();
    }
}
