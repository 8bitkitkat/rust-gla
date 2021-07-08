pub use gl::BufferKind;

use crate::Bindable;
use shrinkwraprs::Shrinkwrap;

#[derive(Debug)]
pub struct Buffer {
    kind: BufferKind,
    handle: gl::Buffer,
}

impl Drop for Buffer {
    fn drop(&mut self) {
        gl::delete_buffer(self.handle)
    }
}

impl Buffer {
    pub fn new(kind: BufferKind) -> Self {
        let handle = gl::gen_buffer();
        Self { kind, handle }
    }

    pub fn data<T>(&self, data: &T, usage: gl::Usage) {
        self.bind();
        gl::buffer_data(self.kind, data, usage)
    }
}

impl Bindable for Buffer {
    fn bind(&self) {
        gl::bind_buffer(self.kind, self.handle)
    }

    fn unbind(&self) {
        gl::bind_buffer(self.kind, gl::Buffer::NONE)
    }
}

#[derive(Debug, Shrinkwrap)]
pub struct BufferArray {
    buffer: Buffer,
}

impl BufferArray {
    pub fn new<T>(data: &T, usage: gl::Usage) -> Self {
        let buffer = Buffer::new(BufferKind::Array);
        buffer.data(data, usage);
        Self { buffer }
    }
}

#[derive(Debug, Shrinkwrap)]
pub struct BufferElementArray {
    #[shrinkwrap(main_field)]
    buffer: Buffer,
    ty: gl::Type,
    count: i32,
}

impl BufferElementArray {
    pub fn new<T>(data: &[T], ty: gl::Type, usage: gl::Usage) -> Self {
        let buffer = Buffer::new(BufferKind::ElementArray);

        let count = data.len();
        buffer.bind();
        let size = std::mem::size_of::<T>() * count;
        unsafe {
            gl::buffer_data_ptr(buffer.kind, size as isize, data.as_ptr() as *const _, usage)
        };

        Self {
            buffer,
            ty,
            count: count as i32,
        }
    }

    pub fn ty(&self) -> gl::Type {
        self.ty
    }

    pub fn count(&self) -> i32 {
        self.count
    }
}

pub struct VertexArray {
    handle: gl::VertexArray,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        gl::delete_vertex_array(self.handle)
    }
}

impl VertexArray {
    pub fn new() -> Self {
        let handle = gl::gen_vertex_array();
        Self { handle }
    }
}

impl Bindable for VertexArray {
    fn bind(&self) {
        gl::bind_vertex_array(self.handle)
    }

    fn unbind(&self) {
        gl::bind_vertex_array(gl::VertexArray::NONE)
    }
}
