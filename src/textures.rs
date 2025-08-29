pub mod textures {
    use std::path::Path;
    use image::{DynamicImage, ImageError};

    pub struct Texture {
        texture_id: u32,
        is_png: bool
    }

    impl Texture {
        pub fn load_texture<P: AsRef<Path>>(texture_path: P, is_png: bool) -> Result<u32, ImageError> {
            let texture = Self::create_texture(is_png);
            texture.configure_texture();
            let img = image::open(texture_path)?.flipv();

            texture.create_texture_image(img);
            texture.generate_mipmap();
            Ok(texture.texture_id)
        }

        fn generate_mipmap(&self) {
            unsafe {
                gl::GenerateMipmap(gl::TEXTURE_2D);
            }
        }

        fn create_texture_image(&self, img: DynamicImage) {
            let format =
                if self.is_png {
                    gl::RGBA
                } else {
                    gl::RGB
                };
            unsafe {
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGB as i32,
                    img.width() as i32,
                    img.height() as i32,
                    0,
                    format,
                    gl::UNSIGNED_BYTE,
                    img.as_bytes().as_ptr().cast()
                );
            }
        }

        fn configure_texture(&self) {
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            }
        }

        fn create_texture(is_png: bool) -> Self {
            let mut texture_id = 0;
            unsafe {
                gl::GenTextures(1, &mut texture_id);
            }
            Texture { texture_id, is_png }
        }
    }
}