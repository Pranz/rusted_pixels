
use sdl2::pixels::Color;
use image_buffer::ImageBuffer;

/*
 * Holds the main state, pretty self explanatory.
 */
pub struct State {
    pub current_color: Color,
    pub left_mouse_down: bool,
    pub right_mouse_down: bool,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub images: Vec<ImageBuffer>
}

impl State {
    pub fn new() -> State {
        State {
            current_color: Color::RGB(255,255,255),
            left_mouse_down: false,
            right_mouse_down: false,
            mouse_x: 0,
            mouse_y: 0,
            images: vec![ImageBuffer::new(32,64)],
        }
    }
}
