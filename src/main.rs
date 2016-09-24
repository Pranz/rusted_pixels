extern crate sdl2;
extern crate sdl2_ttf;
extern crate png;
#[macro_use]extern crate bitflags;


use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::BlendMode;
use sdl2::mouse::Mouse;
use sdl2::video::Window as SdlWindow;
use sdl2::Sdl;
use std::path;

pub mod image_buffer;
pub mod windows;
pub mod state;
pub mod input;
pub mod util;

use input::*;

use image_buffer::ImageBuffer;
use state::State;
use windows::*;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let window = init_sdl_window(&sdl_context);
    
    let mut renderer = window.renderer().present_vsync().build().unwrap();
    renderer.set_blend_mode(BlendMode::Blend);

    let commands = input::get_commands();
    let windows: Vec<Box<Window>> = initialize_windows();
    let mut state = State{images: vec![
        ImageBuffer::load_png_image(&path::PathBuf::from("test.png")).unwrap(),
        ImageBuffer::new(32,64)
    ], ..State::new()};
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'main_loop
                },
                Event::MouseButtonDown { mouse_btn: Mouse::Left,
                                         x, y, .. } => {
                    state.left_mouse_down = true;
                    for window in &windows {
                        window
                            .handle_mouse_down(&mut state, x, y);
                    }
                },
                Event::MouseMotion { x, y, .. } => {
                    state.mouse_x = x;
                    state.mouse_y = y;
                    if state.left_mouse_down {
                        for window in &windows {
                            window
                                .handle_mouse_down(&mut state, x, y);
                        }
                    }
                },
                Event::MouseButtonUp { mouse_btn: Mouse::Left, .. } => {
                    state.left_mouse_down = false;
                },
                Event::KeyDown { keycode: Some(keycode), keymod, .. } => {
                    // every command begins with a single key
                    if state.input.is_empty() {
                        state.input.push(Input::Char(keycode,keymod));
                        match execute_command(&mut state, &commands) {
                            CommandResult::Quit => { break 'main_loop },
                            _ => {}
                        }
                    }
                    // If escape is pressed, clear input buffer or pop
                    // input stack
                    else if keycode == Keycode::Escape {
                        if !state.input_buffer.is_empty() {
                            state.input_buffer = String::new();
                        } else {
                            state.input.pop();
                        }
                    }
                    else if keycode == Keycode::Return {
                        let (input_type, arg)
                            = input::parse_input(&state.input_buffer);
                        state.input.push(input_type);
                        if let Some(arg) = arg {
                            state.args.push(arg);
                        }
                        state.input_buffer = String::new();
                        match execute_command(&mut state, &commands) {
                            CommandResult::Quit => { break 'main_loop },
                            _ => {}
                        }
                    }
                    else {
                        if let Some(chr) = input::keycode_to_char(keycode) {
                            state.input_buffer.push(chr);
                            println!("{:?}", state.input_buffer.as_str());
                        }
                    }
                },
                _ => {}
            }
        }

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.set_draw_color(Color::RGB(255,255,255));

        for window in &windows {
            window.draw(&mut renderer, &state);
        }

        renderer.present();
    }
}

pub fn initialize_windows() -> Vec<Box<Window>> {
    let gray = Color::RGB(50, 50, 50);
    let lighter_gray = Color::RGB(100, 100, 100);
    vec![Box::new(DrawingWindow::new(50, 50, 8, lighter_gray, 0)),
         Box::new(PreviewWindow(DrawingWindow::new(400, 50, 1, gray, 0))),
         Box::new(DrawingWindow::new(400, 400, 2, gray, 0)),
         Box::new(PaletteWindow{x: 400,y: 100,palette_id: 0})]
}

pub fn init_sdl_window(sdl_context: &Sdl) -> SdlWindow {
    let video_subsystem = sdl_context.video().unwrap();
    video_subsystem.window("rusted pixels", 800, 600)
        .resizable()
        .build()
        .unwrap()
}
