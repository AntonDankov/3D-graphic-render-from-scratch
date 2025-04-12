use crate::texture::REDBRICK_TEXTURE;
use crate::types::{Camera, Entity, IntVec2, Texture, TextureUV, Vec2, Vec3};
use crate::vector::Vec4;
use sdl2::pixels::Color;
use sdl2::render::Texture as SdlTexture;
use sdl2::video::Window;
use std::cmp;
use std::num;
use std::thread;

use crate::game_state::{get_color_buffer, get_game_memory, BOX_POINT_COUNTER, HEIGHT, WIDTH};
use crate::math::{
    barycentric_weights, get_inv_slope, light_apply_intensity, perspective_project_point,
    transform_vertex, triangle_avg, triangle_midpoint_uv, triangle_vec2_midpoint,
    triangle_vec4_midpoint, vector3_add, vector3_cross, vector3_dot, vector3_mul,
    vector3_normalize, vector3_sub,
};
use crate::matrix::{
    get_fps_view_matrix, get_look_at_view_matrix, get_projection_matrix, matrix4_mul_matrix4,
    Matrix4,
};

pub fn render_entity(entity: &Entity, camera: &mut Camera) {
    let line_color = 0xFF00FF00;
    let game_memory = get_game_memory();
    // dbg!(
    //     game_memory.entity.rotation.x,
    //     game_memory.entity.rotation.y,
    //     game_memory.entity.rotation.z
    // );

    let mut projection_matrix =
        get_projection_matrix(game_memory.fov, HEIGHT as f32 / WIDTH as f32);
    /*  Look-At method of view with locking on the target
    let view_matrix = get_look_at_view_matrix(
        camera.position,
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 5.0,
        },
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
    ); */
    let view_matrix = get_fps_view_matrix(&mut game_memory.camera);
    projection_matrix = matrix4_mul_matrix4(projection_matrix, view_matrix);
    for (i, triangle) in entity.mesh.triangles.iter().enumerate() {
        // dbg!(i);
        let x_index = (triangle.a as usize) - 1;
        let y_index = (triangle.b as usize) - 1;
        let z_index = (triangle.c as usize) - 1;
        let p0 = transform_vertex(
            entity.mesh.vertices[x_index],
            entity.rotation,
            entity.scale,
            entity.translation,
        );
        let p1 = transform_vertex(
            entity.mesh.vertices[y_index],
            entity.rotation,
            entity.scale,
            entity.translation,
        );
        let p2 = transform_vertex(
            entity.mesh.vertices[z_index],
            entity.rotation,
            entity.scale,
            entity.translation,
        );

        let vector_ab = vector3_sub(p1, p0);
        let vector_ac = vector3_sub(p2, p0);
        let mut normal = vector3_cross(vector_ab, vector_ac);
        vector3_normalize(&mut normal);

        let camera_ray = vector3_sub(camera.position, p0);
        let dot = vector3_dot(normal, camera_ray);

        if (dot < 0.0) {
            continue;
        }
        let mut normal_avg = triangle_avg(p0, p1, p2);

        let projected0 =
            perspective_project_point(p0, camera.position, projection_matrix, HEIGHT, WIDTH);
        let projected1 =
            perspective_project_point(p1, camera.position, projection_matrix, HEIGHT, WIDTH);
        let projected2 =
            perspective_project_point(p2, camera.position, projection_matrix, HEIGHT, WIDTH);
        // let p = IntVec2 {
        //     x: ((projected0.x + projected1.x) / 2.0) as i32,
        //     y: ((projected2.y + projected1.y) / 2.0) as i32,
        // };
        // dbg!(p.x, p.y);
        // dbg!(p0.x, p0.y, p0.z, projected0.x, projected0.y);
        // dbg!(p1.x, p1.y, p1.z, projected1.x, projected1.y);
        // dbg!(p2.x, p2.y, p2.z, projected2.x, projected2.y);
        // let bar_res =
        //     barycentric_weights(projected0.into(), projected1.into(), projected2.into(), p);
        // dbg!(bar_res.x, bar_res.y, bar_res.z);

        if get_game_memory().fill_triangles {
            let light_dot = vector3_dot(game_memory.light, normal) * -1.0;
            let color_by_light = light_apply_intensity(0xFF184787, light_dot);
            if (game_memory.use_textures) {
                let texture_uv0 = triangle.a_uv.clone();
                let texture_uv1 = triangle.b_uv.clone();
                let texture_uv2 = triangle.c_uv.clone();

                fill_triangle_with_texture(
                    projected0,
                    projected1,
                    projected2,
                    texture_uv0,
                    texture_uv1,
                    texture_uv2,
                    &game_memory.texture,
                );
            } else {
                fill_triangle(
                    projected0.into(),
                    projected1.into(),
                    projected2.into(),
                    color_by_light,
                );
            }
        }

        if game_memory.draw_edges {
            render_edges(
                projected0.into(),
                projected1.into(),
                projected2.into(),
                line_color,
            );
        }

        if game_memory.draw_vert {
            render_verticies(projected0.into(), projected1.into(), projected2.into());
        }

        if game_memory.show_normals {
            render_normals(normal, normal_avg, camera.position, projection_matrix);
        }
    }
}

