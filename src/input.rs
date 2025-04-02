use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::game_state::get_game_memory;

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
            _ => {}
        }
    }
}
