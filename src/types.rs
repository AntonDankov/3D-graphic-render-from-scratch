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

pub fn get_vec3_identity() -> Vec3 {
    Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
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

pub struct Texture {
    pub data: Vec<u32>,
    pub width: u32,
    pub height: u32,
}

pub struct Camera {
    pub position: Vec3,
    pub rotation: Vec3,
    pub velocity: Vec3,
    pub direction: Vec3,
}

pub struct Plane {
    pub position: Vec3,
    pub normal_dirrection: Vec3,
}

pub struct CullinSettings {
    pub planes: Vec<Plane>,
    pub z_near: f32,
    pub z_far: f32,
}

pub struct Memory {
    pub delta_time: f32,
    pub color_buffer: Vec<u32>,
    // pub entity: Vec<Vec3>,
    pub entity: Entity,
    pub projected_points: Vec<Vec2>,
    pub camera: Camera,
    pub rotation_objects_type: u32,
    pub speed: f32,
    pub stop: bool,
    pub show_normals: bool,
    pub fill_triangles: bool,
    pub use_textures: bool,
    pub draw_vert: bool,
    pub draw_edges: bool,
    pub fov: Vec2,
    pub light: Vec3,
    pub texture: Texture,
    pub z_buffer: Vec<f32>,
    pub culling_settings: CullinSettings,
    // pub texture: Vec<u32>,
    // pub window_width: i32,
    // pub window_height: i32,
}

#[derive(Clone, Copy)]
pub struct TextureUV {
    pub u: f32,
    pub v: f32,
}

impl Default for TextureUV {
    fn default() -> Self {
        Self { u: 0.0, v: 0.0 }
    }
}

impl From<TextureUV> for Vec2 {
    fn from(v: TextureUV) -> Self {
        Vec2 { x: v.u, y: v.v }
    }
}

impl From<Vec2> for TextureUV {
    fn from(v: Vec2) -> Self {
        TextureUV { u: v.x, v: v.y }
    }
}

pub struct Entity {
    pub mesh: Mesh,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub translation: Vec3,
    // pub projected_points: Vec<Vec3>,
}

pub struct Triangle {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub a_uv: TextureUV,
    pub b_uv: TextureUV,
    pub c_uv: TextureUV,
}

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    // pub indexes: Vec<IntVec3>,
    // pub indexes: Vec<IntVec3>,
    // pub texture_uv: Vec<TextureUV>,
    pub triangles: Vec<Triangle>,
}

pub struct Polygon {
    pub verticies: Vec<Vec3>,
}
