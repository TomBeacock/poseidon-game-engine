use sdl2::{surface::Surface, image::LoadSurface};

pub struct Texture {
    id: u32
}

impl Texture {
    /// Creates a new `Texture`
    /// 
    /// # Arguments
    /// 
    /// * `path` - The image filepath
    pub fn new(path: &str) -> Self {
        let mut id = 0;

        // Load image
        let surface = Surface::from_file(path).unwrap();
        // Flip image
        let pitch: usize = surface.pitch().try_into().unwrap();
        let mut temp_row = vec![0u8; pitch];
        let pixels: *mut u8;
        unsafe {
            pixels = (*surface.raw()).pixels.cast();
            for i in 0..(surface.height() / 2) {
                let row1 = pixels.offset((i * pitch as u32) as isize);
                let row2 = pixels.offset(((surface.height() - i - 1) * pitch as u32) as isize);
                std::ptr::copy_nonoverlapping(row1, temp_row.as_mut_ptr(), pitch);
                std::ptr::copy_nonoverlapping(row2, row1, pitch);
                std::ptr::copy_nonoverlapping(temp_row.as_mut_ptr(), row2, pitch);
            }
        }

        // Create texture
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

    /// Creates a new `Texture` with the given data
    /// 
    /// # Arguments
    /// 
    /// * `data` - The image data as contiguous r, g, b, a bytes
    /// * `width` - The width of the image
    /// * `height` - The height of the image
    pub fn with_data(data: &Vec<u8>, width: u32, height: u32) -> Self {
        assert_eq!(data.len() / 4, (width * height) as usize);
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexImage2D(
                gl::TEXTURE_2D, 
                0, 
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr().cast());

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
        Texture { id }
    }

    /// Make this buffer the active `Texture` in a chosen slot
    pub fn bind_to_slot(&self, slot: u32) {
        unsafe {
            gl::BindTextureUnit(slot, self.id);
        }
    }

    /// Unbind the current `Texture` from a slot
    pub fn unbind_from_slot(slot: u32) {
        unsafe {
            gl::BindTextureUnit(0, 0);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}