pub fn render_verticies(p0: Vec2, p1: Vec2, p2: Vec2) {
    render_box(p0.x as i32, p0.y as i32, 4, 4, 0xFFFF0000);
    render_box(p1.x as i32, p1.y as i32, 4, 4, 0xFFFF0000);
    render_box(p2.x as i32, p2.y as i32, 4, 4, 0xFFFF0000);
}

pub fn render_edges(p0: Vec2, p1: Vec2, p2: Vec2, line_color: u32) {
    render_line(
        p0.x as i32,
        p0.y as i32,
        p1.x as i32,
        p1.y as i32,
        line_color,
    );
    render_line(
        p1.x as i32,
        p1.y as i32,
        p2.x as i32,
        p2.y as i32,
        line_color,
    );
    render_line(
        p0.x as i32,
        p0.y as i32,
        p2.x as i32,
        p2.y as i32,
        line_color,
    );
}

pub fn render_normals(normal: Vec3, normal_avg: Vec3, camera: Vec3, projection_matrix: Matrix4) {
    let normal_end = vector3_add(
        normal_avg,
        vector3_mul(
            normal,
            Vec3 {
                x: 0.1,
                y: 0.1,
                z: 0.1,
            },
        ),
    );
    let projected_normal_start =
        perspective_project_point(normal_avg, camera, projection_matrix, HEIGHT, WIDTH);
    let projected_normal_end =
        perspective_project_point(normal_end, camera, projection_matrix, HEIGHT, WIDTH);

    render_line(
        projected_normal_start.x as i32,
        projected_normal_start.y as i32,
        projected_normal_end.x as i32,
        projected_normal_end.y as i32,
        0xFFFFFF00,
    );
}

pub fn render_line(x_start: i32, y_start: i32, x_end: i32, y_end: i32, color: u32) {
    let width_i32 = WIDTH as i32;
    let height_i32 = HEIGHT as i32;

    let mut x_start_cheched = x_start;

    let mut x_end_cheched = x_end;
    let mut y_start_cheched = y_start;

    let mut y_end_cheched = y_end;

    let dx = x_end_cheched - x_start_cheched;
    let dy = y_end_cheched - y_start_cheched;

    let side_length = cmp::max(dx.abs(), dy.abs());

    let x_inc: f32 = dx as f32 / (side_length) as f32;
    let y_inc: f32 = dy as f32 / side_length as f32;

    let mut x_cur = x_start_cheched as f32;
    let mut y_cur = y_start as f32;

    for i in 0..side_length + 1 {
        render_pixel(x_cur.trunc() as i32, y_cur.trunc() as i32, color);
        x_cur += x_inc;
        y_cur += y_inc;
    }
}

pub fn render_box(x_pos: i32, y_pos: i32, box_width: u32, box_height: u32, color: u32) {
    let width_i32 = WIDTH as i32;
    let height_i32 = HEIGHT as i32;
    let box_width_i32 = box_width as i32;
    let box_height_i32 = box_height as i32;
    let x_begin = cmp::max(0, x_pos);
    let y_begin = cmp::max(0, y_pos);

    let width_i32 = WIDTH as i32;
    let height_i32 = HEIGHT as i32;

    let x_end = cmp::min(width_i32, x_pos.saturating_add(box_width_i32));
    let y_end = cmp::min(height_i32, y_pos.saturating_add(box_height_i32));
    for y in y_begin..y_end {
        for x in x_begin..x_end {
            if x > width_i32 {
                break;
            }
            render_pixel(x, y, color);
        }
    }
}

