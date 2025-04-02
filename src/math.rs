use crate::game_state::{get_game_memory, BOX_POINT_COUNTER, FOV_FACTOR, HEIGHT, WIDTH};
use crate::types::Vec3;
pub fn ortographic_project_entity() {
    let memory = get_game_memory();
    for i in 0..BOX_POINT_COUNTER {
        let point = memory.entity[i];
        memory.projected_points[i].x = (point.x * FOV_FACTOR) + (WIDTH / 2) as f32;
        memory.projected_points[i].y = (point.y * FOV_FACTOR) + (HEIGHT / 2) as f32;
    }
}

pub fn perspective_project_entity() {
    let memory = get_game_memory();
    for i in 0..BOX_POINT_COUNTER {
        let point = memory.entity[i];
        memory.projected_points[i].x =
            (point.x / (point.z - memory.camera_position.z)) * FOV_FACTOR + (WIDTH / 2) as f32;

        memory.projected_points[i].y =
            (point.y / (point.z - memory.camera_position.z)) * FOV_FACTOR + (HEIGHT / 2) as f32;
    }
}

pub fn rotate_entity() {
    let memory = get_game_memory();
    for i in 0..memory.entity.len() {
        if (memory.rotation == 0) {
            memory.entity[i] = rotate_vec3_x(memory.entity[i], 0.05);
        } else if (memory.rotation == 1) {
            memory.entity[i] = rotate_vec3_y(memory.entity[i], 0.05);
        } else if (memory.rotation == 2) {
            memory.entity[i] = rotate_vec3_z(memory.entity[i], 0.05);
        }
    }
}

pub fn rotate_vec3_x(vec: Vec3, angle: f32) -> Vec3 {
    let mut res = Vec3::default();
    res.x = vec.x;
    res.y = vec.y * angle.cos() - vec.z * angle.sin();
    res.z = vec.y * angle.sin() + vec.z * angle.cos();
    res
}

pub fn rotate_vec3_y(vec: Vec3, angle: f32) -> Vec3 {
    let mut res = Vec3::default();
    res.x = vec.x * angle.cos() - vec.z * angle.sin();
    res.y = vec.y;
    res.z = vec.x * angle.sin() + vec.z * angle.cos();
    res
}

pub fn rotate_vec3_z(vec: Vec3, angle: f32) -> Vec3 {
    let mut res = Vec3::default();
    res.x = vec.x * angle.cos() - vec.y * angle.sin();
    res.y = vec.x * angle.sin() + vec.y * angle.cos();
    res.z = vec.z;
    res
}
