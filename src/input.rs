use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::game_state::get_game_memory;
use crate::obj_importer::{import_entity_from_obj, open_model_path};

pub fn process_input(event_pump: &mut sdl2::EventPump, is_loop_running: &mut bool) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => *is_loop_running = false,
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => *is_loop_running = false,
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => get_game_memory().rotation = 0,
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => get_game_memory().rotation = 1,
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => get_game_memory().rotation = 2,
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => get_game_memory().camera_position.y += 0.5,
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => get_game_memory().camera_position.y -= 0.5,
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => get_game_memory().camera_position.x -= 0.5,
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => get_game_memory().camera_position.x += 0.5,
            Event::KeyDown {
                keycode: Some(Keycode::F),
                ..
            } => get_game_memory().camera_position.z += 0.5,
            Event::KeyDown {
                keycode: Some(Keycode::G),
                ..
            } => get_game_memory().camera_position.z -= 0.5,
            Event::KeyDown {
                keycode: Some(Keycode::B),
                ..
            } => get_game_memory().speed = 0.0,
            Event::KeyDown {
                keycode: Some(Keycode::N),
                ..
            } => get_game_memory().speed += 0.0001,
            Event::KeyDown {
                keycode: Some(Keycode::M),
                ..
            } => get_game_memory().speed -= 0.0001,
            Event::KeyDown {
                keycode: Some(Keycode::Z),
                ..
            } => get_game_memory().stop = !get_game_memory().stop,
            Event::KeyDown {
                keycode: Some(Keycode::Num1),
                ..
            } => get_game_memory().fill_triangles = !get_game_memory().fill_triangles,
            Event::KeyDown {
                keycode: Some(Keycode::Num2),
                ..
            } => get_game_memory().draw_vert = !get_game_memory().draw_vert,
            Event::KeyDown {
                keycode: Some(Keycode::Num3),
                ..
            } => get_game_memory().draw_edges = !get_game_memory().draw_edges,
            Event::KeyDown {
                keycode: Some(Keycode::Num4),
                ..
            } => get_game_memory().show_normals = !get_game_memory().show_normals,
            Event::KeyDown {
                keycode: Some(Keycode::O),
                ..
            } => {
                if let Some(path) = open_model_path() {
                    // get_game_memory().entity = import_entity_from_obj("D:\\Coding\\Projects\\graphics_3d_from_scratch_pikuma\\assets\\f22.obj",)
                    get_game_memory().entity = import_entity_from_obj(path.to_str().unwrap_or(""));
                }
            }
            _ => {}
        }
    }
}
