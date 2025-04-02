use crate::types::{Memory, Vec2, Vec3};

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
            entity: vec![Vec3::default(); BOX_POINT_COUNTER],
            projected_points: vec![Vec2::default(); BOX_POINT_COUNTER],
            camera_position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            rotation: 0,
        });
    }
}

pub fn get_game_memory() -> &'static mut Memory {
    unsafe { GAME_MEMORY.as_mut().unwrap() }
}

pub fn init_box() {
    let memory = get_game_memory();
    let mut counter: usize = 0;

    for i in 0..9 {
        let z = -1.0 + (i as f32 * 0.25);
        for j in 0..9 {
            let y = -1.0 + (j as f32 * 0.25);
            for k in 0..9 {
                let x = -1.0 + (k as f32 * 0.25);

                // Check array bounds
                memory.entity[counter] = Vec3 { x, y, z };
                counter += 1;
            }
        }
    }
}

pub fn get_color_buffer() -> &'static mut [u32] {
    unsafe { &mut COLOR_BUFFER }
}
