use cgmath::Vector3;
use types::WorldPos;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub level: u8,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Position {
        Position {
            x: x,
            y: y,
            level: 0,
        }
    }

    pub fn to_world_pos(&self) -> WorldPos {
        WorldPos {
            v: Vector3 {
                x: self.x,
                y: self.y,
                z: 0.0,
            },
        }
    }
}
