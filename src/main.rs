extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::BlendMode;
use sdl2::mouse::Mouse;
use sdl2::rect::*;

pub mod image_buffer;
pub mod windows;

use windows::DrawingWindow;
use image_buffer::ImageBuffer;

pub fn handle_mouse_down(window: &DrawingWindow,
                         image: &mut ImageBuffer,
                         color: Color,
                         mouse_x: i32,
                         mouse_y : i32) {
    
    let coordinates = window.get_index(&image, mouse_x, mouse_y);
    if let Some((x,y)) = coordinates {
        *image.get_mut_ref(x as usize,y as usize) = color; 
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Window", 800, 600)
        .resizable()
        .build()
        .unwrap();

    let mut renderer = window.renderer().present_vsync().build().unwrap();
    renderer.set_blend_mode(BlendMode::Blend);
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut image = ImageBuffer::new(32,64);
    let mut current_color = Color::RGB(255,255,255);
    let mut left_mouse_down = false;

    *image.get_mut_ref(0, 0) = Color::RGB(255, 0, 0);
    *image.get_mut_ref(1, 0) = Color::RGB(255, 255, 255);
    *image.get_mut_ref(5, 5) = Color::RGB(255, 255, 255);

    let mut draw_window = DrawingWindow::new(50, 50, 8,
                                             Color::RGB(100, 100, 100));
    let preview_window = DrawingWindow::new(400, 50, 1,
                                            Color::RGB(0,0,0));

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main_loop
                },
                Event::MouseButtonDown { mouse_btn: Mouse::Left,
                                         x: x, y: y, .. } => {
                    left_mouse_down = true;
                    handle_mouse_down(&draw_window, &mut image,
                                      current_color, x, y);
                },
                Event::MouseMotion { x: x, y: y, .. } => {
                    if left_mouse_down {
                        handle_mouse_down(&draw_window, &mut image,
                                          current_color, x, y);
                    }
                }
                Event::MouseButtonUp { mouse_btn: Mouse::Left, .. } => {
                    left_mouse_down = false;
                }
                _ => {}
            }
        }

        {
            let mut window = renderer.window_mut().unwrap();
            
            
        }

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.set_draw_color(Color::RGB(255,255,255));

        draw_window.draw(&mut renderer, &image);
        preview_window.draw(&mut renderer, &image);
        
        renderer.present();
    }
}
