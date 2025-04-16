use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::game_state::get_game_memory;
use crate::math::{vector3_add, vector3_mul_float, vector3_sub};
use crate::obj_importer::{
    import_entity_from_obj, import_texture, open_model_path, open_texture_path,
};

pub fn process_input(event_pump: &mut sdl2::EventPump, is_loop_running: &mut bool) {
    let memory = get_game_memory();
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
            } => get_game_memory().rotation_objects_type = 0,
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => get_game_memory().rotation_objects_type = 1,
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => get_game_memory().rotation_objects_type = 2,
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                let velocity = vector3_mul_float(memory.camera.direction, 0.09);

                get_game_memory().camera.position = vector3_add(memory.camera.position, velocity);
            }
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                let velocity = vector3_mul_float(memory.camera.direction, 0.09);

                get_game_memory().camera.position = vector3_sub(memory.camera.position, velocity);
            }
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => get_game_memory().camera.velocity.x -= 0.05,
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => get_game_memory().camera.velocity.x += 0.05,
            Event::KeyDown {
                keycode: Some(Keycode::F),
                ..
            } => get_game_memory().camera.position.y += 0.05,
            Event::KeyDown {
                keycode: Some(Keycode::G),
                ..
            } => get_game_memory().camera.position.y -= 0.05,
            Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => get_game_memory().camera.rotation.y -= 0.01,
            Event::KeyDown {
                keycode: Some(Keycode::E),
                ..
            } => get_game_memory().camera.rotation.y += 0.01,
            Event::KeyDown {
                keycode: Some(Keycode::B),
                ..
            } => get_game_memory().speed = 0.0,
            Event::KeyDown {
                keycode: Some(Keycode::N),
                ..
            } => get_game_memory().speed += 0.00001,
            Event::KeyDown {
                keycode: Some(Keycode::M),
                ..
            } => get_game_memory().speed -= 0.00001,
            Event::KeyDown {
                keycode: Some(Keycode::Z),
                ..
            } => memory.stop = !memory.stop,
            Event::KeyDown {
                keycode: Some(Keycode::Num1),
                ..
            } => memory.render_settings.fill_triangles = !memory.render_settings.fill_triangles,
            Event::KeyDown {
                keycode: Some(Keycode::Num2),
                ..
            } => memory.render_settings.draw_vert = !memory.render_settings.draw_vert,
            Event::KeyDown {
                keycode: Some(Keycode::Num3),
                ..
            } => memory.render_settings.draw_edges = !memory.render_settings.draw_edges,
            Event::KeyDown {
                keycode: Some(Keycode::Num4),
                ..
            } => memory.render_settings.show_normals = !memory.render_settings.show_normals,
            Event::KeyDown {
                keycode: Some(Keycode::Num5),
                ..
            } => memory.render_settings.use_textures = !memory.render_settings.use_textures,
            Event::KeyDown {
                keycode: Some(Keycode::Num6),
                ..
            } => memory.render_settings.use_lighting = !memory.render_settings.use_lighting,
            Event::MouseMotion {
                x, xrel, y, yrel, ..
            } => {
                memory.camera.rotation.x += (yrel as f32) * 0.001;
                memory.camera.rotation.y += (xrel as f32) * 0.001;
            }
            Event::KeyDown {
                keycode: Some(Keycode::O),
                ..
            } => {
                if let Some(path) = open_model_path() {
                    // get_game_memory().entity = import_entity_from_obj("D:\\Coding\\Projects\\graphics_3d_from_scratch_pikuma\\assets\\f22.obj",)
                    get_game_memory().entity = import_entity_from_obj(path.to_str().unwrap_or(""));
                }
                if let Some(path) = open_texture_path() {
                    get_game_memory().texture = import_texture(path.to_str().unwrap_or(""));
                }
            }
            _ => {}
        }
    }
}