pub fn fill_triangle(p0: IntVec2, p1: IntVec2, p2: IntVec2, color: u32) {
    let mut top_point = p0;
    let mut mid_point = p1;
    let mut bottom_point = p2;

    if (bottom_point.y < mid_point.y) {
        std::mem::swap(&mut bottom_point, &mut mid_point);
    }
    if mid_point.y < top_point.y {
        std::mem::swap(&mut mid_point, &mut top_point);
    }
    if bottom_point.y < mid_point.y {
        std::mem::swap(&mut bottom_point, &mut mid_point);
    }

    if mid_point.y == bottom_point.y {
        fill_flat_bottom_triangle(top_point, mid_point, bottom_point, color);
    } else if top_point.y == mid_point.y {
        fill_flat_top_triangle(bottom_point, mid_point, top_point, color);
    } else {
        let mid_intersect_point = triangle_vec2_midpoint(top_point, mid_point, bottom_point);
        fill_flat_bottom_triangle(top_point, mid_point, mid_intersect_point, color);
        fill_flat_top_triangle(bottom_point, mid_point, mid_intersect_point, color);
    }
}

pub fn fill_flat_bottom_triangle(p0: IntVec2, p1: IntVec2, p2: IntVec2, color: u32) {
    let mut left_point = p1;
    let mut right_point = p2;
    if (left_point.x > right_point.x) {
        std::mem::swap(&mut left_point, &mut right_point);
    }

    let max_width = right_point.x - left_point.x;

    let left_slope = get_inv_slope(p0, left_point);
    let right_slope = get_inv_slope(p0, right_point);

    let mut x_start = p0.x as f32;
    let mut x_end = p0.x as f32;

    let y_start = p0.y;
    let y_end = p2.y + 1;
    // dbg!(p0.x, p0.y, p1.x, p1.y, p2.x, p2.y);
    // dbg!(y_start, y_end, left_slope, right_slope, x_start, x_end);
    for y in y_start..y_end {
        render_line(x_start as i32, y, x_end as i32, y, color);
        x_start += left_slope;
        x_end += right_slope;
        if (x_end - x_start) as i32 > max_width {
            x_start = left_point.x as f32;
            x_end = right_point.x as f32;
        }
    }
}

pub fn fill_flat_top_triangle(p0: IntVec2, p1: IntVec2, p2: IntVec2, color: u32) {
    let mut left_point = p1;
    let mut right_point = p2;
    if (left_point.x > right_point.x) {
        std::mem::swap(&mut left_point, &mut right_point);
    }

    let max_width = right_point.x - left_point.x;

    let left_slope = get_inv_slope(left_point, p0);
    let right_slope = get_inv_slope(right_point, p0);

    let mut x_start = p0.x as f32;
    let mut x_end = p0.x as f32;

    let y_start = p0.y;
    let y_end = right_point.y as i32;
    let mut y = y_start;
    while y >= y_end {
        render_line(x_start as i32, y, x_end as i32, y, color);
        x_start -= left_slope;
        x_end -= right_slope;
        if (x_end - x_start) as i32 > max_width {
            x_start = left_point.x as f32;
            x_end = right_point.x as f32;
        }
        y -= 1;
    }
}

