use crate::types::{Camera, Entity, IntVec2, Plane, Texture, TextureUV, Vec2, Vec3};
use crate::vector::{vector4_trunk, FixedVec4, Vec4};
use fixed::types::extra::U16;
use fixed::FixedI64;
use std::cmp;
use std::cmp::max;
use std::cmp::min;
use std::thread;
type Fixed = FixedI64<U16>;

use crate::game_state::{get_color_buffer, get_game_memory, HEIGHT, WIDTH};
use crate::math::{
    barycentric_weights, get_inv_slope, light_apply_intensity, perspective_project_point,
    transform_vertex, triangle_avg, triangle_midpoint_uv, triangle_vec2_midpoint,
    triangle_vec4_midpoint, vector2_add, vector2_mul_float, vector2_sub, vector3_add,
    vector3_cross, vector3_dot, vector3_mul, vector3_mul_float, vector3_normalize, vector3_sub,
};
use crate::matrix::{get_fps_view_matrix, get_projection_matrix, Matrix4};

pub fn triangle_cross(a: &FixedVec4, b: &FixedVec4, c: &FixedVec4) -> Fixed {
    let ab_x = b.x - a.x;
    let ab_y = b.y - a.y;
    let ac_x = c.x - a.x;
    let ac_y = c.y - a.y;

    let res = ab_x * ac_y - ab_y * ac_x;

    res
}

pub fn is_edge_top_left(p0: &FixedVec4, p1: &FixedVec4) -> bool {
    let edge_x = p1.x - p0.x;
    let edge_y = p1.y - p0.y;
    let is_edge_top = edge_y == 0 && edge_x > 0;
    let is_edge_left = edge_y < 0;
    let res = is_edge_top || is_edge_left;
    res
}

pub fn subpixel_render_triangle(
    p0: FixedVec4,
    p1: FixedVec4,
    p2: FixedVec4,
    uv0: &TextureUV,
    uv1: &TextureUV,
    uv2: &TextureUV,
    texture: &Texture,
    color: u32,
    light_dot: f32,
) {
    let x_min: i32 = (p0
        .x
        .to_num::<f32>()
        .min(p1.x.to_num::<f32>())
        .min(p2.x.to_num::<f32>()))
    .round() as i32;
    let x_max: i32 = (p0
        .x
        .to_num::<f32>()
        .max(p1.x.to_num::<f32>())
        .max(p2.x.to_num::<f32>()))
    .round() as i32;
    let y_min: i32 = (p0
        .y
        .to_num::<f32>()
        .min(p1.y.to_num::<f32>())
        .min(p2.y.to_num::<f32>()))
    .round() as i32;
    let y_max: i32 = (p0
        .y
        .to_num::<f32>()
        .max(p1.y.to_num::<f32>())
        .max(p2.y.to_num::<f32>()))
    .round() as i32;

    let triangle_area = triangle_cross(&p0, &p1, &p2);

    let delta_w0_col = p1.y - p2.y;
    let delta_w1_col = p2.y - p0.y;
    let delta_w2_col = p0.y - p1.y;

    let delta_w0_row = p2.x - p1.x;
    let delta_w1_row = p0.x - p2.x;
    let delta_w2_row = p1.x - p0.x;

    let threashhold: Fixed = Fixed::from_num(0.001);
    let mut bias0: Fixed = Fixed::from_num(0.0);
    if is_edge_top_left(&p1, &p2) {
        bias0 -= threashhold;
    }
    let mut bias1: Fixed = Fixed::from_num(0.0);
    if is_edge_top_left(&p2, &p0) {
        bias1 -= threashhold;
    }
    let mut bias2: Fixed = Fixed::from_num(0.0);
    if is_edge_top_left(&p0, &p1) {
        bias2 -= threashhold;
    }

    let p_target = FixedVec4 {
        x: Fixed::from_num(x_min as f32 + 0.5),
        y: Fixed::from_num(y_min as f32 + 0.5),
        z: Fixed::from_num(0.5),
        w: Fixed::from_num(0.5),
    };

    let mut w0_row = triangle_cross(&p1, &p2, &p_target) + bias0;
    let mut w1_row = triangle_cross(&p2, &p0, &p_target) + bias1;
    let mut w2_row = triangle_cross(&p0, &p1, &p_target) + bias2;
    let mut is_inside: bool;

    for y in y_min..y_max {
        let mut w0 = w0_row;
        let mut w1 = w1_row;
        let mut w2 = w2_row;

        for x in x_min..x_max {
            is_inside = w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0;
            if is_inside {
                let alpha: f32 = (w0 / triangle_area).to_num();
                let beta: f32 = (w1 / triangle_area).to_num();
                let gamma: f32 = (w2 / triangle_area).to_num();

                let interpolated_u: f32 = uv0.u / p0.w.to_num::<f32>() * alpha
                    + uv1.u / p1.w.to_num::<f32>() * beta
                    + uv2.u / p2.w.to_num::<f32>() * gamma;
                let interpolated_v: f32 = uv0.v / p0.w.to_num::<f32>() * alpha
                    + uv1.v / p1.w.to_num::<f32>() * beta
                    + uv2.v / p2.w.to_num::<f32>() * gamma;
                let interpolated_reciprocal_w: f32 = 1.0 / p0.w.to_num::<f32>() * alpha
                    + 1.0 / p1.w.to_num::<f32>() * beta
                    + 1.0 / p2.w.to_num::<f32>() * gamma;
                let u: u32 =
                    ((interpolated_u / interpolated_reciprocal_w) * texture.width as f32) as u32;
                let v: u32 =
                    ((interpolated_v / interpolated_reciprocal_w) * texture.height as f32) as u32;
                // dbg!(u, v);
                let index =
                    (v * texture.width + u) as usize % (texture.width * texture.height) as usize;
                let w = 1.0 - interpolated_reciprocal_w;
                let pixel_index = (y as u32 * WIDTH + x as u32) as usize;
                let texture_color = texture.data[index];
                let color_after_light = light_apply_intensity(texture_color, light_dot);
                if pixel_index < get_game_memory().z_buffer.len()
                    && get_game_memory().z_buffer[pixel_index] > w
                {
                    get_game_memory().z_buffer[pixel_index] = w;
                    render_pixel(x, y, color_after_light);
                }
                // render_pixel(x, y, color);
            }
            w0 += delta_w0_col;
            w1 += delta_w1_col;
            w2 += delta_w2_col;
        }
        w0_row += delta_w0_row;
        w1_row += delta_w1_row;
        w2_row += delta_w2_row;
    }
}

pub fn render_pixel(x_pos: i32, y_pos: i32, color: u32) {
    let color_buffer = get_color_buffer();
    let width_i32 = WIDTH as i32;
    let height_i32 = HEIGHT as i32;
    if x_pos < width_i32 && x_pos >= 0 && y_pos >= 0 && y_pos < height_i32 {
        color_buffer[(y_pos * width_i32 + x_pos) as usize] = color;
    }
}
