use crate::obj_importer::import_entity_from_obj;
use crate::texture::REDBRICK_TEXTURE;
use crate::types::{
    get_vec3_identity, Camera, CullinSettings, Entity, IntVec3, Memory, Mesh, Plane, Texture,
    TextureUV, Triangle, Vec2, Vec3,
};
use std::f32::consts::PI;

pub static FOV_FACTOR: f32 = 640.0;
pub static BOX_POINT_COUNTER: usize = 9 * 9 * 9;
pub static WIDTH: u32 = 1280;
pub static HEIGHT: u32 = 720;
pub static mut COLOR_BUFFER: [u32; (WIDTH * HEIGHT) as usize] = [0; (WIDTH * HEIGHT) as usize];
pub static mut GAME_MEMORY: Option<Memory> = None;

pub fn init_game_memory() {
    unsafe {
        let texture_u32: Vec<u32> = REDBRICK_TEXTURE
            .chunks_exact(4)
            .map(|chunk| u32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect();
        GAME_MEMORY = Some(Memory {
            delta_time: 0.0,
            color_buffer: vec![0; (WIDTH * HEIGHT) as usize],
            entity: generate_box(),
            // entity: import_entity_from_obj(
            //     "D:\\Coding\\Projects\\graphics_3d_from_scratch_pikuma\\assets\\cube.obj",
            // ),
            projected_points: vec![Vec2::default(); BOX_POINT_COUNTER],
            camera: Camera {
                position: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -5.0,
                },
                rotation: Vec3::default(),
                velocity: Vec3::default(),
                direction: Vec3::default(),
            },
            rotation_objects_type: 0,
            speed: 0.000,
            stop: false,
            show_normals: false,
            fill_triangles: true,
            draw_vert: true,
            draw_edges: true,
            use_textures: true,
            fov: 3.14159265358979323846264338327950288 / 3.0,
            light: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            texture: Texture {
                data: texture_u32,
                width: 64,
                height: 64,
            },
            z_buffer: vec![1.0; (WIDTH * HEIGHT) as usize],
            culling_settings: CullinSettings {
                planes: vec![],
                z_near: 0.01,
                z_far: 100.0,
            },
        });
    }
}

pub fn get_game_memory() -> &'static mut Memory {
    unsafe { GAME_MEMORY.as_mut().unwrap() }
}

pub fn generate_culling_planes(z_near: f32, z_far: f32) {
    let planes: Vec<Plane> = vec![];
}

pub fn generate_box() -> Entity {
    let vertices = vec![
        Vec3 {
            x: -1.0,
            y: -1.0,
            z: -1.0,
        },
        Vec3 {
            x: -1.0,
            y: 1.0,
            z: -1.0,
        },
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: -1.0,
        },
        Vec3 {
            x: 1.0,
            y: -1.0,
            z: -1.0,
        },
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        Vec3 {
            x: 1.0,
            y: -1.0,
            z: 1.0,
        },
        Vec3 {
            x: -1.0,
            y: 1.0,
            z: 1.0,
        },
        Vec3 {
            x: -1.0,
            y: -1.0,
            z: 1.0,
        },
    ];

    let triangles = vec![
        //front
        Triangle {
            a: 1,
            b: 2,
            c: 3,
            a_uv: TextureUV { u: 0.0, v: 0.0 },
            b_uv: TextureUV { u: 0.0, v: 1.0 },
            c_uv: TextureUV { u: 1.0, v: 1.0 },
        },
        Triangle {
            a: 1,
            b: 3,
            c: 4,
            a_uv: TextureUV { u: 0.0, v: 0.0 },
            b_uv: TextureUV { u: 1.0, v: 1.0 },
            c_uv: TextureUV { u: 1.0, v: 0.0 },
        },
        //right
        Triangle {
            a: 4,
            b: 3,
            c: 5,
            a_uv: TextureUV { u: 0.0, v: 0.0 },
            b_uv: TextureUV { u: 0.0, v: 1.0 },
            c_uv: TextureUV { u: 1.0, v: 1.0 },
        },
        Triangle {
            a: 4,
            b: 5,
            c: 6,
            a_uv: TextureUV { u: 0.0, v: 0.0 },
            b_uv: TextureUV { u: 1.0, v: 1.0 },
            c_uv: TextureUV { u: 1.0, v: 0.0 },
        },
        //back
        Triangle {
            a: 6,
            b: 5,
            c: 7,
            a_uv: TextureUV { u: 0.0, v: 0.0 },
            b_uv: TextureUV { u: 0.0, v: 1.0 },
            c_uv: TextureUV { u: 1.0, v: 1.0 },
        },
        Triangle {
            a: 6,
            b: 7,
            c: 8,
            a_uv: TextureUV { u: 0.0, v: 0.0 },
            b_uv: TextureUV { u: 1.0, v: 1.0 },
            c_uv: TextureUV { u: 1.0, v: 0.0 },
        },
        //left
        Triangle {
            a: 8,
            b: 7,
            c: 2,
            a_uv: TextureUV { u: 0.0, v: 0.0 },
            b_uv: TextureUV { u: 0.0, v: 1.0 },
            c_uv: TextureUV { u: 1.0, v: 1.0 },
        },
        Triangle {
            a: 8,
            b: 2,
            c: 1,
            a_uv: TextureUV { u: 0.0, v: 0.0 },
            b_uv: TextureUV { u: 1.0, v: 1.0 },
            c_uv: TextureUV { u: 1.0, v: 0.0 },
        },
        //top
        Triangle {
            a: 2,
            b: 7,
            c: 5,
            a_uv: TextureUV { u: 0.0, v: 0.0 },
            b_uv: TextureUV { u: 0.0, v: 1.0 },
            c_uv: TextureUV { u: 1.0, v: 1.0 },
        },
        Triangle {
            a: 2,
            b: 5,
            c: 3,
            a_uv: TextureUV { u: 0.0, v: 0.0 },
            b_uv: TextureUV { u: 1.0, v: 1.0 },
            c_uv: TextureUV { u: 1.0, v: 0.0 },
        },
        //bottom
        Triangle {
            a: 6,
            b: 8,
            c: 1,
            a_uv: TextureUV { u: 0.0, v: 0.0 },
            b_uv: TextureUV { u: 0.0, v: 1.0 },
            c_uv: TextureUV { u: 1.0, v: 1.0 },
        },
        Triangle {
            a: 6,
            b: 1,
            c: 4,
            a_uv: TextureUV { u: 0.0, v: 0.0 },
            b_uv: TextureUV { u: 1.0, v: 1.0 },
            c_uv: TextureUV { u: 1.0, v: 0.0 },
        },
    ];

    let mut translation = Vec3::default();
    translation.z = 5.0;
    let entity = Entity {
        mesh: Mesh {
            vertices: vertices,
            triangles: triangles,
        },
        rotation: Vec3::default(),
        scale: get_vec3_identity(),
        translation: translation,
    };
    entity
}

pub fn get_color_buffer() -> &'static mut [u32] {
    unsafe { &mut COLOR_BUFFER }
}
