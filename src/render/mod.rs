pub mod buffer;
pub mod color;
pub mod display;
pub mod enums;
pub mod geometry;
pub mod message;
pub mod shader;
pub mod vertex;

use bounded_spsc_queue;
use cgmath::*;
use channel::consume_spsc;
use gl;
use render::buffer::geometry::*;
use render::display::*;
use render::enums::*;
use render::geometry::quad::*;
use render::geometry::*;
use render::message::*;
use render::shader::color::*;
use render::vertex::color::*;
use std::thread::sleep;
use std::time::Duration;
use time::timer::*;

struct RenderState {
    display: Display,
    shape_shader: ColorShader,
    quad_buffer: GeometryBuffer<Quad<ColorVertex>>,
}

pub fn start(
    mut display: Display,
    render_consumer: bounded_spsc_queue::Consumer<RenderFrame>,
    resize_consumer: consume_spsc::Consumer<Vector2<u32>>,
) {
    // Initialize the display. The display is bound in the thread we're going to be making opengl
    // calls in. Behavior is undefined is the display is bound outside of the thread and usually
    // segfaults.
    display.bind();
    display.enable_clear_color();
    display.enable_clear_depth();
    display.enable_cull_face();
    display.set_clear_color(0.0, 0.0, 0.2, 1.0);
    display.set_depth_test(DepthTest::LessEqual);
    display.set_cull_face(CullFace::Back);

    // Create the render state.
    let mut state = RenderState {
        display: display,
        shape_shader: ColorShader::new(),
        quad_buffer: Quad::new_geometry_buffer(2500),
    };

    // Log render timings.
    let mut timer_render = Timer::new("[R] Frame");
    loop {
        // Frame processing.
        match render_consumer.try_pop().as_mut() {
            Some(f) => {
                // Start timing.
                timer_render.start();
                // Clear the screen.
                state.display.clear();
                // Resizing.
                state.resize(resize_consumer.consume());
                // Message handling.
                state.handle_messages(&mut f.messages);
                // Draw shapes.
                state.shape_shader.bind();
                state.quad_buffer.draw();
                // Finish.
                state.display.swap_buffers();
                // Finish timing.
                timer_render.stop();
            },
            None => {},
        }
        // Sleep to avoid pegging a core.
        sleep(Duration::new(0, 100));
    }
}

impl RenderState {
    fn handle_messages(&mut self, messages: &mut Vec<RenderMessage>) {
        let mut shader_modified = false;
        let mut geometry_modified = false;
        for message in messages.drain(..) {
            match message {
                // Quads
                RenderMessage::QuadCreate { pos, size, color } => {
                    let quad = Quad::new_rect(pos, size, color);
                    self.quad_buffer.add(quad);
                    geometry_modified = true;
                },
                RenderMessage::QuadUpdate { id, pos, size, color } => {
                    let quad = Quad::new_rect(pos, size, color);
                    self.quad_buffer.update(id, quad);
                    geometry_modified = true;
                },
                RenderMessage::QuadRemove { id } => {
                    self.quad_buffer.remove(id);
                    geometry_modified = true;
                },
                RenderMessage::TextureCreate { .. } => {
                    // TODO
                },
                RenderMessage::Translate { pos } => {
                    self.shape_shader.set_translation(pos);
                    shader_modified = true;
                },
                RenderMessage::Scale { factor } => {
                    self.shape_shader.set_scale(factor);
                    shader_modified = true;
                },
            }
        }
        if geometry_modified {
            self.quad_buffer.sync();
        }
        if shader_modified {
            self.shape_shader.sync();
        }
    }

    fn resize(&mut self, message: Option<Vector2<u32>>) {
        match message {
            Some(msg) => unsafe {
                self.display.resize(msg.x, msg.y);
                gl::Viewport(0, 0, msg.x as i32, msg.y as i32);
                self.shape_shader.bind();
                self.shape_shader.set_bounds(msg.x as f32, msg.y as f32);
            },
            None => {},
        }
    }
}
