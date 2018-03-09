use bounded_spsc_queue::Consumer;
use gl;
use input::*;
use render::buffer::geometry::*;
use render::display::*;
use render::geometry::*;
use render::geometry::quad::*;
use render::geometry::triangle::*;
use render::message::*;
use render::shader::shape::*;
use render::vertex::shape::*;
use time::timer::*;

pub struct RenderConsumer {
    display: Display,
    render_consumer: Consumer<RenderFrame>,
    shape_shader: ShapeShader,
    triangle_buffer: GeometryBuffer<Triangle<ShapeVertex>>,
    quad_buffer: GeometryBuffer<Quad<ShapeVertex>>,
    timer_render: Timer,
}

impl RenderConsumer {
    pub fn new(display: Display, render_consumer: Consumer<RenderFrame>) -> RenderConsumer {
        // Get the composed consumer
        let mut consumer = RenderConsumer {
            display: display,
            render_consumer: render_consumer,
            shape_shader: ShapeShader::new(),
            triangle_buffer: Triangle::new_geometry_buffer(50),
            quad_buffer: Quad::new_geometry_buffer(100),
            timer_render: Timer::new("Frame"),
        };
        // Initialize it
        consumer.display.enable_clear_color();
        consumer.display.clear_color(0.0, 0.0, 0.0, 1.0);
        consumer.shape_shader.bind();
        consumer.shape_shader.set_scale(0.5f32);
        // Return
        consumer
    }

    pub fn handle_quads(&mut self, messages: &mut Vec<QuadMessage>) {
        for message in messages.drain(..) {
            match message {
                QuadMessage::Create { pos, size, color } => {
                    let quad = Quad::new_rect(pos, size, color);
                    self.quad_buffer.add(quad);
                },
                QuadMessage::Update { id, pos } => {
                    println!("Render: Unimplemented quad update {}, {}", id, pos.x);
                },
                QuadMessage::Remove { id } => {
                    self.quad_buffer.remove(id);
                },
                QuadMessage::None => {},
            }
        }
    }

    pub fn handle_triangles(&mut self, messages: &mut Vec<TriangleMessage>) {
        for message in messages.drain(..) {
            match message {
                TriangleMessage::Create { pos, height, color } => {
                    let triangle = Triangle::new_iso(pos, height, color);
                    self.triangle_buffer.add(triangle);
                },
                TriangleMessage::Update { id, pos } => {
                    println!("Render: Unimplemented triangle update {}, {}", id, pos.x);
                },
                TriangleMessage::Remove { id } => {
                    self.triangle_buffer.remove(id);
                },
                TriangleMessage::None => {},
            }
        }
    }

    pub fn handle_set_translation(&mut self, message: Option<SetTranslationMessage>) {
        match message {
            Some(msg) => {
                self.shape_shader.set_translation(msg.translation);
            },
            None => {},
        };
    }

    pub fn handle_resize(&mut self, message: Option<ResizeMessage>) {
        match message {
            Some(msg) => unsafe {
                gl::Viewport(0, 0, msg.width as i32, msg.height as i32);
                self.shape_shader.bind();
                self.shape_shader
                    .set_bounds(msg.width as f32, msg.height as f32);
            },
            None => {},
        }
    }

    pub fn tick(&mut self) {
        // Frame processing
        match self.render_consumer.try_pop().as_mut() {
            Some(f) => {
                self.timer_render.start();

                // Message Quads
                self.handle_quads(&mut f.quads);
                self.quad_buffer.sync();
                // Message Triangles
                self.handle_triangles(&mut f.triangles);
                self.triangle_buffer.sync();
                // Message Shader
                self.shape_shader.bind();
                self.handle_set_translation(f.translation);
                // Draw Shapes
                self.shape_shader.bind();
                self.quad_buffer.draw();
                self.triangle_buffer.draw();
                // Finish
                self.display.swap_buffers();
                self.display.clear();

                self.timer_render.stop();
            },
            None => {},
        }
    }
}