pub fn fill_triangle_with_texture(
    p0: Vec4,
    p1: Vec4,
    p2: Vec4,
    texture_uv0: TextureUV,
    texture_uv1: TextureUV,
    texture_uv2: TextureUV,
    texture: &Texture,
) {
    let mut top_point = p0;
    let mut top_point_texture = texture_uv0;

    let mut mid_point = p1;
    let mut mid_point_texture = texture_uv1;

    let mut bottom_point = p2;
    let mut bottom_point_texture = texture_uv2;

    if (top_point.y > mid_point.y) {
        std::mem::swap(&mut top_point, &mut mid_point);
        std::mem::swap(&mut top_point_texture, &mut mid_point_texture);
    }
    if mid_point.y > bottom_point.y {
        std::mem::swap(&mut mid_point, &mut bottom_point);
        std::mem::swap(&mut mid_point_texture, &mut bottom_point_texture);
    }
    if top_point.y > mid_point.y {
        std::mem::swap(&mut top_point, &mut mid_point);
        std::mem::swap(&mut top_point_texture, &mut mid_point_texture);
    }

    // let mut inv_slope_1 = 0.0;
    // let mut inv_slope_2 = 0.0;
    // if mid_point.y - top_point.y != 0 {
    //     inv_slope_1 =
    //         ((mid_point.x - top_point.x) as f32 / (mid_point.y - top_point.y).abs() as f32);
    // }
    // if bottom_point.y - top_point.y != 0 {
    //     inv_slope_2 =
    //         (bottom_point.x - top_point.x) as f32 / (bottom_point.y - top_point.y).abs() as f32;
    // }
    // if mid_point.y - top_point.y != 0 {
    //     for y in top_point.y..mid_point.y + 1 {
    //         let mut x_start = mid_point.x + ((y - mid_point.y) as f32 * inv_slope_1) as i32;
    //         let mut x_end = top_point.x + ((y - top_point.y) as f32 * inv_slope_2) as i32;
    //         if (x_start > x_end) {
    //             std::mem::swap(&mut x_start, &mut x_end);
    //         }
    //         for x in x_start..x_end + 1 {
    //             render_texel(
    //                 IntVec2 { x: x, y: y },
    //                 top_point,
    //                 mid_point,
    //                 bottom_point,
    //                 top_point_texture,
    //                 mid_point_texture,
    //                 bottom_point_texture,
    //                 texture,
    //             );
    //         }
    //     }
    // }
    //
    // inv_slope_1 = 0.0;
    // inv_slope_2 = 0.0;
    // if bottom_point.y - mid_point.y != 0 {
    //     inv_slope_1 =
    //         ((bottom_point.x - mid_point.x) as f32 / (bottom_point.y - mid_point.y).abs() as f32);
    // }
    // if bottom_point.y - top_point.y != 0 {
    //     inv_slope_2 =
    //         (bottom_point.x - top_point.x) as f32 / (bottom_point.y - top_point.y).abs() as f32;
    // }
    // if bottom_point.y - mid_point.y != 0 {
    //     for y in mid_point.y + 1..bottom_point.y + 1 {
    //         let mut x_start = mid_point.x + ((y - mid_point.y) as f32 * inv_slope_1) as i32;
    //         let mut x_end = top_point.x + ((y - top_point.y) as f32 * inv_slope_2) as i32;
    //         if (x_start > x_end) {
    //             std::mem::swap(&mut x_start, &mut x_end);
    //         }
    //         for x in x_start..x_end + 1 {
    //             render_texel(
    //                 IntVec2 { x: x, y: y },
    //                 top_point,
    //                 mid_point,
    //                 bottom_point,
    //                 top_point_texture,
    //                 mid_point_texture,
    //                 bottom_point_texture,
    //                 texture,
    //             );
    //         }
    //     }
    // }

    if mid_point.y == bottom_point.y {
        fill_flat_bottom_triangle_with_texture(
            top_point,
            mid_point,
            bottom_point,
            top_point_texture,
            mid_point_texture,
            bottom_point_texture,
            texture,
        );
    } else if top_point.y == mid_point.y {
        fill_flat_top_triangle_with_texture(
            bottom_point,
            mid_point,
            top_point,
            bottom_point_texture,
            mid_point_texture,
            top_point_texture,
            texture,
        );
    } else {
        let mid_intersect_point = triangle_vec4_midpoint(top_point, mid_point, bottom_point);
        let mid_intersect_uv = triangle_midpoint_uv(
            top_point,
            mid_point,
            bottom_point,
            top_point_texture,
            mid_point_texture,
            bottom_point_texture,
        );
        fill_flat_bottom_triangle_with_texture(
            top_point,
            mid_point,
            mid_intersect_point,
            top_point_texture,
            mid_point_texture,
            mid_intersect_uv,
            texture,
        );
        fill_flat_top_triangle_with_texture(
            bottom_point,
            mid_point,
            mid_intersect_point,
            bottom_point_texture,
            mid_point_texture,
            mid_intersect_uv,
            texture,
        );
    }
}

