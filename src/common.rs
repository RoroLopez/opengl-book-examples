pub mod common {
    use glfw::{Action, Key};
    use crate::camera::camera::{Camera, CameraMovement};

    pub fn handle_window_event_original(window: &mut glfw::Window, event: glfw::WindowEvent, wireframe_mode: &mut bool) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            glfw::WindowEvent::Key(Key::LeftControl, _, Action::Press, _) => {
                if *wireframe_mode {
                    unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL); }
                } else {
                    unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }
                }
                *wireframe_mode = !*wireframe_mode;
            }
            _ => {}
        }
    }

    pub fn handle_window_event(
        window: &mut glfw::Window,
        event: glfw::WindowEvent,
        first_mouse: &mut bool,
        last_x: &mut f32,
        last_y: &mut f32,
        camera: &mut Camera,
        wireframe_mode: &mut bool,
    ) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::CursorPos(x_position, y_position) => {
                let (x_position, y_position) = (x_position as f32, y_position as f32);
                if *first_mouse {
                    *last_x = x_position;
                    *last_y = y_position;
                    *first_mouse = false;
                }

                let x_offset = x_position - *last_x;
                let y_offset = *last_y - y_position;
                *last_x = x_position;
                *last_y = y_position;

                camera.process_mouse_movement(x_offset, y_offset, true);
            }
            glfw::WindowEvent::Scroll(x_offset, y_offset) => {
                camera.process_mouse_scroll(y_offset as f32);
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            glfw::WindowEvent::Key(Key::LeftControl, _, Action::Press, _) => {
                if *wireframe_mode {
                    unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL); }
                } else {
                    unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE); }
                }
                *wireframe_mode = !*wireframe_mode;
            }
            _ => {}
        }
    }

    pub fn process_input(window: &glfw::Window, camera: &mut Camera, delta_time: f32) {
        if window.get_key(Key::W) == Action::Press {
            camera.process_keyboard_movement(CameraMovement::FORWARD, delta_time);
        }
        if window.get_key(Key::S) == Action::Press {
            camera.process_keyboard_movement(CameraMovement::BACKWARD, delta_time);
        }
        if window.get_key(Key::D) == Action::Press {
            camera.process_keyboard_movement(CameraMovement::RIGHT, delta_time);
        }
        if window.get_key(Key::A) == Action::Press {
            camera.process_keyboard_movement(CameraMovement::LEFT, delta_time);
        }
    }
}