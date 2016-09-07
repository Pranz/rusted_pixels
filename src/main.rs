#![feature(slice_patterns)]

extern crate sdl2;
extern crate png;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::BlendMode;
use sdl2::mouse::Mouse;
use std::{fs,io,path};

pub mod image_buffer;
pub mod windows;
pub mod state;

use image_buffer::ImageBuffer;
use state::State;
use windows::{DrawingWindow, Window, PreviewWindow};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rusted pixels", 800, 600)
        .resizable()
        .build()
        .unwrap();

    let mut renderer = window.renderer().present_vsync().build().unwrap();

    // this is the most intuitive blend mode.
    renderer.set_blend_mode(BlendMode::Blend);

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut state = State{images: vec![load_image(&path::PathBuf::from("test.png")).unwrap(),ImageBuffer::new(32,64)], ..State::new()};

    let mut windows: Vec<Box<Window>> =
        vec![Box::new(DrawingWindow::new(50, 50, 8,
                                         Color::RGB(100, 100, 100), 0)),
             Box::new(PreviewWindow(
                 DrawingWindow::new(400, 50, 1,
                                    Color::RGB(50,50,50), 0))),
             Box::new(DrawingWindow::new(400, 400, 2,
                                         Color::RGB(50,50,50), 0))];

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
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
                }
                Event::MouseButtonUp { mouse_btn: Mouse::Left, .. } => {
                    state.left_mouse_down = false;
                }
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

fn load_image(path: &path::PathBuf) -> io::Result<ImageBuffer> {
    use png::ColorType::*;

    let decoder = png::Decoder::new(try!(fs::File::open(path)));
    let (info,mut reader) = try!(decoder.read_info());
    let mut img_data = vec![0; info.buffer_size()];
    try!(reader.next_frame(&mut img_data));

    Ok(ImageBuffer{
        width: info.width as usize,
        height: info.height as usize,
        buffer: match info.color_type {
            RGB => {
                img_data.chunks(3).map(|color_data|{
                    if let &[r,g,b] = color_data{
                        Color::RGB(r,g,b)
                    }else{
                        panic!("but it said that it was gonna be rgb..")
                    }
                }).collect()
            },
            RGBA => {
                img_data.chunks(4).map(|color_data|{
                    if let &[r,g,b,a] = color_data{
                        Color::RGBA(r,g,b,a)
                    }else{
                        panic!("but it said that it was gonna be rgb..")
                    }
                }).collect()
            },
            _ => unreachable!("uncovered color type")
        }
    })
}
