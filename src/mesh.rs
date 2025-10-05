/// Mesh module for all objects and textures that get rendered to the scene
pub mod mesh {
    use std::ffi::CString;
    use std::mem::offset_of;
    use std::ptr;
    use glam::{Vec2, Vec3};
    use crate::shaders::shaders::ShaderProgram;

    enum TextureType {
        DIFFUSE,
        SPECULAR
    }

    pub struct Vertex {
        position: Vec3,
        normal: Vec3,
        tex_coord: Vec2
    }

    pub struct Texture {
        id: u32,
        tex_type: TextureType
    }

    pub struct Mesh {
        pub vertices: Vec<Vertex>,
        pub indices: Vec<u32>,
        pub textures: Vec<Texture>,
        vao: u32,
        vbo: u32,
        ebo: u32
    }

    impl Mesh {
        pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Self {
            let (vao, vbo, ebo) = Self::setup_mesh(&vertices, &indices);
            Mesh {
                vertices,
                indices,
                textures,
                vao,
                vbo,
                ebo
            }
        }

        pub fn draw(&self, shader: &ShaderProgram) {
            let mut diffuse_n = 1;
            let mut specular_n = 1;

            for (i, texture) in self.textures.iter().enumerate() {
                unsafe {
                    gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                }
                let mut number = 0;
                let mut name = String::new();
                match texture.tex_type {
                    TextureType::DIFFUSE => {
                        number = diffuse_n;
                        diffuse_n += 1;
                        name = "texture_diffuse".to_string();
                    },
                    TextureType::SPECULAR => {
                        number = specular_n;
                        specular_n += 1;
                        name = "texture_specular".to_string();
                    }
                }
                let c_str = CString::new(format!("material.{name}{number}")).unwrap();
                shader.set_int(&*c_str, i as u32);
                unsafe {
                    gl::BindTexture(gl::TEXTURE_2D, texture.id);
                }
            }
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0);

                // Draw mesh
                gl::BindVertexArray(0);
                gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
                gl::BindVertexArray(0);
            }
        }

        fn setup_mesh(vertices: &[Vertex], indices: &[u32]) -> (u32, u32, u32) {
            let (mut vao, mut vbo, mut ebo) = (0,0,0);
            unsafe {
                gl::GenVertexArrays(1, &mut vao);
                gl::GenBuffers(1, &mut vbo);
                gl::GenBuffers(1, &mut ebo);

                gl::BindVertexArray(vao);
                gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (vertices.len() * size_of::<Vertex>()) as isize,
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

                // vertex positions
                gl::EnableVertexAttribArray(0);
                gl::VertexAttribPointer(
                    0,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    size_of::<Vertex>() as i32,
                    offset_of!(Vertex, position) as *const _
                );

                // normals
                gl::EnableVertexAttribArray(1);
                gl::VertexAttribPointer(
                    1,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    size_of::<Vertex>() as i32,
                    offset_of!(Vertex, normal) as *const _
                );

                // texture coords
                gl::EnableVertexAttribArray(2);
                gl::VertexAttribPointer(
                    2,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    size_of::<Vertex>() as i32,
                    offset_of!(Vertex, tex_coord) as *const _
                );

                gl::BindVertexArray(0);
            }

            (vao, vbo, ebo)
        }
    }
}