use std::path::Path;
use std::rc::Rc;

use glow::HasContext;
pub struct Texture {
    gl: Rc<glow::Context>,
    pub id: glow::NativeTexture,
}

impl Texture {
    pub fn from_path(gl: &Rc<glow::Context>, path: &Path) -> Self {
        let gl = gl.clone();
        match image::open(path) {
            Ok(img) => {
                let rgba = image::imageops::flip_vertical(&img.to_rgba8());
                let (w, h) = rgba.dimensions();
                let data = rgba.into_raw();
                Self::from_rgba8(&gl, w as i32, h as i32, &data, true)
            }
            Err(e) => {
                eprintln!("texture load {}: {e}, using fallback", path.display());
                Self::solid_rgba(&gl, 255, 0, 255, 255)
            }
        }
    }

    pub fn solid_rgba(gl: &Rc<glow::Context>, r: u8, g: u8, b: u8, a: u8) -> Self {
        let gl = gl.clone();
        Self::from_rgba8(&gl, 1, 1, &[r, g, b, a], false)
    }

    fn from_rgba8(gl: &Rc<glow::Context>, width: i32, height: i32, rgba: &[u8], mipmaps: bool) -> Self {
        let gl = gl.clone();
        unsafe {
            let id = gl.create_texture().unwrap();
            gl.bind_texture(glow::TEXTURE_2D, Some(id));
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
            let (min_f, mag_f) = if mipmaps {
                (glow::LINEAR as i32, glow::LINEAR as i32)
            } else {
                (glow::NEAREST as i32, glow::NEAREST as i32)
            };
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, min_f);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, mag_f);

            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width,
                height,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(Some(rgba)),
            );
            if mipmaps && width > 1 && height > 1 {
                gl.generate_mipmap(glow::TEXTURE_2D);
            }
            gl.bind_texture(glow::TEXTURE_2D, None);
            Self { gl, id }
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.bind_texture(glow::TEXTURE_2D, Some(self.id));
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_texture(self.id);
        }
    }
}
