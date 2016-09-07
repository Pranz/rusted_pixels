
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
    pub images: Vec<ImageBuffer>,
    pub palettes: Vec<Color>,//TODO: Multiple palettes
}

impl State {
    pub fn new() -> Self {
        State {
            current_color: Color::RGB(255,255,255),
            left_mouse_down: false,
            right_mouse_down: false,
            mouse_x: 0,
            mouse_y: 0,
            images: vec![],
            palettes: vec![
                Color::RGB(0,0,0),
                Color::RGB(128,128,128),
                Color::RGB(255,255,255),
                Color::RGB(192,128,112),
            ],
        }
    }
}
