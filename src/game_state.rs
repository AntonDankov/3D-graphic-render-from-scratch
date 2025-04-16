use crate::texture::REDBRICK_TEXTURE;
use crate::types::{
    get_vec3_identity, Camera, Entity, Memory, Mesh, Plane, RenderSettings, Texture, TextureUV,
    Triangle, Vec2, Vec3, ViewSettings,
};

pub static BOX_POINT_COUNTER: usize = 9 * 9 * 9;
pub static WIDTH: u32 = 1280;
pub static HEIGHT: u32 = 720;
pub static mut COLOR_BUFFER: [u32; (WIDTH * HEIGHT) as usize] = [0; (WIDTH * HEIGHT) as usize];
pub static mut GAME_MEMORY: Option<Memory> = None;

pub fn init_game_memory() {
    unsafe {
        let render_settings = RenderSettings {
            show_normals: false,
            fill_triangles: true,
            draw_vert: true,
            draw_edges: true,
            use_textures: false,
            use_lighting: false,
            default_render_color: 0xFF184787,
        };
        let texture_u32: Vec<u32> = REDBRICK_TEXTURE
            .chunks_exact(4)
            .map(|chunk| u32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect();
        let fov_y: f32 = 3.14159265358979323846264338327950288 / 3.0;
        let aspect_ratio_x: f32 = WIDTH as f32 / HEIGHT as f32;
        let mut view_settings = ViewSettings {
            z_near: 0.01,
            z_far: 100.0,
            planes: vec![],
            fov: Vec2 {
                x: ((fov_y / 2.0).tan() * aspect_ratio_x).atan() * 2.0,
                y: fov_y,
            },
            width: 1280,
            height: 720,
        };
        view_settings.planes =
            generate_culling_planes(view_settings.fov, view_settings.z_near, view_settings.z_far);
        GAME_MEMORY = Some(Memory {
            delta_time: 0.0,
            color_buffer: vec![0; (WIDTH * HEIGHT) as usize],
            entity: generate_box(),
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
            render_settings: render_settings,

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
            view_settings,
        });
    }
}

pub fn get_game_memory() -> &'static mut Memory {
    unsafe { GAME_MEMORY.as_mut().unwrap() }
}

pub fn generate_culling_planes(fov: Vec2, z_near: f32, z_far: f32) -> Vec<Plane> {
    let mut planes: Vec<Plane> = vec![];
    let cos_half_fov_x = (fov.x / 2.0).cos();
    let sin_half_fov_x = (fov.x / 2.0).sin();
    let cos_half_fov_y = (fov.y / 2.0).cos();
    let sin_half_fov_y = (fov.y / 2.0).sin();

    let left_plane = Plane {
        position: Vec3::default(),
        normal_dirrection: Vec3 {
            x: cos_half_fov_x,
            y: 0.0,
            z: sin_half_fov_x,
        },
    };
    planes.push(left_plane);
    let right_plane = Plane {
        position: Vec3::default(),
        normal_dirrection: Vec3 {
            x: -cos_half_fov_x,
            y: 0.0,
            z: sin_half_fov_x,
        },
    };
    planes.push(right_plane);
    let top_plane = Plane {
        position: Vec3::default(),
        normal_dirrection: Vec3 {
            x: 0.0,
            y: -cos_half_fov_y,
            z: sin_half_fov_y,
        },
    };
    planes.push(top_plane);
    let bottom_plane = Plane {
        position: Vec3::default(),
        normal_dirrection: Vec3 {
            x: 0.0,
            y: cos_half_fov_y,
            z: sin_half_fov_y,
        },
    };
    planes.push(bottom_plane);
    let near_plane = Plane {
        position: Vec3 {
            x: 0.0,
            y: 0.0,
            z: z_near,
        },
        normal_dirrection: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    planes.push(near_plane);
    let far_plane = Plane {
        position: Vec3 {
            x: 0.0,
            y: 0.0,
            z: z_far,
        },
        normal_dirrection: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
    };
    planes.push(far_plane);
    planes
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
