#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Vec2 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

pub struct Memory {
    pub color_buffer: Vec<u32>,
    pub entity: Vec<Vec3>,
    pub projected_points: Vec<Vec2>,
    pub camera_position: Vec3,
    pub rotation: u32,
}

pub struct Entity {
    pub mesh: Mesh,
    pub projected_points: Vec<Vec3>,
}

pub struct Mesh {
    pub vertecies: Vec<Vec3>,
    pub indexes: Vec<Vec3>,
}
