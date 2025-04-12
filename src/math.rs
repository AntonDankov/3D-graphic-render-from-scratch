use crate::game_state::{get_game_memory, BOX_POINT_COUNTER, FOV_FACTOR, HEIGHT, WIDTH};
use crate::matrix::{
    get_matrix4_rotation_x, get_matrix4_rotation_y, get_matrix4_rotation_z, get_matrix4_scale,
    get_matrix4_translation, get_projection_matrix, matrix4_mul_vec4, Matrix4,
};
use crate::types::{IntVec2, TextureUV, Vec2, Vec3};
use crate::vector::Vec4;
// pub fn ortographic_project_entity() {
//     let memory = get_game_memory();
//     for i in 0..BOX_POINT_COUNTER {
//         let point = memory.entity[i];
//         memory.projected_points[i].x = (point.x * FOV_FACTOR) + (WIDTH / 2) as f32;
//         memory.projected_points[i].y = (point.y * FOV_FACTOR) + (HEIGHT / 2) as f32;
//     }
// }

pub fn perspective_project_point(
    point: Vec3,
    camera: Vec3,
    projection_matrix: Matrix4,
    window_height: u32,
    window_width: u32,
) -> Vec4 {
    let normalized = matrix4_mul_vec4(projection_matrix, point.into());

    let mut projected = Vec4 {
        // x: (point.x - camera.x) * FOV_FACTOR / (point.z - camera.z) + (WIDTH / 2) as f32,
        // y: (point.y - camera.y) * FOV_FACTOR / (point.z - camera.z) + (HEIGHT / 2) as f32,
        // x: (point.x) * FOV_FACTOR / point.z + (WIDTH / 2) as f32,
        // y: (point.y) * FOV_FACTOR / point.z + (HEIGHT / 2) as f32,
        x: normalized.x,
        y: normalized.y,
        z: normalized.z,
        w: normalized.w,
    };
    if normalized.w != 0.0 {
        projected.x /= normalized.w;
        projected.y /= normalized.w;
    }
    projected.x *= (window_width as f32 / 2.0) as f32;
    projected.y *= (window_height as f32 / 2.0) as f32;

    // projected.x *= -1.0;
    projected.y *= -1.0;

    projected.x += (window_width as f32 / 2.0) as f32;
    projected.y += (window_height as f32 / 2.0) as f32;

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
//
//
pub fn barycentric_weights(a: IntVec2, b: IntVec2, c: IntVec2, p: IntVec2) -> Vec3 {
    let ac = intvector2_sub(c, a);
    let ab = intvector2_sub(b, a);
    let ap = intvector2_sub(p, a);
    let pc = intvector2_sub(c, p);
    let pb = intvector2_sub(b, p);

    let area_parallelogram_abc = vector2_cross(ac, ab);
    let alpha = vector2_cross(pc, pb) / area_parallelogram_abc;
    let beta = vector2_cross(ac, ap) / area_parallelogram_abc;
    let gamma = 1.0 - alpha - beta;

    Vec3 {
        x: alpha,
        y: beta,
        z: gamma,
    }
}

pub fn transform_vertex(vert: Vec3, rotation: Vec3, scale: Vec3, translation: Vec3) -> Vec3 {
    let mut vec4: Vec4 = vert.into();
    vec4 = matrix4_mul_vec4(get_matrix4_scale(scale.x, scale.y, scale.z), vec4);
    vec4 = matrix4_mul_vec4(get_matrix4_rotation_x(rotation.x), vec4);
    vec4 = matrix4_mul_vec4(get_matrix4_rotation_y(rotation.y), vec4);
    vec4 = matrix4_mul_vec4(get_matrix4_rotation_z(rotation.z), vec4);

    vec4 = matrix4_mul_vec4(
        get_matrix4_translation(translation.x, translation.y, translation.z),
        vec4,
    );

    let mut res: Vec3 = vec4.into();
    // res.z += 5.0;
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
        if (memory.rotation_objects_type == 0) {
            memory.entity.rotation.x += memory.speed * memory.delta_time;
            // memory.entity.scale.x += memory.speed;
            // memory.entity.translation.x += memory.speed;
        } else if (memory.rotation_objects_type == 1) {
            memory.entity.rotation.y += memory.speed * memory.delta_time;
            // memory.entity.scale.y += memory.speed;
            // memory.entity.translation.y += memory.speed;
        } else if (memory.rotation_objects_type == 2) {
            memory.entity.rotation.z += memory.speed * memory.delta_time;
            // memory.entity.scale.z += memory.speed;
            // memory.entity.translation.z = 5.0;
        }
    }
}

