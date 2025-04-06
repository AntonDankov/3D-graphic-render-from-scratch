#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy)]
pub struct IntVec2 {
    pub x: i32,
    pub y: i32,
}

pub struct IntVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
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

impl From<Vec2> for IntVec2 {
    fn from(v: Vec2) -> Self {
        IntVec2 {
            x: v.x.trunc() as i32,
            y: v.y.trunc() as i32,
        }
    }
}

pub struct Memory {
    pub color_buffer: Vec<u32>,
    // pub entity: Vec<Vec3>,
    pub entity: Entity,
    pub projected_points: Vec<Vec2>,
    pub camera_position: Vec3,
    pub rotation: u32,
    pub speed: f32,
    pub stop: bool,
    pub show_normals: bool,
    pub fill_triangles: bool,
    pub draw_vert: bool,
    pub draw_edges: bool,
}

pub struct Entity {
    pub mesh: Mesh,
    pub rotation: Vec3,
    // pub projected_points: Vec<Vec3>,
}

pub struct Tris {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub indexes: Vec<IntVec3>,
    pub triangles: Vec<Tris>,
}
