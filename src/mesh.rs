use context::Context;
use fs;
use gfx;
use gfx::traits::FactoryExt;
use gfx_gl;
use pipeline::Vertex;
use texture::{create_flat_texture, load_texture, Texture};
use types::{Size2, WorldPos};

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

    pub fn plane(context: &mut Context, bottom_left: WorldPos, top_right: WorldPos) -> Mesh {
        let bottom_left = bottom_left.v32();
        let top_right = top_right.v32();
        // Move through vertices counterclockwise
        let v1 = Vertex {
            pos: [bottom_left.x, bottom_left.y, bottom_left.z],
            uv: [0.0, 0.0],
        };
        let v2 = Vertex {
            pos: [top_right.x, bottom_left.y, bottom_left.z],
            uv: [0.0, 0.0],
        };
        let v3 = Vertex {
            pos: [top_right.x, top_right.y, top_right.z],
            uv: [0.0, 0.0],
        };
        let v4 = Vertex {
            pos: [bottom_left.x, top_right.y, top_right.z],
            uv: [0.0, 0.0],
        };
        let vertices = [v1, v2, v3, v4];
        let indices: [u16; 6] = [0, 1, 2, 2, 0, 3];
        let texture = create_flat_texture(context, Size2 { w: 4, h: 4 }, [255, 120, 20, 180]);

        Mesh::new(context, &vertices, &indices, texture)
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