pub fn get_inv_slope(p0: IntVec2, p1: IntVec2) -> f32 {
    let slope = (p1.x - p0.x) as f32 / (p1.y - p0.y) as f32;
    slope
}

pub fn light_apply_intensity(original_color: u32, percentage_factor: f32) -> u32 {
    let alpha = original_color & 0xFF000000;
    let red_mask = 0x00FF0000;
    let blue_mask = 0x0000FF00;
    let green_mask = 0x000000FF;
    let red = ((original_color & red_mask) as f32 * percentage_factor) as u32;
    let green = ((original_color & green_mask) as f32 * percentage_factor) as u32;
    let blue = ((original_color & blue_mask) as f32 * percentage_factor) as u32;

    let new_color = alpha | (red & red_mask) | (green & green_mask) | (blue & blue_mask);
    new_color
}

pub fn triangle_vec2_midpoint(p0: IntVec2, p1: IntVec2, p2: IntVec2) -> IntVec2 {
    let res = IntVec2 {
        x: (((p1.y - p0.y) * (p2.x - p0.x)) / (p2.y - p0.y)) + p0.x,
        y: p1.y,
    };
    res
}

pub fn triangle_vec4_midpoint(p0: Vec4, p1: Vec4, p2: Vec4) -> Vec4 {
    let res = Vec4 {
        x: (((p1.y - p0.y) * (p2.x - p0.x)) / (p2.y - p0.y)) + p0.x,
        y: p1.y,
        z: p1.z,
        w: (((p1.y - p0.y) * (p2.w - p0.w)) / (p2.y - p0.y)) + p0.w,
    };
    res
}

pub fn triangle_midpoint_uv(
    p0: Vec4,
    p1: Vec4,
    p2: Vec4,
    uv0: TextureUV,
    uv1: TextureUV,
    uv2: TextureUV,
) -> TextureUV {
    // Get the midpoint in screen space
    let mid_point = triangle_vec4_midpoint(p0, p1, p2);

    // Calculate barycentric weights for this point
    let weights = barycentric_weights(p0.into(), p1.into(), p2.into(), mid_point.into());

    let alpha = weights.x;
    let beta = weights.y;
    let gamma = weights.z;

    let interpolated_reciprocal_w = 1.0 / p0.w * alpha + 1.0 / p1.w * beta + 1.0 / p2.w * gamma;

    // Interpolate texture coordinates using barycentric weights
    let mut texture = TextureUV {
        u: uv0.u * alpha / p0.w + uv1.u / p1.w * beta + uv2.u / p2.w * weights.z,
        v: uv0.v / p0.w * weights.x + uv1.v / p1.w * weights.y + uv2.v / p2.w * weights.z,
    };
    texture.u /= interpolated_reciprocal_w;
    texture.v /= interpolated_reciprocal_w;
    texture
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
pub fn intvector2_sub(a: IntVec2, b: IntVec2) -> IntVec2 {
    let res = IntVec2 {
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
pub fn vector3_mul_float(a: Vec3, b: f32) -> Vec3 {
    let res = Vec3 {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
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

pub fn vector2_cross(a: IntVec2, b: IntVec2) -> f32 {
    let res = a.x as f32 * b.y as f32 - a.y as f32 * b.x as f32;
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