pub fn fill_flat_bottom_triangle_with_texture(
    p0: Vec4,
    p1: Vec4,
    p2: Vec4,
    texture_uv0: TextureUV,
    texture_uv1: TextureUV,
    texture_uv2: TextureUV,
    texture: &Texture,
) {
    let mut left_point = p1;
    let mut left_point_uv = texture_uv1;
    let mut right_point = p2;
    let mut right_point_uv = texture_uv2;
    if (left_point.x > right_point.x) {
        std::mem::swap(&mut left_point, &mut right_point);
        std::mem::swap(&mut left_point_uv, &mut right_point_uv);
    }

    let max_width = right_point.x.trunc() as i32 - left_point.x.trunc() as i32;

    let left_slope = get_inv_slope(p0.into(), left_point.into());
    let right_slope = get_inv_slope(p0.into(), right_point.into());

    let mut x_start = p0.x;
    let mut x_end = p0.x;

    let y_start = p0.y.trunc() as i32;
    let y_end = p2.y.trunc() as i32 + 1;
    // dbg!(p0.x, p0.y, p1.x, p1.y, p2.x, p2.y);
    // dbg!(y_start, y_end, left_slope, right_slope, x_start, x_end);
    for y in y_start..y_end {
        let xs = x_start as i32;
        let xe = x_end as i32;
        for x in xs..xe + 1 {
            render_texel(
                IntVec2 { x: x, y: y },
                p0,
                left_point,
                right_point,
                texture_uv0,
                left_point_uv,
                right_point_uv,
                texture,
            );
        }
        x_start += left_slope;
        x_end += right_slope;
        if (x_end - x_start) as i32 > max_width {
            x_start = left_point.x as f32;
            x_end = right_point.x as f32;
        }
    }
}

pub fn render_texel(
    p: IntVec2,
    p0: Vec4,
    p1: Vec4,
    p2: Vec4,
    uv0: TextureUV,
    uv1: TextureUV,
    uv2: TextureUV,
    texture: &Texture,
) {
    let weights = barycentric_weights(p0.into(), p1.into(), p2.into(), p);
    let alpha = weights.x;
    let beta = weights.y;
    let gamma = weights.z;

    let interpolated_u = uv0.u / p0.w * alpha + uv1.u / p1.w * beta + uv2.u / p2.w * gamma;
    let interpolated_v = uv0.v / p0.w * alpha + uv1.v / p1.w * beta + uv2.v / p2.w * gamma;
    let interpolated_reciprocal_w =
        (1.0 / p0.w) * alpha + (1.0 / p1.w) * beta + (1.0 / p2.w) * gamma;
    // dbg!(
    //     alpha,
    //     beta,
    //     gamma,
    //     uv0.u,
    //     uv1.u,
    //     uv2.u,
    //     interpolated_u,
    //     interpolated_v
    // );
    let texture_x_coord =
        ((interpolated_u / interpolated_reciprocal_w) * texture.width as f32).abs() as u32;
    let texture_y_coord =
        ((interpolated_v / interpolated_reciprocal_w) * texture.height as f32).abs() as u32;

    let mut index = (texture_y_coord * texture.width + texture_x_coord) as usize
        % (texture.width * texture.height) as usize;
    // if (index >= 4096) {
    // index = 4095;
    // }
    let w = 1.0 - interpolated_reciprocal_w;
    let pixel_index = (p.y as u32 * WIDTH + p.x as u32) as usize;
    if get_game_memory().z_buffer[pixel_index] > w {
        get_game_memory().z_buffer[pixel_index] = w;
        render_pixel(p.x, p.y, texture.data[index]);
    }
    // render_pixel(p.x, p.y, index as u32);
}

