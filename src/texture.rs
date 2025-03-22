use gl::types::{GLenum, GLint, GLsizei, GLuint};
use image::GenericImageView;

/// A simple OpenGL texture ```id: GLuint``` wrapper.
pub struct Texture {
    id: GLuint,
}

impl Texture {
    /// Loads image and returns a [Texture] object from a file at ```path```.
    /// Also you can specify ```filter``` and ```wrap``` for the OpenGL texture.
    /// Right now mipmaps are generated and enabled by default. The max mipmap level is 4.
    /// 
    /// # Filters and Wraps Example
    /// ```rust
    /// use tinystorm::{texture::Texture, gl};
    /// 
    /// let pixelated_texture = Texture::load_from_file("./assets/super_mario.png", gl::NEAREST, gl::CLAMP_TO_EDGE);
    /// let smooth_texture = Texture::load_from_file("./assets/super_mario.png", gl::LINEAR, gl::REPEAT);
    /// ```
    pub fn load_from_file(path: &str, filter: GLenum, wrap: GLenum) -> Self {
        let image = image::open(path);
        if let Err(error) = image { panic!("Failed to load texture at: {}. Error: {}.", path, error); }

        let image = image.unwrap().flipv();
        let (width, height) = image.dimensions();
        let data = image.to_rgba8();

        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap as GLint);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, (filter + gl::NEAREST_MIPMAP_LINEAR - gl::NEAREST) as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filter as GLint);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 4);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                width as GLsizei,
                height as GLsizei,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const std::ffi::c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Self { id }
    }

    /// Binds the texture to certain slot.
    /// Slot is just a ```gl::ActiveTexture(gl::TEXTURE0 + slot);```
    pub fn bind(&self, slot: GLenum) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
    /// Unbinds all texture from OpenGL's state.
    pub fn unbind() {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0); }
    }
}
impl Drop for Texture {
    /// You don't need to manually unbind and delete textures, it's done automatically!
    fn drop(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::DeleteTextures(1, &self.id);
        }
    }
}