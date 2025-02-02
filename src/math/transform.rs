use cgmath::*;

pub const IDENTITY_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0, //
    0.0, 1.0, 0.0, 0.0, //
    0.0, 0.0, 1.0, 0.0, //
    0.0, 0.0, 0.0, 1.0, //
);

/// Creates an orthographic matrix from screen bounds with a fixed aspect ratio and with 0,0 in the
/// center.
pub fn ortho_from_bounds(bounds: &Vector2<f32>) -> Matrix4<f32> {
    let w = bounds.x / 2.0;
    let h = bounds.y / 2.0;
    ortho(-w.floor(), w.ceil(), -h.floor(), h.ceil(), -1.0, 1.0)
}

pub struct TransformParameters {
    /// The translation of the layer.
    pub translation: Vector2<f32>,
    /// The zoom level of the layer. This is 1.0 by default, meaning 1 pixel takes up 1x1 pixels on
    /// screen.
    pub scale: f32,
    /// Rotation is measured in turns from [0, 1). Values outside of the range are wrapped into the
    /// range. For example, 1.75 is wrapped into 0.75, -0.4 is wrapped into 0.6.
    pub rotation: f32,
}

pub struct Transform {
    params: TransformParameters,
    logical_size: Vector2<f32>,

    transform: Matrix4<f32>,
    transform_dirty: bool,
    ortho: Matrix4<f32>,
    ortho_dirty: bool,
    ortho_transform: Matrix4<f32>,
    ortho_transform_dirty: bool,
}

impl Transform {
    pub fn new(logical_size: Vector2<f32>) -> Transform {
        Transform {
            params: TransformParameters {
                translation: Vector2::new(0.0, 0.0),
                scale: 1.0,
                rotation: 0.0,
            },
            logical_size,

            transform: IDENTITY_MATRIX,
            transform_dirty: false,
            ortho: IDENTITY_MATRIX,
            ortho_dirty: true,
            ortho_transform: IDENTITY_MATRIX,
            ortho_transform_dirty: true,
        }
    }

    pub fn get(&mut self) -> &TransformParameters {
        &self.params
    }

    pub fn set(&mut self) -> &mut TransformParameters {
        self.transform_dirty = true;
        self.ortho_transform_dirty = true;
        &mut self.params
    }

    /// Logical size of the viewport.
    pub fn set_size(&mut self, logical_size: Vector2<f32>) {
        self.ortho_dirty = true;
        self.ortho_transform_dirty = true;
        self.logical_size = logical_size;
    }

    /// Creates a new transform matix based on the parameters of the LayerTransform. The transform
    /// matrix is built in this order: Scale * Translation * Rotation. A new matrix is only created
    /// if it has changed.
    pub fn generate(&mut self) -> Option<Matrix4<f32>> {
        if self.transform_dirty {
            let mut translation = self.params.translation;
            translation.x = (translation.x * self.params.scale).floor() / self.params.scale;
            translation.y = (translation.y * self.params.scale).floor() / self.params.scale;
            self.transform = Matrix4::from_scale(self.params.scale)
                * Matrix4::from_translation(translation.extend(0.0))
                * Matrix4::from_angle_z(Rad(core::f32::consts::PI * 2.0 * self.params.rotation));
            self.transform_dirty = false;
        }

        if self.ortho_dirty {
            self.ortho = ortho_from_bounds(&self.logical_size);
            self.ortho_dirty = false;
        }

        if self.ortho_transform_dirty {
            self.ortho_transform = self.ortho * self.transform;
            self.ortho_transform_dirty = false;
            Some(self.ortho_transform)
        } else {
            None
        }
    }

    /// Creates a new transform matix based on the parameters of the LayerTransform. The transform
    /// matrix is built in this order: Scale * Translation * Rotation.
    pub fn matrix(&mut self) -> Matrix4<f32> {
        self.generate();
        self.ortho_transform
    }
}
