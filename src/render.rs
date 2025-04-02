use crate::types::Vec2;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::video::Window;
use std::cmp;
use std::thread;

use crate::game_state::{get_color_buffer, get_game_memory, BOX_POINT_COUNTER, HEIGHT, WIDTH};

pub fn render_entity() {
    let memory = get_game_memory();
    for i in 0..BOX_POINT_COUNTER {
        let point = memory.projected_points[i];
        render_box(point.x as i32, point.y as i32, 4, 4, 0xFFFF0000);
    }
}

pub fn render_box(x_pos: i32, y_pos: i32, box_width: u32, box_height: u32, color: u32) {
    let width_i32 = WIDTH as i32;
    let height_i32 = HEIGHT as i32;
    let box_width_i32 = box_width as i32;
    let box_height_i32 = box_height as i32;
    let x_begin = cmp::max(0, x_pos);
    let y_begin = cmp::max(0, y_pos);

    let x_end = cmp::min(width_i32, x_pos + box_width_i32);
    let y_end = cmp::min(height_i32, y_pos + box_height_i32);
    for y in y_begin..y_end {
        for x in x_begin..x_end {
            render_pixel(x, y, color);
        }
    }
}

pub fn render_pixel(x_pos: i32, y_pos: i32, color: u32) {
    let color_buffer = get_color_buffer();
    let width_i32 = WIDTH as i32;
    let height_i32 = HEIGHT as i32;
    if (x_pos <= width_i32 && x_pos >= 0 && y_pos >= 0 && y_pos <= height_i32) {
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

pub fn make_grid(color_line: u32, color_back: u32) {
    let num_threads = thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(4)
        .min(HEIGHT as usize);

    let rows_per_thread = (HEIGHT as usize + num_threads - 1) / num_threads;

    thread::scope(|s| {
        for thread_id in 0..num_threads {
            let start_row = thread_id * rows_per_thread;
            let end_row = std::cmp::min((thread_id + 1) * rows_per_thread, HEIGHT as usize);

            s.spawn(move || unsafe {
                let buffer = get_color_buffer();

                for y in start_row..end_row {
                    let row_start = y * WIDTH as usize;
                    let is_horizontal_line = y % 10 == 0;

                    if is_horizontal_line {
                        for x in 0..WIDTH as usize {
                            buffer[row_start + x] = color_line;
                        }
                    } else {
                        for x in 0..WIDTH as usize {
                            buffer[row_start + x] =
                                if x % 10 == 0 { color_line } else { color_back };
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
    // canvas.set_draw_color(Color::RGB(0, 0, 0));
    // canvas.clear();
    // clear_color_buffer(0xFFFF0000);
    make_grid(0xFF505966, 0xFF292B2E);
    render_entity();
    // render_box(100, 100, 200, 200, 0xFFEDAB74);
    // render_box(-100, -100, 200, 200, 0xFFEDAB74);
    // render_pixel((width / 2) as i32, (height / 2) as i32, 0xFFED0010);
    render_color_buffer(canvas, texture);
    Ok(())
}
