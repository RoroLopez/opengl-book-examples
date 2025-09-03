pub mod camera {
    use glam::{Mat4, Vec3};

    #[derive(Debug)]
    pub enum CameraMovement {
        FORWARD,
        BACKWARD,
        RIGHT,
        LEFT,
        UP,
        DOWN
    }

    pub struct Camera {
        // camera Attributes
        pub position: Vec3,
        front: Vec3,
        up: Vec3,
        right: Vec3,
        world_up: Vec3,
        // euler Angles
        yaw: f32,
        pitch: f32,
        // camera options
        movement_speed: f32,
        mouse_sensitivity: f32,
        pub zoom: f32,
        // for fps setting
        enable_fps: bool,
        fixed_position_y: f32
    }

    impl Camera {
        // default Camera values ... might add a way to change them later
        const YAW: f32          = -90.0;
        const PITCH: f32        = 0.0;
        const SPEED: f32        = 2.5;
        const SENSITIVITY: f32  = 0.1;
        const ZOOM: f32         = 45.0;

        pub fn new(position: Vec3, world_up: Vec3, enable_fps: bool) -> Camera {
            let front = Self::get_front_vector(Camera::YAW, Camera::PITCH);
            let right = Self::get_right_vector(front, world_up);
            let up = Self::get_up_vector(right, front);
            Camera {
                position,
                world_up,
                front,
                up,
                right,
                yaw: Camera::YAW,
                pitch: Camera::PITCH,
                movement_speed: Camera::SPEED,
                mouse_sensitivity: Camera::SENSITIVITY,
                zoom: Camera::ZOOM,
                enable_fps,
                fixed_position_y: position.y
            }
        }

        pub fn process_keyboard_movement(&mut self, direction: CameraMovement, delta_time: f32) {
            let velocity: f32 = self.movement_speed * delta_time;
            match direction {
                CameraMovement::FORWARD => {
                    self.position += velocity * self.front;
                }
                CameraMovement::BACKWARD => {
                    self.position -= velocity * self.front;
                }
                CameraMovement::RIGHT => {
                    self.position += velocity * self.right;
                }
                CameraMovement::LEFT => {
                    self.position -= velocity * self.right;
                }
                CameraMovement::UP => {
                    self.position += velocity * self.up;
                    if self.enable_fps {
                        self.fixed_position_y = self.position.y;
                    }
                }
                CameraMovement::DOWN => {
                    self.position -= velocity * self.up;
                    if self.enable_fps {
                        self.fixed_position_y = self.position.y;
                    }
                }
            }
            if self.enable_fps {
                self.position.y = self.fixed_position_y;
            }
        }

        pub fn process_mouse_movement(&mut self, mut x_offset: f32, mut y_offset: f32, constrain_pitch: bool) {
            x_offset *= Camera::SENSITIVITY;
            y_offset *= Camera::SENSITIVITY;

            self.yaw += x_offset;
            self.pitch += y_offset;

            if constrain_pitch {
                if self.pitch > 89.0 {
                    self.pitch = 89.0;
                }
                if self.pitch < -89.0 {
                    self.pitch = -89.0;
                }
            }

            let front = Self::get_front_vector(self.yaw, self.pitch);
            let right = Self::get_right_vector(self.front, self.world_up);
            let up = Self::get_up_vector(self.right, self.front);

            self.front = front;
            self.right = right;
            self.up = up;
        }

        pub fn process_mouse_scroll(&mut self, y_offset: f32) {
            self.zoom -= y_offset;
            if self.zoom < 1.0 {
                self.zoom = 1.0;
            }
            if self.zoom > 45.0 {
                self.zoom = 45.0;
            }
        }

        pub fn get_view_matrix(&self) -> Mat4 {
            Mat4::look_at_rh(self.position, self.position + self.front, self.up)
        }

        fn get_front_vector(yaw: f32, pitch: f32) -> Vec3 {
            Vec3::new(
                yaw.to_radians().cos() * pitch.to_radians().cos(),
                pitch.to_radians().sin(),
                yaw.to_radians().sin() * pitch.to_radians().cos()
            ).normalize()
        }

        fn get_right_vector(front: Vec3, world_up: Vec3) -> Vec3 {
            front.cross(world_up).normalize()
        }

        fn get_up_vector(right: Vec3, front: Vec3) -> Vec3 {
            right.cross(front).normalize()
        }
    }
}