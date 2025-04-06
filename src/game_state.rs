use crate::obj_importer::import_entity_from_obj;
use crate::types::{Entity, IntVec3, Memory, Mesh, Tris, Vec2, Vec3};

pub static FOV_FACTOR: f32 = 640.0;
pub static BOX_POINT_COUNTER: usize = 9 * 9 * 9;
pub static WIDTH: u32 = 1280;
pub static HEIGHT: u32 = 720;
pub static mut COLOR_BUFFER: [u32; (WIDTH * HEIGHT) as usize] = [0; (WIDTH * HEIGHT) as usize];
pub static mut GAME_MEMORY: Option<Memory> = None;

pub fn init_game_memory() {
    unsafe {
        GAME_MEMORY = Some(Memory {
            color_buffer: vec![0; (WIDTH * HEIGHT) as usize],
            // entity: generate_box(),
            entity: import_entity_from_obj(
                "D:\\Coding\\Projects\\graphics_3d_from_scratch_pikuma\\assets\\cube.obj",
            ),
            projected_points: vec![Vec2::default(); BOX_POINT_COUNTER],
            camera_position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: 0,
            speed: 0.000,
            stop: false,
            show_normals: false,
            fill_triangles: true,
            draw_vert: true,
            draw_edges: true,
        });
    }
}

pub fn get_game_memory() -> &'static mut Memory {
    unsafe { GAME_MEMORY.as_mut().unwrap() }
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
    let indexes = vec![
        //front
        IntVec3 { x: 1, y: 2, z: 3 },
        IntVec3 { x: 1, y: 3, z: 4 },
        //right
        IntVec3 { x: 4, y: 3, z: 5 },
        IntVec3 { x: 4, y: 5, z: 6 },
        //back
        IntVec3 { x: 6, y: 5, z: 7 },
        IntVec3 { x: 6, y: 7, z: 8 },
        //left
        IntVec3 { x: 8, y: 7, z: 2 },
        IntVec3 { x: 8, y: 2, z: 1 },
        //top
        IntVec3 { x: 2, y: 7, z: 5 },
        IntVec3 { x: 2, y: 5, z: 3 },
        //bottom
        IntVec3 { x: 6, y: 8, z: 1 },
        // IntVec3 { x: 6, y: 1, z: 4 },
    ];

    let triangles: Vec<Tris> = vec![];

    let entity = Entity {
        mesh: Mesh {
            vertices: vertices,
            indexes: indexes,
            triangles: triangles,
        },
        rotation: Vec3::default(),
    };
    entity
}

pub fn get_color_buffer() -> &'static mut [u32] {
    unsafe { &mut COLOR_BUFFER }
}
