use crate::types::{IntVec2, Vec2, Vec3};
use fixed::types::extra::U16;
use fixed::FixedI64;
type Fixed = FixedI64<U16>;

#[derive(Copy, Clone)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub struct FixedVec4 {
    pub x: Fixed,
    pub y: Fixed,
    pub z: Fixed,
    pub w: Fixed,
}

pub fn vector4_trunk(from: Vec4) -> Vec4 {
    Vec4 {
        x: from.x.trunc(),
        y: from.y.trunc(),
        z: from.z,
        w: from.w,
    }
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

impl From<Vec4> for FixedVec4 {
    fn from(v: Vec4) -> Self {
        FixedVec4 {
            x: Fixed::from_num(v.x),
            y: Fixed::from_num(v.y),
            z: Fixed::from_num(v.z),
            w: Fixed::from_num(v.w),
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
