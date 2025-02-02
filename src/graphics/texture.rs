use crate::color::ColorDescriptor;
use crate::ctx;
use crate::graphics::{
    max_texture_size, resource, TextureBindingTarget, TextureLoadTarget, TextureMagFilterValue,
    TextureMinFilterValue, TextureParameterTarget, TextureSection, TextureWrapValue,
};
use crate::image::Image;
use alloc::rc::Rc;

/// Represents a GPU resource for a texture.
pub struct Texture {
    id: resource::Texture,
    width: u32,
    height: u32,
    rc: Rc<()>,
}

impl Clone for Texture {
    fn clone(&self) -> Self {
        Texture {
            id: self.id,
            width: self.width,
            height: self.height,
            rc: self.rc.clone(),
        }
    }
}

impl Texture {
    /// Interpret a slice of bytes as a PNG, decodes it into an RGBA image, then uploads it image to
    /// the GPU, creating a texture.
    pub fn from_png(bytes: &[u8]) -> Texture {
        Self::from_image(&Image::from_png(bytes))
    }

    /// Uploads an image to the GPU, creating a texture.
    pub fn from_image<T: ColorDescriptor>(image: &Image<T>) -> Texture {
        let max_size = max_texture_size() as u32;
        if image.width() > max_size || image.height() > max_size {
            panic!(
                "The max width or height texture may have on this device is {}. \
                 The given image has a (width, height) of ({}, {})",
                max_size,
                image.width(),
                image.height()
            );
        }
        let gl = ctx().graphics().gl();
        let id = gl.create_texture();
        let texture = Texture {
            id,
            width: image.width(),
            height: image.height(),
            rc: Rc::new(()),
        };
        gl.bind_texture(TextureBindingTarget::Texture2D, Some(id));
        gl.tex_image_2d(
            TextureLoadTarget::Texture2D,
            0,
            image.width() as i32,
            image.height() as i32,
            0,
            T::layout().gpu_format(),
            T::layout().cpu_format(),
            T::component_type().pixel_type(),
            image.as_slice(),
        );
        gl.tex_parameter_wrap_s(TextureParameterTarget::Texture2D, TextureWrapValue::ClampToEdge);
        gl.tex_parameter_wrap_t(TextureParameterTarget::Texture2D, TextureWrapValue::ClampToEdge);
        gl.tex_parameter_min_filter(TextureParameterTarget::Texture2D, TextureMinFilterValue::Nearest);
        gl.tex_parameter_mag_filter(TextureParameterTarget::Texture2D, TextureMagFilterValue::Nearest);
        gl.bind_texture(TextureBindingTarget::Texture2D, None);
        texture
    }

    /// The width of the texture.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of the texture.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Coordinates relative to the top left corner of the texture. (0, 0) is the top left of the
    /// texture, and (width, height) is the bottom right of the texture.
    pub fn subsection(&self, left: u32, right: u32, top: u32, bottom: u32) -> TextureSection {
        TextureSection::from_texture(&self, left, right, top, bottom)
    }

    /// Sets a subsection of the texture to the given image. (0, 0) is the top left of the texture,
    /// and (width, height) is the bottom right of the texture.
    /// # Arguments
    ///
    /// * `offset_x` - The top left texel x coordinate to offset the image by.
    /// * `offset_y` - The top left texel y coordinate to offset the image by.
    /// * `image` - The image to overwrite the texture with.
    pub fn set<Z: ColorDescriptor>(&self, offset_x: u32, offset_y: u32, image: &Image<Z>) {
        assert!(image.width() + offset_x <= self.width && image.height() + offset_y <= self.height);
        let gl = ctx().graphics().gl();
        gl.bind_texture(TextureBindingTarget::Texture2D, Some(self.id));
        gl.tex_sub_image_2d(
            TextureLoadTarget::Texture2D,
            0,
            offset_x as i32,
            offset_y as i32,
            image.width() as i32,
            image.height() as i32,
            Z::layout().cpu_format(),
            Z::component_type().pixel_type(),
            image.as_slice(),
        );
        gl.bind_texture(TextureBindingTarget::Texture2D, None);
    }

    pub(crate) fn bind(&self, unit: u32) {
        let gl = ctx().graphics().gl();
        gl.active_texture(unit);
        gl.bind_texture(TextureBindingTarget::Texture2D, Some(self.id));
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        if Rc::<()>::strong_count(&self.rc) == 1 {
            ctx().graphics().gl().delete_texture(self.id);
        }
    }
}
