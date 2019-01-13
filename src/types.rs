use cgmath::{Vector2, Vector3};

pub use core::types::Size2;

#[derive(Copy, Clone, Debug)]
pub struct WorldPos {
    pub v: Vector3<f64>,
}

impl WorldPos {
    pub fn v32(&self) -> Vector3<f32> {
        Vector3 {
            x: self.v.x as f32,
            y: self.v.y as f32,
            z: self.v.z as f32,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct VertexCoord {
    pub v: Vector3<f64>,
}

#[derive(Copy, Clone, Debug)]
pub struct ScreenPos {
    pub v: Vector2<i32>,
}

#[derive(Copy, Clone, Debug)]
pub struct Time {
    pub n: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Speed {
    pub n: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct WorldDistance {
    pub n: f32,
}
