use crate::game_state::{get_game_memory, BOX_POINT_COUNTER, FOV_FACTOR, HEIGHT, WIDTH};
use crate::types::{IntVec2, Vec2, Vec3};
// pub fn ortographic_project_entity() {
//     let memory = get_game_memory();
//     for i in 0..BOX_POINT_COUNTER {
//         let point = memory.entity[i];
//         memory.projected_points[i].x = (point.x * FOV_FACTOR) + (WIDTH / 2) as f32;
//         memory.projected_points[i].y = (point.y * FOV_FACTOR) + (HEIGHT / 2) as f32;
//     }
// }

pub fn perspective_project_point(point: Vec3, camera: Vec3) -> Vec2 {
    let projected = Vec2 {
        // x: (point.x - camera.x) * FOV_FACTOR / (point.z - camera.z) + (WIDTH / 2) as f32,
        // y: (point.y - camera.y) * FOV_FACTOR / (point.z - camera.z) + (HEIGHT / 2) as f32,
        x: (point.x) * FOV_FACTOR / point.z + (WIDTH / 2) as f32,
        y: (point.y) * FOV_FACTOR / point.z + (HEIGHT / 2) as f32,
    };
    projected
}

// pub fn perspective_project_entity() {
//     let memory = get_game_memory();
//     for i in 0..BOX_POINT_COUNTER {
//         let point = memory.entity[i];
//         memory.projected_points[i].x =
//             (point.x / (point.z - memory.camera_position.z)) * FOV_FACTOR + (WIDTH / 2) as f32;
//
//         memory.projected_points[i].y =
//             (point.y / (point.z - memory.camera_position.z)) * FOV_FACTOR + (HEIGHT / 2) as f32;
//     }
// }

pub fn transform_vertex(vert: Vec3, rotation: Vec3) -> Vec3 {
    let mut res = vert;
    res = rotate_vec3_x(res, rotation.x);
    res = rotate_vec3_y(res, rotation.y);
    res = rotate_vec3_z(res, rotation.z);
    res.z += 5.0;
    res
}

// pub fn line_intersection(
//     a_start: IntVec2,
//     a_end: IntVec2,
//     b_start: IntVec2,
//     b_end: IntVec2,
// ) -> IntVec2 {
// }

pub fn rotate_entity() {
    let memory = get_game_memory();
    for i in 0..memory.entity.mesh.vertices.len() {
        if (memory.rotation == 0) {
            memory.entity.rotation.x += memory.speed;
            // memory.entity.mesh.vertices[i] = rotate_vec3_x(memory.entity.mesh.vertices[i], memory.speed);
        } else if (memory.rotation == 1) {
            memory.entity.rotation.y += memory.speed;
            // memory.entity.mesh.vertices[i] = rotate_vec3_y(memory.entity.mesh.vertices[i], memory.speed);
        } else if (memory.rotation == 2) {
            memory.entity.rotation.z += memory.speed;
            // memory.entity.mesh.vertices[i] = rotate_vec3_z(memory.entity.mesh.vertices[i], memory.speed);
        }
    }
}

pub fn get_inv_slope(p0: IntVec2, p1: IntVec2) -> f32 {
    let slope = (p1.x - p0.x) as f32 / (p1.y - p0.y) as f32;
    slope
}

pub fn triangle_midpoint(p0: IntVec2, p1: IntVec2, p2: IntVec2) -> IntVec2 {
    let res = IntVec2 {
        x: (((p1.y - p0.y) * (p2.x - p0.x)) / (p2.y - p0.y)) + p0.x,
        y: p1.y,
    };
    res
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

pub fn triangle_avg(vec1: Vec3, vec2: Vec3, vec3: Vec3) -> Vec3 {
    let mut res = Vec3::default();
    res = vector3_add(res, vec1);
    res = vector3_add(res, vec2);
    res = vector3_add(res, vec3);
    res = vector3_div(
        res,
        Vec3 {
            x: 3.0,
            y: 3.0,
            z: 3.0,
        },
    );
    res
}

pub fn vector3_length(vec: Vec3) -> f32 {
    let length = (vec.x * vec.x + vec.y * vec.y + vec.z * vec.z).sqrt();
    length
}

pub fn vector2_length(vec: Vec2) -> f32 {
    let length = (vec.x * vec.x + vec.y * vec.y).sqrt();
    length
}

pub fn vector3_add(a: Vec3, b: Vec3) -> Vec3 {
    let res = Vec3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    };
    res
}

pub fn vector2_add(a: Vec2, b: Vec2) -> Vec2 {
    let res = Vec2 {
        x: a.x + b.x,
        y: a.y + b.y,
    };
    res
}

pub fn vector3_sub(a: Vec3, b: Vec3) -> Vec3 {
    let res = Vec3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    };
    res
}

pub fn vector2_sub(a: Vec2, b: Vec2) -> Vec2 {
    let res = Vec2 {
        x: a.x - b.x,
        y: a.y - b.y,
    };
    res
}

pub fn vector3_mul(a: Vec3, b: Vec3) -> Vec3 {
    let res = Vec3 {
        x: a.x * b.x,
        y: a.y * b.y,
        z: a.z * b.z,
    };
    res
}

pub fn vector2_mul(a: Vec2, b: Vec2) -> Vec2 {
    let res = Vec2 {
        x: a.x * b.x,
        y: a.y * b.y,
    };
    res
}

pub fn vector3_div(a: Vec3, b: Vec3) -> Vec3 {
    let res = Vec3 {
        x: a.x / b.x,
        y: a.y / b.y,
        z: a.z / b.z,
    };
    res
}

pub fn vector2_div(a: Vec2, b: Vec2) -> Vec2 {
    let res = Vec2 {
        x: a.x / b.x,
        y: a.y / b.y,
    };
    res
}

pub fn vector3_cross(a: Vec3, b: Vec3) -> Vec3 {
    let res = Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    };
    res
}

pub fn vector3_dot(a: Vec3, b: Vec3) -> f32 {
    let dot = (a.x * b.x) + (a.y * b.y) + (a.z * b.z);
    dot
}

pub fn vector2_dot(a: Vec2, b: Vec2) -> f32 {
    let dot = a.x * b.x + a.y * b.y;
    dot
}

pub fn vector3_normalize(a: &mut Vec3) {
    let length = (a.x * a.x + a.y * a.y + a.z * a.z).sqrt();
    a.x = a.x / length;
    a.y = a.y / length;
    a.z = a.z / length;
}
