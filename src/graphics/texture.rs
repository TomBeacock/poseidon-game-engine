use sdl2::surface::Surface;

pub struct Texture {
    id: u32
}

impl Texture {
    pub fn new(surface: &Surface) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(
                gl::TEXTURE_2D, 
                0, 
                gl::RGBA as i32,
                surface.width() as i32,
                surface.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                (*surface.raw()).pixels);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
        Texture { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}