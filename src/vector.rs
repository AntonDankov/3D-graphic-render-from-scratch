use crate::types::{IntVec2, Vec2, Vec3};

#[derive(Copy, Clone)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<Vec3> for Vec4 {
    fn from(v: Vec3) -> Self {
        Vec4 {
            x: v.x,
            y: v.y,
            z: v.z,
            w: 1.0,
        }
    }
}

impl From<Vec4> for Vec3 {
    fn from(v: Vec4) -> Self {
        Vec3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<Vec4> for Vec2 {
    fn from(v: Vec4) -> Self {
        Vec2 { x: v.x, y: v.y }
    }
}

impl From<Vec4> for IntVec2 {
    fn from(v: Vec4) -> Self {
        IntVec2 {
            x: v.x.trunc() as i32,
            y: v.y.trunc() as i32,
        }
    }
}