pub fn fill_flat_top_triangle_with_texture(
    p0: Vec4,
    p1: Vec4,
    p2: Vec4,
    texture_uv0: TextureUV,
    texture_uv1: TextureUV,
    texture_uv2: TextureUV,
    texture: &Texture,
) {
    let mut left_point = p1;
    let mut left_point_uv = texture_uv1;
    let mut right_point = p2;
    let mut right_point_uv = texture_uv2;
    if (left_point.x > right_point.x) {
        std::mem::swap(&mut left_point, &mut right_point);
        std::mem::swap(&mut left_point_uv, &mut right_point_uv);
    }

    let max_width = right_point.x.trunc() as i32 - left_point.x.trunc() as i32;

    let left_slope = get_inv_slope(left_point.into(), p0.into());
    let right_slope = get_inv_slope(right_point.into(), p0.into());

    let mut x_start = p0.x;
    let mut x_end = p0.x;

    let y_start = p0.y.trunc() as i32;
    let y_end = right_point.y.trunc() as i32;
    let mut y = y_start;
    while y >= y_end {
        let xs = x_start as i32;
        let xe = x_end as i32;
        for x in xs..xe + 1 {
            render_texel(
                IntVec2 { x: x, y: y },
                p0,
                left_point,
                right_point,
                texture_uv0,
                left_point_uv,
                right_point_uv,
                texture,
            );
        }
        x_start -= left_slope;
        x_end -= right_slope;
        if (x_end - x_start) as i32 > max_width {
            x_start = left_point.x as f32;
            x_end = right_point.x as f32;
        }
        y -= 1;
    }
}

pub fn render_pixel(x_pos: i32, y_pos: i32, color: u32) {
    let color_buffer = get_color_buffer();
    let width_i32 = WIDTH as i32;
    let height_i32 = HEIGHT as i32;
    if (x_pos < width_i32 && x_pos >= 0 && y_pos >= 0 && y_pos < height_i32) {
        color_buffer[(y_pos * width_i32 + x_pos) as usize] = color;
    }
}

pub fn clear_color_buffer(color: u32) {
    let color_buffer = get_color_buffer();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            color_buffer[(y * WIDTH + x) as usize] = color;
        }
    }
}

pub fn render_color_buffer(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    texture: &mut sdl2::render::Texture,
) -> Result<(), String> {
    let color_buffer = get_color_buffer();
    texture.update(
        None,
        unsafe {
            std::slice::from_raw_parts(color_buffer.as_ptr() as *const u8, color_buffer.len() * 4)
        },
        (WIDTH * 4) as usize,
    );
    canvas.copy(texture, None, None)?;
    canvas.present();
    Ok(())
}

pub fn make_grid(color_line: u32, color_back: u32, width: u32, height: u32) {
    let num_threads = thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(4)
        .min(HEIGHT as usize);

    let mid_x = (width / 2) as usize;
    let mid_y = (height / 2) as usize;

    let rows_per_thread = (HEIGHT as usize + num_threads - 1) / num_threads;

    thread::scope(|s| {
        for thread_id in 0..num_threads {
            let start_row = thread_id * rows_per_thread;
            let end_row = std::cmp::min((thread_id + 1) * rows_per_thread, HEIGHT as usize);

            s.spawn(move || unsafe {
                let buffer = get_color_buffer();

                for y in start_row..end_row {
                    let row_start = y * WIDTH as usize;
                    let is_horizontal_line = y == mid_y;

                    for x in 0..WIDTH as usize {
                        if is_horizontal_line || x == mid_x || (x % 10 == 0 && y % 10 == 0) {
                            buffer[row_start + x] = color_line;
                        } else {
                            buffer[row_start + x] = color_back;
                        }
                    }
                }
            });
        }
    });
}

pub fn render(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    texture: &mut sdl2::render::Texture,
    color_buffer: &mut [u32],
    width: u32,
    height: u32,
) -> Result<(), String> {
    let memory = get_game_memory();

    // canvas.set_draw_color(Color::RGB(0, 0, 0));
    // canvas.clear();
    // clear_color_buffer(0xFFFF0000);
    make_grid(0xFF505966, 0xFF292B2E, width, height);
    render_entity(&memory.entity, &mut memory.camera);
    // render_box(100, 100, 200, 200, 0xFFEDAB74);
    // render_box(-100, -100, 200, 200, 0xFFEDAB74);
    // render_pixel((width / 2) as i32, (height / 2) as i32, 0xFFED0010);
    render_color_buffer(canvas, texture);
    for i in 0..memory.z_buffer.len() {
        memory.z_buffer[i] = 1.0;
    }
    Ok(())
}
