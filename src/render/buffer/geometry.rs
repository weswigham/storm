use render::buffer::dynamic::*;
use render::raw::*;
use render::vertex::*;

pub struct GeometryBuffer<T: Vertex + Copy> {
    vertex_buffer: DynamicBuffer<T>,
    vertex_array: VertexArray<T>,
}

impl<T: Vertex + Copy> GeometryBuffer<T> {
    pub fn new() -> GeometryBuffer<T> {
        let vertex_buffer = DynamicBuffer::new(BufferBindingTarget::ArrayBuffer);
        let vertex_array = VertexArray::new();
        GeometryBuffer {
            vertex_buffer: vertex_buffer,
            vertex_array: vertex_array,
        }
    }

    #[inline]
    pub fn set_flattened(&mut self, items: &Vec<Vec<T>>) {
        self.vertex_buffer.set_flattened(items);
    }

    #[inline]
    pub fn set(&mut self, items: &Vec<T>) {
        self.vertex_buffer.set(items);
    }

    #[inline]
    pub fn push(&mut self, element: T) {
        self.vertex_buffer.push(element);
    }

    #[inline]
    pub fn swap_remove(&mut self, index: usize) {
        self.vertex_buffer.swap_remove(index);
    }

    #[inline]
    pub fn update(&mut self, index: usize, element: T) {
        self.vertex_buffer.update(index, element);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.vertex_buffer.clear();
    }

    #[inline]
    pub fn sync(&mut self) {
        self.vertex_buffer.sync();
    }

    #[inline]
    pub fn draw(&mut self) {
        let vertices = self.vertex_buffer.len();
        self.vertex_array.bind();
        draw_arrays_instanced(DrawMode::TriangleStrip, 0, 4, vertices as i32);
    }
}
