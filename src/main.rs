mod game_state;
mod input;
mod math;
mod matrix;
mod obj_importer;
mod render;
mod texture;
mod types;
mod vector;

use std::thread;
use std::time::Duration;

use game_state::{get_game_memory, init_game_memory};
use input::process_input;
use math::rotate_entity;
use render::render;

fn main() -> Result<(), String> {
    init_game_memory();
    let sdl_context = sdl2::init()?;
    let width = 1270;
    let height = 720;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("3D from scratch pukima course", width, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let mut is_loop_running = true;
    let mut texture = texture_creator
        .create_texture_streaming(sdl2::pixels::PixelFormatEnum::ARGB8888, width, height)
        .map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut color_buffer = vec![0u32; (width * height) as usize];

    let ten_millis = Duration::from_millis(10);

    const TARGET_FPS: f64 = 90.0;
    const FRAME_TIME: f64 = 1000.0 / TARGET_FPS;
    let timer_subsystem = sdl_context.timer()?;
    let performance_frequency = timer_subsystem.performance_frequency() as f64;
    let mut frame_count = 0;
    let mut fps_timer = timer_subsystem.performance_counter();
    let mut previous_frame_time = timer_subsystem.performance_counter();
    while (is_loop_running) {
        let frame_start = timer_subsystem.performance_counter();

        get_game_memory().delta_time =
            ((frame_start - previous_frame_time) as f64 * 1000.0 / performance_frequency) as f32;
        previous_frame_time = frame_start;
        process_input(&mut event_pump, &mut is_loop_running);
        if !get_game_memory().stop {
            update();
            render(&mut canvas, &mut texture, &mut color_buffer, width, height);
        }

        let frame_end = timer_subsystem.performance_counter();
        let elapsed_ms = ((frame_end - frame_start) as f64 * 1000.0) / performance_frequency;

        if elapsed_ms < FRAME_TIME {
            let delay_time = (FRAME_TIME - elapsed_ms) as u32;
            timer_subsystem.delay(delay_time);
        }
        frame_count += 1;
        let current_time = timer_subsystem.performance_counter();
        let time_elapsed = (current_time - fps_timer) as f64 / performance_frequency;
        if time_elapsed >= 1.0 {
            let fps = frame_count as f64 / time_elapsed;
            println!("FPS: {:.2}", fps);
            frame_count = 0;
            fps_timer = current_time;
        }
    }
    Ok(())
}
pub fn update() {
    // ortographic_project_entity();
    rotate_entity();
    // perspective_project_entity();
}
