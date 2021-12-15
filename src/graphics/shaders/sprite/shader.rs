use crate::default_texture;
use crate::graphics::{
    shaders::sprite::Sprite, AsStd140, Buffer, Shader, ShaderDescriptor, Texture, Uniform,
};
use cgmath::Matrix4;

impl ShaderDescriptor<1> for SpriteShader {
    const VERTEX_SHADER: &'static str = include_str!("vertex.glsl");
    const FRAGMENT_SHADER: &'static str = include_str!("fragment.glsl");
    const TEXTURE_NAMES: [&'static str; 1] = ["tex"];
    const VERTEX_UNIFORM_NAME: &'static str = "vertex";
    type VertexUniformType = SpriteUniform;
    type VertexDescriptor = Sprite;
}

#[derive(AsStd140)]
pub struct SpriteUniform {
    pub ortho: Matrix4<f32>,
}

impl SpriteUniform {
    pub fn new(ortho: Matrix4<f32>) -> SpriteUniform {
        SpriteUniform {
            ortho,
        }
    }
}

pub struct SpriteShader {
    shader: Shader<SpriteShader, 1>,
}

impl SpriteShader {
    pub fn new() -> SpriteShader {
        SpriteShader {
            shader: Shader::new(),
        }
    }

    /// Draws to the screen.
    pub fn draw(&self, uniform: &Uniform<SpriteUniform>, atlas: &Texture, buffer: &Buffer<Sprite>) {
        self.shader.draw_instanced(uniform, [atlas], buffer, 4);
    }
}

pub struct SpriteShaderPass {
    pub uniform: Uniform<SpriteUniform>,
    pub atlas: Texture,
    pub buffer: Buffer<Sprite>,
}

impl SpriteShaderPass {
    pub fn new(ortho: Matrix4<f32>) -> SpriteShaderPass {
        SpriteShaderPass {
            uniform: Uniform::new(SpriteUniform::new(ortho)),
            atlas: default_texture(),
            buffer: Buffer::new(),
        }
    }

    /// Draws the pass to the screen.
    pub fn draw(&mut self, shader: &SpriteShader) {
        shader.draw(&self.uniform, &self.atlas, &self.buffer);
    }
}
