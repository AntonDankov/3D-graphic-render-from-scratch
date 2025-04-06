use crate::types::{Entity, IntVec2, Vec2, Vec3};
use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::video::Window;
use std::cmp;
use std::num;
use std::thread;

use crate::game_state::{get_color_buffer, get_game_memory, BOX_POINT_COUNTER, HEIGHT, WIDTH};
use crate::math::{
    get_inv_slope, perspective_project_point, transform_vertex, triangle_avg, triangle_midpoint,
    vector3_add, vector3_cross, vector3_dot, vector3_mul, vector3_normalize, vector3_sub,
};

pub fn render_entity(entity: &Entity, camera: Vec3) {
    let line_color = 0xFF00FF00;
    let game_memory = get_game_memory();
    // dbg!(
    //     game_memory.entity.rotation.x,
    //     game_memory.entity.rotation.y,
    //     game_memory.entity.rotation.z
    // );
    for (i, index) in entity.mesh.indexes.iter().enumerate() {
        // dbg!(i);
        let x_index = (index.x as usize) - 1;
        let y_index = (index.y as usize) - 1;
        let z_index = (index.z as usize) - 1;
        let p1 = transform_vertex(entity.mesh.vertices[x_index], entity.rotation);
        let p2 = transform_vertex(entity.mesh.vertices[y_index], entity.rotation);
        let p3 = transform_vertex(entity.mesh.vertices[z_index], entity.rotation);
        let vector_ab = vector3_sub(p2, p1);
        let vector_ac = vector3_sub(p3, p1);
        let mut normal = vector3_cross(vector_ab, vector_ac);
        vector3_normalize(&mut normal);

        let camera_ray = vector3_sub(camera, p1);
        let dot = vector3_dot(normal, camera_ray);

        if (dot < 0.0) {
            continue;
        }
        let normal_avg = triangle_avg(p1, p2, p3);

        let projected1 = perspective_project_point(p1, camera);
        let projected2 = perspective_project_point(p2, camera);
        let projected3 = perspective_project_point(p3, camera);

        // dbg!(p1.x, p1.y, p1.z, projected1.x, projected1.y);
        // dbg!(p2.x, p2.y, p2.z, projected2.x, projected2.y);
        // dbg!(p3.x, p3.y, p3.z, projected3.x, projected3.y);

        if get_game_memory().fill_triangles {
            fill_triangle(
                projected1.into(),
                projected2.into(),
                projected3.into(),
                0xFF184787,
            );
        }

        if game_memory.draw_edges {
            render_edges(projected1, projected2, projected3, line_color);
        }

        if game_memory.draw_vert {
            render_verticies(projected1, projected2, projected3);
        }

        if game_memory.show_normals {
            render_normals(normal, normal_avg, camera);
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

pub fn render_normals(normal: Vec3, normal_avg: Vec3, camera: Vec3) {
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
    let projected_normal_start = perspective_project_point(normal_avg, camera);
    let projected_normal_end = perspective_project_point(normal_end, camera);

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
    if x_start_cheched < 0 {
        x_start_cheched = 0;
    }
    if x_start_cheched > width_i32 {
        x_start_cheched = width_i32;
    }

    let mut x_end_cheched = x_end;
    if x_end_cheched < 0 {
        x_end_cheched = 0;
    }
    if x_end_cheched > width_i32 {
        x_end_cheched = width_i32;
    }

    let mut y_start_cheched = y_start;
    if y_start_cheched < 0 {
        y_start_cheched = 0;
    }
    if y_start_cheched > height_i32 {
        y_start_cheched = height_i32;
    }

    let mut y_end_cheched = y_end;
    if y_end_cheched < 0 {
        y_end_cheched = 0;
    }
    if y_end_cheched > height_i32 {
        y_end_cheched = height_i32;
    }

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
        let mid_intersect_point = triangle_midpoint(top_point, mid_point, bottom_point);
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
    render_entity(&memory.entity, memory.camera_position);
    // render_box(100, 100, 200, 200, 0xFFEDAB74);
    // render_box(-100, -100, 200, 200, 0xFFEDAB74);
    // render_pixel((width / 2) as i32, (height / 2) as i32, 0xFFED0010);
    render_color_buffer(canvas, texture);
    Ok(())
}
