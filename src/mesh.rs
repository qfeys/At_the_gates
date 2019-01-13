use context::Context;
use fs;
use gfx;
use gfx::traits::FactoryExt;
use gfx_gl;
use pipeline::Vertex;
use texture::{load_texture, Texture};

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct MeshId {
    pub id: u16,
}

#[derive(Clone, Debug)]
pub struct Mesh {
    slice: gfx::Slice<gfx_gl::Resources>,
    vertex_buffer: gfx::handle::Buffer<gfx_gl::Resources, Vertex>,
    texture: Texture,
}

impl Mesh {
    pub fn new(context: &mut Context, vertices: &[Vertex], indices: &[u16], tex: Texture) -> Mesh {
        let (v, s) = context
            .factory_mut()
            .create_vertex_buffer_with_slice(vertices, indices);
        Mesh {
            slice: s,
            vertex_buffer: v,
            texture: tex,
        }
    }

    pub fn slice(&self) -> &gfx::Slice<gfx_gl::Resources> {
        &self.slice
    }

    pub fn vertex_buffer(&self) -> &gfx::handle::Buffer<gfx_gl::Resources, Vertex> {
        &self.vertex_buffer
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }
}
