#![allow(dead_code)]
use cgmath::{Angle, Rad, Vector3};
use std::f32::consts::PI;
// use core::position::{ExactPos, MapPos, SlotId, get_slots_count};
use core::position::Position as MapPos;
use types::{VertexCoord, WorldDistance, WorldPos};

pub const MIN_LIFT_HEIGHT: f32 = 0.01;

pub fn vec3_z(z: f32) -> Vector3<f32> {
    Vector3 {
        x: 0.0,
        y: 0.0,
        z: z,
    }
}

/// Still need to do checks to see if this is the correct level
pub fn world_pos_to_map_pos(pos: WorldPos) -> MapPos {
    MapPos::new(pos.v.x, pos.v.y)
}

/// Still need to do checks to get the height from the level.
pub fn map_pos_to_world_pos(p: MapPos) -> WorldPos {
    let v: Vector3<f64> = Vector3::new(p.x, p.y, 0.0);
    WorldPos { v: v }
}

pub fn lift(v: Vector3<f32>) -> Vector3<f32> {
    let mut v = v;
    v.z += MIN_LIFT_HEIGHT;
    v
}

/// Not sure what this does. It gives a vertex on a circle?
pub fn index_to_circle_vertex_rnd(count: i32, i: i32, pos: MapPos) -> VertexCoord {
    let n = 2.0 * (PI as f64) * (i as f64) / (count as f64);
    let n = (n as f64) + ((pos.x + pos.y) * 7.0) % 4.0; // TODO: remove magic numbers
    let v = Vector3 {
        x: n.cos(),
        y: n.sin(),
        z: 0.0,
    };
    VertexCoord { v: v }
}

/// Gives a vertex on a circle in the x-y plane. The point is at an angle of (i / count)
pub fn index_to_circle_vertex(count: i32, i: i32) -> VertexCoord {
    let n = (PI / 2.0 + 2.0 * PI * (i as f32) / (count as f32)) as f64;
    VertexCoord {
        v: Vector3 {
            x: n.cos(),
            y: n.sin(),
            z: 0.0,
        },
    }
}

pub fn dist(a: WorldPos, b: WorldPos) -> WorldDistance {
    let dx = (b.v.x - a.v.x).abs();
    let dy = (b.v.y - a.v.y).abs();
    let dz = (b.v.z - a.v.z).abs();
    WorldDistance {
        n: ((dx.powi(2) + dy.powi(2) + dz.powi(2)) as f32).sqrt(),
    }
}

/// Returns the angle of the line connecting these two points with respect to the x? axis
pub fn get_rot_angle(a: WorldPos, b: WorldPos) -> Rad<f32> {
    let diff = b.v - a.v;
    let angle = diff.x.atan2(diff.y) as f32;
    Rad(-angle).normalize()
}

#[cfg(test)]
mod tests {
    use super::{get_rot_angle, index_to_circle_vertex};
    use cgmath::Vector3;
    use std::f32::consts::PI;
    use types::WorldPos;

    const EPS: f32 = 0.001;

    #[test]
    fn test_get_rot_angle_30_deg() {
        let count = 12;
        for i in 0..count {
            let a = WorldPos {
                v: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            };
            let b = WorldPos {
                v: index_to_circle_vertex(count, i).v,
            };
            let expected_angle = i as f32 * (PI * 2.0) / (count as f32);
            let angle = get_rot_angle(a, b);
            let diff = (expected_angle - angle.0).abs();
            assert!(diff < EPS, "{} != {}", expected_angle, angle.0);
        }
    }
}
